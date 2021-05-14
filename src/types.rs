use anyhow::Result;
use libept::base::v1::Package;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseVerConfig {
    pub version: String,
    pub use_ept: bool,
}

#[derive(Debug)]
pub struct BaseConfig {
    pub version: BaseVerConfig,
    pub files: toml::map::Map<String, toml::Value>,
}

impl BaseConfig {
    pub fn from_str(i: &String) -> Result<Self> {
        let v = toml::from_str::<toml::Value>(i)?;
        let av = {
            if let Some(z) = v.get("bept") {
                z
            } else {
                return Err(BaseError::BaseConfigError(v)).map_err(anyhow::Error::new);
            }
        };
        let az = {
            if let Some(bv) = av.get("version") {
                bv.to_owned().try_into::<BaseVerConfig>()?
            } else {
                return Err(BaseError::BaseConfigError(v)).map_err(anyhow::Error::new);
            }
        };
        let bz = {
            if let Some(cv) = av.get("files") {
                if let Some(dv) = (*cv).as_table() {
                    dv
                } else {
                    return Err(BaseError::BaseConfigError(v)).map_err(anyhow::Error::new);
                }
            } else {
                return Err(BaseError::BaseConfigError(v)).map_err(anyhow::Error::new);
            }
        };

        Ok(Self {
            version: az,
            files: bz.to_owned(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryNode {
    pub url: String,
    pub edgeless_compat: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryDefault {
    pub index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryConfig {
    pub list: Vec<RegistryNode>,
    pub default: RegistryDefault,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexesOutput {
    pub indexes: Vec<Package>,
}

impl std::ops::Deref for IndexesOutput {
    type Target = Vec<Package>;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}

impl From<Vec<Package>> for IndexesOutput {
    fn from(indexes: Vec<Package>) -> Self {
        Self { indexes }
    }
}

#[derive(Debug, Error)]
pub enum BaseError {
    #[error("Error Config!")]
    BaseConfigError(toml::Value),

    #[error("Unknown Registry")]
    UnknownRegistry,
}
