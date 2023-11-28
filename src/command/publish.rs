use std::path::Path;

use anyhow::{bail, Context, Result};
use dialoguer::Select;
use semver::Version;

use crate::{
    api,
    descriptor::{Dataset, TableSchemaObjectPrimaryKey},
    session,
};

pub async fn publish(descriptor_path: &Path) -> Result<()> {
    let package = Dataset::read(descriptor_path)
        .with_context(|| format!("Failed to read descriptor at {}", descriptor_path.display()))?;

    let mut tables_missing_pk: Vec<&str> = package
        .tables
        .iter()
        .filter(|&table| match &table.schema.as_ref().unwrap().primary_key {
            Some(TableSchemaObjectPrimaryKey::Variant0(names)) => names.is_empty(),
            Some(TableSchemaObjectPrimaryKey::Variant1(name)) => name.is_empty(),
            None => true,
        })
        .map(|t| t.name.as_str())
        .collect();
    tables_missing_pk.sort();

    if !tables_missing_pk.is_empty() {
        let name_list = tables_missing_pk.join(", ");
        bail!(
            "Cannot publish package when some tables do not have a primary key defined: {}",
            name_list
        )
    }

    let token = session::get_token()?;
    let client = api::Client::new(&token)?;

    // Note: The `find` below depends on `client.get_package_versions` returning versions in
    // reverse version order.
    let response = client.get_dataset_versions(&package.id.to_string()).await?;
    let latest_release_version = response.as_ref().and_then(|response| {
        response
            .dataset_versions
            .iter()
            .find(|package_version| package_version.version.pre.is_empty())
    });

    let resolved_accelerated = if let Some(latest_version) = latest_release_version {
        match (latest_version.accelerated, package.accelerated) {
            (_, true) => true,
            (false, false) => false,
            (true, false) => {
                match verify_intended_acceleration(&latest_version.version, &package.version)? {
                    Some(intention) => intention,
                    None => {
                        eprintln!("Publish cancelled");
                        return Ok(());
                    }
                }
            }
        }
    } else {
        package.accelerated
    };

    client
        .create_version(
            package.id,
            &package.version,
            &api::CreateDatasetVersion {
                name: &package.name,
                draft: false,
                accelerated: resolved_accelerated,
                description: &package.description.unwrap_or("".into()),
                tables: &package.tables,
            },
        )
        .await?;

    eprintln!(
        "Published package {} version {}",
        package.name, package.version
    );
    if package.accelerated {
        eprintln!(
            "This version is ✨ accelerated. ✨ What this means:
- Patch is performing the intial data acceleration now.
- Building release packages (`dpm build-package -p <REF>`) is not
  supported until initial acceleration is complete. To check
  the status of the acceleration, run `dpm dataset list`."
        )
    }

    Ok(())
}

/// Prompts user to confirm their intent around "reverting" the acceleration of
/// a package. Returns `Some` intended value for `accelerated`, or `None` if
/// publication should abort.
fn verify_intended_acceleration(
    latest_version: &Version,
    next_version: &Version,
) -> Result<Option<bool>> {
    eprintln!(
        "WARNING: Latest release version ({}) is accelerated, but the version about to be published ({}) is not.\n",
        latest_version,
        next_version,
    );

    #[derive(PartialEq)]
    enum Choice {
        Accelerated,
        Direct,
    }
    impl ToString for Choice {
        fn to_string(&self) -> String {
            match self {
                Choice::Accelerated => "Ignore descriptor, publish accelerated".into(),
                Choice::Direct => "Respect descriptor, publish unaccelerated".into(),
            }
        }
    }

    let choices = &[Choice::Accelerated, Choice::Direct];
    let choice = Select::new()
        .with_prompt("How do you want to proceed?")
        .items(choices)
        .default(0)
        .interact_opt()?;

    Ok(choice.map(|i| choices[i] == Choice::Accelerated))
}
