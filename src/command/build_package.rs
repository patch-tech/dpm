use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use semver::Version;

use crate::{
    api::{Client, CreatePackageVersion, GetPackageVersionResponse, PatchState},
    codegen::{generate_package, Target},
    descriptor::Dataset,
    session,
};

pub async fn build(
    descriptor: PathBuf,
    package: Option<String>,
    target: Target,
    out_dir: PathBuf,
    assume_yes: bool,
) -> Result<()> {
    let session = session::get_token().expect("unable to get session");
    let client = Client::new(&session).expect("unable to get client");

    // `descriptor` is always defined (possibly via its
    // default_value), whereas the caller may instead opt to build a
    // published package via -p. Before reaching this function, clap
    // will have verified that if -p was given, -d was not given.
    let build_input: GetPackageVersionResponse = if let Some(package_ref) = package {
        let package_identifier: Vec<&str> = package_ref.split('@').collect();
        if package_identifier.len() != 2 {
            bail!("invalid -p value; expected \"<package name>@<version>\"")
        }
        let version: Version =
            Version::parse(package_identifier[1]).expect("package identifier `version` is invalid");

        match client
            .get_package_version(package_identifier[0], version)
            .await?
        {
            Some(response) => response,
            None => bail!("package or package version not found: \"{}\"", package_ref),
        }
    } else {
        let dp = Dataset::read(&descriptor)
            .with_context(|| format!("failed to read {}", descriptor.display()))?;

        eprintln!("creating draft version of {}@{}", dp.name, dp.version);

        let created_version = client
            .create_version(
                dp.id,
                &dp.version,
                &CreatePackageVersion {
                    name: &dp.name,
                    draft: true,
                    accelerated: false,
                    description: &dp.description.clone().unwrap_or("".into()),
                    dataset: &dp.tables,
                },
            )
            .await?;

        eprintln!(
            "draft version created: {}@{}",
            dp.name, created_version.version
        );
        eprintln!("tip: Your drafts are queryable only by you. To enable access by others, create a release version with `dpm publish`.");

        GetPackageVersionResponse {
            package_name: dp.name.to_string(),
            package_uuid: uuid::Uuid::parse_str(&dp.id.to_string()).unwrap(),
            package_description: dp.description.unwrap_or("".into()),
            version: created_version,
        }
    };

    if build_input.version.accelerated {
        match build_input.version.patch_state.as_ref() {
            Some(PatchState::SyncingInitial) => {
                let message = "The package you requested is acceleration-enabled but has not yet completed its initial sync.
Because it would be potentially confusing for an instance of an \"accelerated\" package
version to execute its queries without acceleration, the build will abort now.
Please try to build it again once its initial sync has completed.

tip: To check the state of the version, use `dpm package list`.";
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

                let message = format!("The package you requested to build failed to complete its initial acceleration.

{}

Because it would be potentially confusing for an instance of an \"accelerated\" package
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
