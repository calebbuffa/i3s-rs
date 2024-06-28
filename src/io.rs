use core::fmt;
use serde_json::Value;
use std::error;
use std::fs::File;
use std::io;
use std::io::Read;
use zip::ZipArchive;

use flate2::read::GzDecoder;
use serde_json;
use zip::read::ZipFile;
use zip::result::ZipError;

pub fn find_node_page_paths(zip_archive: &zip::ZipArchive<File>) -> Vec<String> {
    zip_archive
        .file_names()
        .filter(|name| name.contains("nodepages"))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

#[derive(Debug)]
pub enum ZipFileReadError {
    IO(io::Error),
    Json(serde_json::Error),
    Zip(ZipError),
}

impl From<ZipError> for ZipFileReadError {
    fn from(err: ZipError) -> Self {
        ZipFileReadError::Zip(err)
    }
}

impl From<io::Error> for ZipFileReadError {
    fn from(err: io::Error) -> Self {
        ZipFileReadError::IO(err)
    }
}

impl From<serde_json::Error> for ZipFileReadError {
    fn from(err: serde_json::Error) -> Self {
        ZipFileReadError::Json(err)
    }
}

impl fmt::Display for ZipFileReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZipFileReadError: ")
    }
}

pub trait ZipFileReader {
    fn from_zip_file(zip_file: ZipFile) -> Result<Self, Box<dyn std::error::Error + 'static>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let json_string = decode_json_gz(zip_file)?;
        let result: Self = serde_json::from_str(&json_string)?;
        Ok(result)
    }
}

pub fn decode_json_gz<R: Read>(json_gz: R) -> Result<String, io::Error> {
    let mut gzip_decoder = GzDecoder::new(json_gz);
    let mut json_content = String::new();
    gzip_decoder.read_to_string(&mut json_content)?;
    Ok(json_content)
}

pub fn decode_gzip_buffer(buffer: &[u8]) -> Result<Vec<u8>, io::Error> {
    let mut decoder = flate2::read::GzDecoder::new(buffer);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

pub fn unzip_scene_layer_info(
    zip_archive: &mut ZipArchive<File>,
) -> Result<Value, Box<dyn error::Error>> {
    let zip_file = zip_archive.by_name("3dSceneLayer.json.gz")?;
    let json = decode_json_gz(zip_file)?;
    let val = serde_json::from_str::<Value>(&json)?;
    Ok(val)
}
