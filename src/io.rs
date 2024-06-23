use core::fmt;
use std::fs::File;
use std::io::{Error, Read};

use flate2::read::GzDecoder;
use zip::read::ZipFile;
use serde_json;

fn find_node_page_paths(zip_archive: &zip::ZipArchive<File>) -> Vec<String> {
    zip_archive
        .file_names()
        .filter(|name| name.contains("nodepages"))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

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

impl fmt::Display for ZipReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZipReadError::IO(err) => write!(f, "IO error: {}", err),
            ZipReadError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl From<serde_json::Error> for ZipReadError {
    fn from(err: serde_json::Error) -> Self {
        ZipReadError::Json(err)
    }
}
pub trait SLPKReader {
    fn from_slpk(slpk: ZipFile) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let json_string = decode_json_gz(slpk)?;
        let result: Self = serde_json::from_str(&json_string)?;
        Ok(result)
        // match result {
        //     Ok(value) => Ok(value),
        //     Err(err) => Err(ZipReadError::Json(err)),
        // }
    }
}

pub fn decode_json_gz<R: Read>(json_gz: R) -> Result<String, Error> {
    let mut gzip_decoder = GzDecoder::new(json_gz);
    let mut json_content = String::new();
    gzip_decoder.read_to_string(&mut json_content)?;
    Ok(json_content)
}

pub struct SceneLayerPackage {
    zip_archive: zip::ZipArchive<std::fs::File>,
}

impl SceneLayerPackage {

    // TODO add error type
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;
        let zip_archive = zip::ZipArchive::new(file)?;
        Ok(SceneLayerPackage { zip_archive })
    }

    // TODO add error type
    pub fn get(&mut self, path: &str) -> Result<ZipFile, Box<dyn std::error::Error>> {
        self.zip_archive.by_name(path).map_err(|err| err.into())
    }

    pub fn node_page_paths(&self) -> Vec<String> {
        find_node_page_paths(&self.zip_archive)
    }
}
