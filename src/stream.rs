use std::io::{Error, Read};

use flate2::read::GzDecoder;
use zip::read::ZipFile;
use serde_json;

#[derive(Debug)]
pub enum ZipReadError {
    IO(Error),
    Json(serde_json::Error),
}

impl From<Error> for ZipReadError {
    fn from(err: Error) -> Self {
        ZipReadError::IO(err)
    }
}

impl From<serde_json::Error> for ZipReadError {
    fn from(err: serde_json::Error) -> Self {
        ZipReadError::Json(err)
    }
}
pub trait ZipFileReader {
    fn from_zip(zip_file: &mut ZipFile) -> Result<Self, ZipReadError>
    where Self: Sized + serde::de::DeserializeOwned {
        let json_string = decode_json_gz(zip_file)?;
        let result = serde_json::from_str(&json_string);
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(ZipReadError::Json(err)),
        }
    }
}

pub fn decode_json_gz<R: Read>(json_gz: &mut R) -> Result<String, Error> {
    let mut gzip_decoder = GzDecoder::new(json_gz);
    let mut json_content = String::new();
    gzip_decoder.read_to_string(&mut json_content)?;
    Ok(json_content)
}