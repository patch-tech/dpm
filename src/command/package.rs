use anyhow::Result;
use clap::Subcommand;
use comfy_table::Table;

use crate::{api::Client, session};

#[derive(Debug, Subcommand)]
pub enum DatasetAction {
    /// List datasets this account is authorized to use (build, query).
    List,
}

pub async fn list() -> Result<()> {
    let token = session::get_token()?;
    let client = Client::new(&token)?;

    let mut response = client.list_datasets().await?;
    response
        .datasets
        .sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let rows: Vec<Vec<String>> = response
        .datasets
        .into_iter()
        .flat_map(|mut p| {
            p.dataset_versions
                .sort_unstable_by(|a, b| b.version.cmp(&a.version));
            p.dataset_versions
                .iter()
                .map(|pv| -> Vec<String> {
                    vec![
                        p.name.to_owned(),
                        pv.version.to_string(),
                        if pv.accelerated {
                            "Accelerated".into()
                        } else {
                            "Direct".into()
                        },
                        pv.patch_state
                            .as_ref()
                            .map(|s| {
                                if let Some(data) = pv
                                    .patch_state_data
                                    .as_ref()
                                    .and_then(|data| serde_json::to_string_pretty(data).ok())
                                {
                                    format!("{}: {}", s, data)
                                } else {
                                    s.to_string()
                                }
                            })
                            .unwrap_or("n/a".into()),
                    ]
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.set_header(vec!["Name", "Version", "Performance mode", "State"]);
    table.add_rows(rows);

    println!("{table}");

    Ok(())
}
