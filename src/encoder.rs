#[cfg(feature = "std")]
use std::io::{Read, Write};

use super::error::Error;
use alloc::vec::Vec;
use serde::{de::DeserializeOwned, Serialize};

pub trait Encoder<T: Serialize + DeserializeOwned> {
    fn extensions(&self) -> &[&str];
    #[cfg(feature = "std")]
    fn load_reader(&self, reader: &mut dyn Read) -> Result<T, Error> {
        let mut buffer = Vec::default();
        reader.read_to_end(&mut buffer)?;
        self.load(&buffer)
    }
    fn load(&self, content: &[u8]) -> Result<T, Error>;
    fn save(&self, content: &T) -> Result<Vec<u8>, Error>;
    fn save_pretty(&self, content: &T) -> Result<Vec<u8>, Error> {
        self.save(content)
    }

    #[cfg(feature = "std")]
    fn write(&self, writer: &mut dyn Write, content: &T) -> Result<(), Error> {
        let out = self.save(content)?;
        writer.write_all(&out)?;
        Ok(())
    }

    #[cfg(feature = "std")]
    fn write_pretty(&self, writer: &mut dyn Write, content: &T) -> Result<(), Error> {
        let out = self.save_pretty(content)?;
        writer.write_all(&out)?;
        Ok(())
    }
}
