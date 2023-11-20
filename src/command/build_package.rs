use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use semver::Version;

use crate::{
    api::{Client, CreateDatasetVersion, GetDatasetVersionResponse, PatchState},
    codegen::{generate_package, Target},
    descriptor::Dataset,
    session,
};

pub async fn build(
    descriptor: PathBuf,
    dataset_ref: Option<String>,
    target: Target,
    out_dir: PathBuf,
    assume_yes: bool,
) -> Result<()> {
    let session = session::get_token().expect("unable to get session");
    let client = Client::new(&session).expect("unable to get client");

    // `descriptor` is always defined (possibly via its default_value), whereas
    // the caller may instead opt to build a published dataset via --dataset.
    // Before reaching this function, clap will have verified that if --dataset
    // was given, --descriptor was not given.
    let build_input: GetDatasetVersionResponse = if let Some(dataset_ref) = dataset_ref {
        let dataset_identifier: Vec<&str> = dataset_ref.split('@').collect();
        if dataset_identifier.len() != 2 {
            bail!("invalid --dataset value; expected \"<dataset name>@<version>\"")
        }
        let version: Version =
            Version::parse(dataset_identifier[1]).expect("dataset identifier `version` is invalid");

        match client
            .get_dataset_version(dataset_identifier[0], version)
            .await?
        {
            Some(response) => response,
            None => bail!("dataset or dataset version not found: \"{}\"", dataset_ref),
        }
    } else {
        let dataset = Dataset::read(&descriptor)
            .with_context(|| format!("failed to read {}", descriptor.display()))?;

        eprintln!(
            "creating draft version of {}@{}",
            dataset.name, dataset.version
        );

        let created_version = client
            .create_version(
                dataset.id,
                &dataset.version,
                &CreateDatasetVersion {
                    name: &dataset.name,
                    draft: true,
                    accelerated: false,
                    description: &dataset.description.clone().unwrap_or("".into()),
                    tables: &dataset.tables,
                },
            )
            .await?;

        eprintln!(
            "draft version created: {}@{}",
            dataset.name, created_version.version
        );
        eprintln!("tip: Your drafts are queryable only by you. To enable access by others, create a release version with `dpm publish`.");

        GetDatasetVersionResponse {
            name: dataset.name.to_string(),
            uuid: uuid::Uuid::parse_str(&dataset.id.to_string()).unwrap(),
            description: dataset.description.unwrap_or("".into()),
            version: created_version,
        }
    };

    if build_input.version.accelerated {
        match build_input.version.patch_state.as_ref() {
            Some(PatchState::SyncingInitial) => {
                let message = "The dataset you requested is acceleration-enabled but has not yet completed its initial sync.
Because it would be potentially confusing for an instance of an \"accelerated\" dataset
version to execute its queries without acceleration, the build will abort now.
Please try to build it again once its initial sync has completed.

tip: To check the state of the version, use `dpm dataset list`.";
                bail!(message)
            }
            Some(PatchState::ErrorSyncingInitial) => {
                let fallback_message =
                    String::from("An unknown error occurred. Please report this issue!");
                let error_message = format!(
                    "Error: {}",
                    build_input
                        .version
                        .patch_state_data
                        .as_ref()
                        .map_or(fallback_message.to_owned(), |data| {
                            serde_json::to_string_pretty(data).unwrap_or(fallback_message)
                        })
                );

                let message = format!("The dataset you requested to build failed to complete its initial acceleration.

{}

Because it would be potentially confusing for an instance of an \"accelerated\" dataset
version to execute its queries without acceleration, the build will abort now.
Resolve the error above, then try building again.", error_message);

                bail!(message)
            }
            Some(PatchState::Syncing) | Some(PatchState::ErrorSyncing) => (
                // Initial sync has completed => allow build to proceed
            ),
            None => bail!("An invalid state has occurred. Please report this issue!"),
        }
    }

    create_dir_all(&out_dir).expect("error creating output directory");
    check_output_dir(&out_dir);
    generate_package(&build_input, &target, &out_dir, assume_yes);

    Ok(())
}

/// Checks that the output directory exists and is accessible.
fn check_output_dir(p: &Path) {
    match p.try_exists() {
        Ok(v) if !v => panic!("Output directory {:?} does not exist", p),
        Err(e) => {
            panic!("Error accessing output directory {e}")
        }
        _ => {}
    }
}
