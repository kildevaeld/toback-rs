use std::io::Error as IOError;
use std::str::Utf8Error;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "json")]
    #[error("json")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "yaml")]
    #[error("yaml")]
    Yaml(#[from] serde_yaml::Error),
    #[cfg(feature = "toml")]
    #[error("toml")]
    Toml(#[from] TomlError),
    #[cfg(feature = "ron")]
    #[error("ron")]
    Ron(#[from] ron::Error),
    #[cfg(feature = "gura")]
    #[error("gura")]
    Gura(#[from] serde_gura::Error),
    #[error("utf8")]
    Utf8(Utf8Error),
    //
    #[error("encoder not found")]
    EncoderNotFound(String),
    #[error("io error {0}")]
    Io(#[from] IOError),
}

#[cfg(feature = "toml")]
#[derive(ThisError, Debug)]
pub enum TomlError {
    #[error("serialize")]
    Serialize(toml::ser::Error),
    #[error("deserialize")]
    Deserialize(toml::de::Error),
}
