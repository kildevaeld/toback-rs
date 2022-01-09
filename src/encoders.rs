#[allow(unused)]
use crate::{Encoder, Error};
#[allow(unused)]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "json")]
#[derive(Clone, Copy)]
pub struct JsonEncoder;

#[cfg(feature = "json")]

impl<T: Serialize + DeserializeOwned> Encoder<T> for JsonEncoder {
    fn extensions(&self) -> &[&str] {
        &["json"]
    }
    fn load(&self, content: Vec<u8>) -> Result<T, Error> {
        Ok(serde_json::from_slice::<T>(&content).map_err(Error::Json)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec_pretty(content).map_err(Error::Json)?)
    }
}

#[cfg(feature = "yaml")]
#[derive(Clone, Copy)]
pub struct YamlEncoder;

#[cfg(feature = "yaml")]
impl<T: Serialize + DeserializeOwned> Encoder<T> for YamlEncoder {
    fn extensions(&self) -> &[&str] {
        &["yaml", "yml"]
    }
    fn load(&self, content: Vec<u8>) -> Result<T, Error> {
        Ok(serde_yaml::from_slice::<T>(&content).map_err(Error::Yaml)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_yaml::to_vec(content).map_err(Error::Yaml)?)
    }
}

#[cfg(feature = "toml")]
#[derive(Clone, Copy)]
pub struct TomlEncoder;

#[cfg(feature = "toml")]
impl<T: Serialize + DeserializeOwned> Encoder<T> for TomlEncoder {
    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
    fn load(&self, content: Vec<u8>) -> Result<T, Error> {
        Ok(toml::from_slice::<T>(&content)
            .map_err(crate::TomlError::Deserialize)
            .map_err(Error::Toml)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(toml::to_vec(content)
            .map_err(crate::TomlError::Serialize)
            .map_err(Error::Toml)?)
    }
}

#[cfg(feature = "ron")]
#[derive(Clone, Copy)]
pub struct RonEncoder;

#[cfg(feature = "ron")]
impl<T: Serialize + DeserializeOwned> Encoder<T> for RonEncoder {
    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
    fn load(&self, content: Vec<u8>) -> Result<T, Error> {
        let content = String::from_utf8(content).map_err(Error::Utf8)?;

        Ok(ron::from_str::<T>(&content).map_err(Error::Ron)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(ron::to_string(content).map(Vec::from).map_err(Error::Ron)?)
    }
}

#[cfg(feature = "gura")]
#[derive(Clone, Copy)]
pub struct GuraEncoder;

#[cfg(feature = "gura")]
impl<T: Serialize + DeserializeOwned> Encoder<T> for GuraEncoder {
    fn extensions(&self) -> &[&str] {
        &["ura"]
    }
    fn load(&self, content: Vec<u8>) -> Result<T, Error> {
        let content = String::from_utf8(content).map_err(Error::Utf8)?;
        Ok(serde_gura::from_str::<T>(&content).map_err(Error::Gura)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_gura::to_string(content)
            .map(Vec::from)
            .map_err(Error::Gura)?)
    }
}
