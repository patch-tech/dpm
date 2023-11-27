use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

// Source: https://docs.snowflake.com/en/user-guide/admin-account-identifier#organization-name
// We assume that when ^ refers to "letters" that means [a-zA-Z].
const ORG_NAME_PATTERN: &str = "[a-zA-Z][a-zA-Z0-9]*";
const ACCOUNT_NAME_PATTERN: &str = "[a-zA-Z]\\w*";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizationName(String);
impl std::str::FromStr for OrganizationName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = format!("^{}$", ORG_NAME_PATTERN);

        if regress::Regex::new(&pattern).unwrap().find(s).is_none() {
            bail!("doesn't match pattern \"{}\"", pattern);
        }

        Ok(Self(s.to_owned()))
    }
}

/// If `account_name` is of the form '{org}.{account}', prefer that. Otherwise,
/// rely on both org and account name having been provided separately. If
/// neither work out, return `Err`.
///
/// See also: https://docs.snowflake.com/en/user-guide/admin-account-identifier
pub fn resolve_account_identifiers<'a>(
    organization_name: Option<&'a OrganizationName>,
    account_name: &'a str,
) -> Result<(OrganizationName, &'a str)> {
    let combined_pattern = regress::Regex::new(&format!(
        "^(?<org_name>{})[.-](?<account_name>{})$",
        ORG_NAME_PATTERN, ACCOUNT_NAME_PATTERN
    ))
    .unwrap();

    if let Some(result) = combined_pattern.find(account_name) {
        // SAFETY: Regex match implies that both groups matched.
        let org_name = &account_name[result.named_group("org_name").unwrap()];
        let account_name = &account_name[result.named_group("account_name").unwrap()];

        return Ok((OrganizationName(org_name.to_owned()), account_name));
    }

    if let Some(org_name) = organization_name {
        return Ok((org_name.to_owned(), account_name));
    }

    bail!("Invalid account identifers given. Provide account identifiers either via `--account ${{ORG_NAME}}.${{ACCOUNT_NAME}}`, or via `--organization ${{ORG_NAME}} --account ${{ACCOUNT_NAME}}`.")
}
