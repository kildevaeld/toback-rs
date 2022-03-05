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
    fn load_reader(&self, reader: &mut dyn std::io::Read) -> Result<T, Error> {
        Ok(serde_json::from_reader(reader)?)
    }
    fn load(&self, content: &[u8]) -> Result<T, Error> {
        Ok(serde_json::from_slice::<T>(&content).map_err(Error::Json)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(content).map_err(Error::Json)?)
    }

    fn save_pretty(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec_pretty(content).map_err(Error::Json)?)
    }

    fn save_writer(&self, writer: &mut dyn std::io::Write, content: &T) -> Result<(), Error> {
        Ok(serde_json::to_writer(writer, content)?)
    }

    fn save_writer_pretty(
        &self,
        writer: &mut dyn std::io::Write,
        content: &T,
    ) -> Result<(), Error> {
        Ok(serde_json::to_writer_pretty(writer, content)?)
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

    fn load_reader(&self, reader: &mut dyn std::io::Read) -> Result<T, Error> {
        Ok(serde_yaml::from_reader(reader)?)
    }

    fn load(&self, content: &[u8]) -> Result<T, Error> {
        Ok(serde_yaml::from_slice::<T>(&content).map_err(Error::Yaml)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_yaml::to_vec(content).map_err(Error::Yaml)?)
    }

    fn save_writer(&self, writer: &mut dyn std::io::Write, content: &T) -> Result<(), Error> {
        Ok(serde_yaml::to_writer(writer, content)?)
    }

    fn save_writer_pretty(
        &self,
        writer: &mut dyn std::io::Write,
        content: &T,
    ) -> Result<(), Error> {
        Ok(serde_yaml::to_writer(writer, content)?)
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
    fn load(&self, content: &[u8]) -> Result<T, Error> {
        Ok(toml::from_slice::<T>(&content)
            .map_err(crate::TomlError::Deserialize)
            .map_err(Error::Toml)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(toml::to_vec(content)
            .map_err(crate::TomlError::Serialize)
            .map_err(Error::Toml)?)
    }

    fn save_pretty(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(toml::ser::to_string_pretty(content)
            .map(Vec::from)
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
    fn load_reader(&self, reader: &mut dyn std::io::Read) -> Result<T, Error> {
        Ok(ron::de::from_reader(reader)?)
    }
    fn load(&self, content: &[u8]) -> Result<T, Error> {
        Ok(ron::de::from_bytes::<T>(&content).map_err(Error::Ron)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(ron::ser::to_string(content)
            .map(Vec::from)
            .map_err(Error::Ron)?)
    }

    fn save_pretty(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(
            ron::ser::to_string_pretty(content, ron::ser::PrettyConfig::default())
                .map(Vec::from)
                .map_err(Error::Ron)?,
        )
    }

    fn save_writer(&self, writer: &mut dyn std::io::Write, content: &T) -> Result<(), Error> {
        Ok(ron::ser::to_writer(writer, content)?)
    }

    fn save_writer_pretty(
        &self,
        writer: &mut dyn std::io::Write,
        content: &T,
    ) -> Result<(), Error> {
        Ok(ron::ser::to_writer_pretty(
            writer,
            content,
            ron::ser::PrettyConfig::default(),
        )?)
    }
}

#[cfg(feature = "gura")]
#[derive(Clone, Copy)]
pub struct GuraEncoder;

#[cfg(feature = "gura")]
impl<T: Serialize + DeserializeOwned> Encoder<T> for GuraEncoder {
    fn extensions(&self) -> &[&str] {
        &["ura", "gura"]
    }
    fn load(&self, content: &[u8]) -> Result<T, Error> {
        let content = std::str::from_utf8(content).map_err(Error::Utf8)?;
        Ok(serde_gura::from_str::<T>(&content).map_err(Error::Gura)?)
    }
    fn save(&self, content: &T) -> Result<Vec<u8>, Error> {
        Ok(serde_gura::to_string(content)
            .map(Vec::from)
            .map_err(Error::Gura)?)
    }
}
