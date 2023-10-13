use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::{
    api,
    descriptor::{DataPackage, TableSchema, TableSchemaObjectPrimaryKey},
    session,
};

pub async fn publish(descriptor_path: &Path) -> Result<()> {
    let package = DataPackage::read(descriptor_path)
        .with_context(|| format!("Failed to read descriptor at {}", descriptor_path.display()))?;

    let mut tables_missing_pk: Vec<&str> = package
        .dataset
        .iter()
        .filter(|&table| match table.schema.as_ref().unwrap() {
            TableSchema::Object { primary_key, .. } => match primary_key {
                Some(TableSchemaObjectPrimaryKey::Variant0(names)) => names.is_empty(),
                Some(TableSchemaObjectPrimaryKey::Variant1(name)) => name.is_empty(),
                None => true,
            },
            TableSchema::String(_) => unreachable!(),
        })
        .map(|t| t.name.as_str())
        .collect();
    tables_missing_pk.sort();

    if !tables_missing_pk.is_empty() {
        let name_list = tables_missing_pk.join(", ");
        bail!("Cannot publish accelerated package when some tables do not have a primary key defined: {}", name_list)
    }

    let token = session::get_token()?;
    let client = api::Client::new(&token)?;

    client
        .create_version(
            package.id,
            &package.version,
            &api::CreatePackageVersion {
                name: &package.name,
                draft: false,
                accelerated: package.accelerated,
                description: &package.description.unwrap_or("".into()),
                dataset: &package.dataset,
            },
        )
        .await?;

    eprintln!(
        "Published package {} version {}",
        package.name, package.version
    );
    if package.accelerated {
        eprint!(
            "This version is ✨ accelerated. ✨ What this means:
            - Patch is performing the intial data acceleration now.
            - Building release packages (`build-package -p <ref>`) is not
              supported until initial acceleration is complete. To check
              the status of the acceleration, run `package list`."
        )
    }

    Ok(())
}
