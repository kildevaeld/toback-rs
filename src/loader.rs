#[allow(unused)]
use crate::{encoders, Encoder, Error};
use serde::{de::DeserializeOwned, Serialize};

pub struct TobackBuilder<T: Serialize + DeserializeOwned> {
    encoders: Vec<Box<dyn Encoder<T>>>,
}

impl<T: Serialize + DeserializeOwned> Default for TobackBuilder<T> {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut instance = Self::new();

        #[cfg(feature = "json")]
        instance.encoders.push(Box::new(encoders::JsonEncoder));
        #[cfg(feature = "yaml")]
        instance.encoders.push(Box::new(encoders::YamlEncoder));
        #[cfg(feature = "toml")]
        instance.encoders.push(Box::new(encoders::TomlEncoder));
        #[cfg(feature = "ron")]
        instance.encoders.push(Box::new(encoders::RonEncoder));
        #[cfg(feature = "gura")]
        instance.encoders.push(Box::new(encoders::GuraEncoder));

        instance
    }
}

impl<T: Serialize + DeserializeOwned> TobackBuilder<T> {
    pub fn new() -> TobackBuilder<T> {
        TobackBuilder {
            encoders: Vec::default(),
        }
    }

    pub fn encoder<E: Encoder<T> + 'static>(mut self, encoder: E) -> Self {
        self.encoders.push(Box::new(encoder));
        self
    }

    pub fn build(self) -> Toback<T> {
        let exts = self
            .encoders
            .iter()
            .map(|m| m.extensions())
            .flatten()
            .map(|m| m.to_string())
            .collect();
        Toback {
            encoders: self.encoders,
            exts,
        }
    }
}

pub struct Toback<T: Serialize + DeserializeOwned> {
    encoders: Vec<Box<dyn Encoder<T>>>,
    exts: Vec<String>,
}

impl<T: Serialize + DeserializeOwned> Toback<T> {
    pub fn new() -> Toback<T> {
        TobackBuilder::new().build()
    }

    pub fn build() -> TobackBuilder<T> {
        TobackBuilder::new()
    }

    pub fn extensions(&self) -> &[String] {
        &self.exts
    }

    pub fn encoder(&self, ext: &str) -> Result<&dyn Encoder<T>, Error> {
        match self
            .encoders
            .iter()
            .find(|loader| loader.extensions().contains(&ext))
        {
            Some(s) => Ok(s.as_ref()),
            None => Err(Error::EncoderNotFound(ext.to_string())),
        }
    }

    pub fn load(&self, content: &[u8], ext: &str) -> Result<T, Error> {
        let encoder = match self
            .encoders
            .iter()
            .find(|loader| loader.extensions().contains(&ext))
        {
            Some(s) => s,
            None => return Err(Error::EncoderNotFound(ext.to_string())),
        };

        encoder.load(content)
    }

    pub fn save(&self, content: &T, ext: &str) -> Result<Vec<u8>, Error> {
        let encoder = match self
            .encoders
            .iter()
            .find(|loader| loader.extensions().contains(&ext))
        {
            Some(s) => s,
            None => return Err(Error::EncoderNotFound(ext.to_string())),
        };

        encoder.save(content)
    }

    pub fn save_pretty(&self, content: &T, ext: &str) -> Result<Vec<u8>, Error> {
        let encoder = match self
            .encoders
            .iter()
            .find(|loader| loader.extensions().contains(&ext))
        {
            Some(s) => s,
            None => return Err(Error::EncoderNotFound(ext.to_string())),
        };

        encoder.save_pretty(content)
    }
}
