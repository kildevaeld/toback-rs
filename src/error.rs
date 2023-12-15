use core::fmt;
use core::str::Utf8Error;
#[cfg(feature = "std")]
use std::io::Error as IoError;

use alloc::string::String;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "lua")]
    Lua(mlua::Error),
    #[cfg(feature = "json")]
    Json(serde_json::Error),
    #[cfg(feature = "yaml")]
    Yaml(serde_yaml::Error),
    #[cfg(feature = "toml")]
    Toml(TomlError),
    #[cfg(feature = "ron")]
    Ron(ron::Error),
    #[cfg(feature = "gura")]
    Gura(serde_gura::Error),
    Utf8(Utf8Error),
    EncoderNotFound(String),
    #[cfg(feature = "std")]
    Io(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "json")]
            Error::Json(err) => write!(f, "json error: {err}"),
            #[cfg(feature = "lua")]
            Error::Lua(err) => write!(f, "lua error: {err}"),
            #[cfg(feature = "yaml")]
            Error::Yaml(err) => write!(f, "yaml error: {err}"),
            #[cfg(feature = "toml")]
            Error::Toml(err) => write!(f, "tom error: {err}"),
            #[cfg(feature = "ron")]
            Error::Ron(err) => write!(f, "ron error: {err}"),
            #[cfg(feature = "gura")]
            Error::Gura(err) => write!(f, "gura error: {err}"),
            Error::Utf8(err) => write!(f, "utf8 error: {err}"),
            Error::EncoderNotFound(err) => write!(f, "encoder not found: {err}"),
            #[cfg(feature = "std")]
            Error::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

#[cfg(feature = "std")]
impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::Io(error)
    }
}

#[cfg(feature = "toml")]
#[derive(Debug)]
pub enum TomlError {
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

#[cfg(feature = "toml")]
impl fmt::Display for TomlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TomlError::Deserialize(s) => s.fmt(f),
            TomlError::Serialize(s) => s.fmt(f),
        }
    }
}

#[cfg(all(feature = "std", feature = "toml"))]
impl std::error::Error for TomlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TomlError::Serialize(err) => Some(err),
            TomlError::Deserialize(err) => Some(err),
        }
    }
}

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for TomlError {
    fn from(value: toml::ser::Error) -> Self {
        TomlError::Serialize(value)
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for TomlError {
    fn from(value: toml::de::Error) -> Self {
        TomlError::Deserialize(value)
    }
}

#[cfg(feature = "lua")]
impl From<mlua::Error> for Error {
    fn from(value: mlua::Error) -> Error {
        Error::Lua(value)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}

#[cfg(feature = "ron")]
impl From<ron::Error> for Error {
    fn from(value: ron::Error) -> Self {
        Error::Ron(value)
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Error::Yaml(value)
    }
}

#[cfg(feature = "gura")]
impl From<serde_gura::Error> for Error {
    fn from(value: serde_gura::Error) -> Self {
        Error::Gura(value)
    }
}

#[cfg(feature = "toml")]
impl From<TomlError> for Error {
    fn from(value: TomlError) -> Self {
        Error::Toml(value)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "json")]
            Error::Json(err) => Some(err),
            #[cfg(feature = "yaml")]
            Error::Yaml(err) => Some(err),
            #[cfg(feature = "toml")]
            Error::Toml(err) => Some(err),
            #[cfg(feature = "ron")]
            Error::Ron(err) => Some(err),
            #[cfg(feature = "gura")]
            Error::Gura(err) => Some(err),
            #[cfg(feature = "lua")]
            Error::Lua(err) => Some(err),
            Error::Utf8(err) => Some(err),
            Error::EncoderNotFound(_) => None,
            Error::Io(err) => Some(err),
        }
    }
}
