use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizationName(String);
impl std::str::FromStr for OrganizationName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Source: https://docs.snowflake.com/en/user-guide/admin-account-identifier#organization-name
        // We assume that when ^ refers to "letters" that means [a-zA-Z].
        if regress::Regex::new("^[a-zA-Z][a-zA-Z0-9]*$")
            .unwrap()
            .find(s)
            .is_none()
        {
            bail!("doesn't match pattern \"^[a-zA-Z][a-zA-Z0-9]*$\"");
        }
        Ok(Self(s.to_string()))
    }
}
