use super::error::Error;
use serde::{de::DeserializeOwned, Serialize};

pub trait Encoder<T: Serialize + DeserializeOwned>: Send + Sync {
    fn extensions(&self) -> &[&str];
    fn load(&self, content: Vec<u8>) -> Result<T, Error>;
    fn save(&self, content: &T) -> Result<Vec<u8>, Error>;
}
