use crate::{Error, Result};

pub fn env_var(name: &str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::EnvVarDoesNotExist(name.to_string()))
}
