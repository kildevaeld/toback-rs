// use std::path::Path;

use crate::{Encoder, Error};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "send")]
type EncoderBox<T> = Box<dyn Encoder<T> + Send + Sync>;

#[cfg(not(feature = "send"))]
type EncoderBox<T> = Box<dyn Encoder<T>>;

pub struct TobackBuilder<T: Serialize + DeserializeOwned> {
    encoders: Vec<EncoderBox<T>>,
}

impl<T: Serialize + DeserializeOwned> Default for TobackBuilder<T> {
    fn default() -> Self {
        #[allow(unused_mut)]
        let mut instance = Self::new();

        #[cfg(feature = "json")]
        instance
            .encoders
            .push(Box::new(crate::encoders::JsonEncoder));
        #[cfg(feature = "yaml")]
        instance
            .encoders
            .push(Box::new(crate::encoders::YamlEncoder));
        #[cfg(feature = "toml")]
        instance
            .encoders
            .push(Box::new(crate::encoders::TomlEncoder));
        #[cfg(feature = "ron")]
        instance
            .encoders
            .push(Box::new(crate::encoders::RonEncoder));
        #[cfg(feature = "gura")]
        instance
            .encoders
            .push(Box::new(crate::encoders::GuraEncoder));

        instance
    }
}

impl<T: Serialize + DeserializeOwned> TobackBuilder<T> {
    pub fn new() -> TobackBuilder<T> {
        TobackBuilder {
            encoders: Vec::default(),
        }
    }

    #[cfg(feature = "send")]
    pub fn encoder<E: Encoder<T> + 'static + Send + Sync>(mut self, encoder: E) -> Self {
        self.encoders.push(Box::new(encoder));
        self
    }

    #[cfg(not(feature = "send"))]
    pub fn encoder<E: Encoder<T> + 'static>(mut self, encoder: E) -> Self {
        self.encoders.push(Box::new(encoder));
        self
    }

    #[cfg(feature = "send")]
    pub fn add_encoder<E: Encoder<T> + 'static + Send + Sync>(&mut self, encoder: E) -> &mut Self {
        self.encoders.push(Box::new(encoder));
        self
    }

    #[cfg(not(feature = "send"))]
    pub fn add_encoder<E: Encoder<T> + 'static>(&mut self, encoder: E) -> &mut Self {
        self.encoders.push(Box::new(encoder));
        self
    }

    pub fn build(self) -> Toback<T> {
        let exts = self
            .encoders
            .iter()
            .flat_map(|m| m.extensions())
            .map(|m| m.to_string())
            .collect();
        Toback {
            encoders: self.encoders,
            exts,
        }
    }
}

pub struct Toback<T: Serialize + DeserializeOwned> {
    encoders: Vec<EncoderBox<T>>,
    exts: Vec<String>,
}

impl<T: Serialize + DeserializeOwned> Toback<T> {
    /// Creates a new with all features enabled formats
    pub fn new() -> Toback<T> {
        TobackBuilder::default().build()
    }

    /// Creates a new empty builder
    pub fn build() -> TobackBuilder<T> {
        TobackBuilder::new()
    }

    pub fn extensions(&self) -> &[String] {
        &self.exts
    }

    pub fn encoder(&self, ext: &str) -> Result<&EncoderBox<T>, Error> {
        match self
            .encoders
            .iter()
            .find(|loader| loader.extensions().contains(&ext))
        {
            Some(s) => Ok(s),
            None => Err(Error::EncoderNotFound(ext.to_string())),
        }
    }

    #[cfg(feature = "std")]
    pub fn encoder_from_path(&self, path: impl AsRef<std::path::Path>) -> Option<&EncoderBox<T>> {
        let path = path.as_ref();
        let ext = match path.extension() {
            Some(ext) => ext,
            None => return None,
        };

        let ext = match ext.to_str() {
            Some(ext) => ext,
            None => return None,
        };

        self.encoder(ext).ok()
    }

    pub fn load(&self, content: &[u8], ext: &str) -> Result<T, Error> {
        let encoder = self.encoder(ext)?;
        encoder.load(content)
    }

    #[cfg(feature = "std")]
    pub fn load_reader<R: std::io::Read>(&self, reader: &mut R, ext: &str) -> Result<T, Error> {
        let encoder = self.encoder(ext)?;
        encoder.load_reader(reader)
    }

    pub fn save(&self, content: &T, ext: &str) -> Result<Vec<u8>, Error> {
        let encoder = self.encoder(ext)?;
        encoder.save(content)
    }

    pub fn save_pretty(&self, content: &T, ext: &str) -> Result<Vec<u8>, Error> {
        let encoder = self.encoder(ext)?;
        encoder.save_pretty(content)
    }

    #[cfg(feature = "std")]
    pub fn write<W: std::io::Write>(
        &self,
        writer: &mut W,
        content: &T,
        ext: &str,
    ) -> Result<(), Error> {
        let encoder = self.encoder(ext)?;
        encoder.write(writer, content)
    }

    #[cfg(feature = "std")]
    pub fn write_pretty<W: std::io::Write>(
        &self,
        writer: &mut W,
        content: &T,
        ext: &str,
    ) -> Result<(), Error> {
        let encoder = self.encoder(ext)?;
        encoder.write_pretty(writer, content)
    }
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[cfg(feature = "lua")]
    #[test]
    fn test() {
        let builder = TobackBuilder::<()>::default();

        builder.encoder(crate::encoders::LuaEncoder::default());
    }
}
