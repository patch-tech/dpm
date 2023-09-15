use std::path::Path;

use anyhow::{Context, Result};

use crate::{api, descriptor::DataPackage, session};

pub async fn publish(descriptor_path: &Path) -> Result<()> {
    let package = DataPackage::read(descriptor_path)
        .with_context(|| format!("Failed to read descriptor at {}", descriptor_path.display()))?;

    let token = session::get_token()?;
    let client = api::Client::new(&token)?;

    client
        .create_version(
            package.id,
            &package.version,
            &api::CreatePackageVersion {
                name: &package.name,
                draft: false,
                description: &package.description.unwrap_or("".into()),
                dataset: &package.dataset,
            },
        )
        .await?;

    eprintln!(
        "Published package {} version {}",
        package.name, package.version
    );

    Ok(())
}
