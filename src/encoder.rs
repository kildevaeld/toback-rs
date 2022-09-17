use std::io::{Read, Write};

use super::error::Error;
use serde::{de::DeserializeOwned, Serialize};

pub trait Encoder<T: Serialize + DeserializeOwned> {
    fn extensions(&self) -> &[&str];
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

    fn save_writer(&self, writer: &mut dyn Write, content: &T) -> Result<(), Error> {
        let out = self.save(content)?;
        writer.write_all(&out)?;
        Ok(())
    }

    fn save_writer_pretty(&self, writer: &mut dyn Write, content: &T) -> Result<(), Error> {
        let out = self.save_pretty(content)?;
        writer.write_all(&out)?;
        Ok(())
    }
}
