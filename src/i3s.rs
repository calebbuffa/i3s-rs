use std::error::Error;
use std::io::Read;
use std::path::Path;
use url::Url;

use crate::bld;
use crate::cmn;
use crate::io;
use crate::io::ZipFileReadError;
use crate::pcl;
use crate::psl;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum I3SInfo {
    IntegratedMesh(cmn::SceneLayerInformation),
    DDDObject(cmn::SceneLayerInformation),
    Point(psl::SceneLayerInformation),
    Building(bld::SceneLayerInformation),
    PointCloud(pcl::SceneLayerInformation),
}

impl I3SInfo {
    pub fn layer_type(&self) -> &str {
        match self {
            I3SInfo::IntegratedMesh(_) => "IntegratedMesh",
            I3SInfo::DDDObject(_) => "3DObject",
            I3SInfo::Point(_) => "Point",
            I3SInfo::Building(_) => "Building",
            I3SInfo::PointCloud(_) => "PointCloud",
        }
    }
}

pub trait I3SFormat {}
pub trait I3SProfile {}

fn unpack_scene_layer_information(buffer: &[u8]) -> Result<I3SInfo, Box<dyn Error>> {
    let scene_layer_info = serde_json::from_slice::<serde_json::Value>(buffer)?;
    let layer_type = get_layer_type(&scene_layer_info)?;

    match layer_type {
        "IntegratedMesh" => {
            let parsed = serde_json::from_value::<cmn::SceneLayerInformation>(scene_layer_info)?;
            Ok(I3SInfo::IntegratedMesh(parsed))
        }
        "Point" => {
            let parsed = serde_json::from_value::<psl::SceneLayerInformation>(scene_layer_info)?;
            Ok(I3SInfo::Point(parsed))
        }
        "Building" => {
            let parsed = serde_json::from_value::<bld::SceneLayerInformation>(scene_layer_info)?;
            Ok(I3SInfo::Building(parsed))
        }
        "PointCloud" => {
            let parsed = serde_json::from_value::<pcl::SceneLayerInformation>(scene_layer_info)?;
            Ok(I3SInfo::PointCloud(parsed))
        }
        "3DObject" => {
            let parsed = serde_json::from_value::<cmn::SceneLayerInformation>(scene_layer_info)?;
            Ok(I3SInfo::DDDObject(parsed))
        }
        _ => Err("Layer type not supported".into()),
    }
}

fn get_node_page_index(node_index: &f32, nodes_per_page: &f32) -> usize {
    (node_index / nodes_per_page).floor() as usize
}

fn get_node_index_in_node_page(node_index: &f32, nodes_per_page: &f32) -> usize {
    (node_index % nodes_per_page) as usize
}

pub fn get_layer_type(scene_layer_info: &serde_json::Value) -> Result<&str, Box<dyn Error>> {
    let layer_type = scene_layer_info
        .get("layerType")
        .ok_or("Cannot parse layer type")?
        .as_str()
        .ok_or("Cannot parse layer type")?;
    Ok(layer_type)
}
#[derive(Debug)]
pub struct Service {
    pub base: Url,
    client: reqwest::Client,
}

impl I3SFormat for Service {}

impl Service {
    pub fn connect(base: Url) -> Self {
        let client = reqwest::Client::new();
        Self { base, client }
    }

    /*
    Every Service must have scene layer information at layers/0, so we return an error
    if the file is not found. Something is either wrong with the service or the code.
    */
    pub async fn scene_layer_information(&mut self) -> Result<I3SInfo, Box<dyn Error>> {
        let mut buffer = self.get("layers/0").await?;
        buffer = io::decode_gzip_buffer(&buffer)?;
        unpack_scene_layer_information(&buffer)
    }

    // TODO add error type
    pub async fn get(&self, path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        match path {
            "" => {
                let url = self.base.clone();
                let resp = self.client.get(url).send().await?;
                Ok(resp.bytes().await?.to_vec())
            }
            _ => {
                let url = self.base.join(path)?;
                let resp = self.client.get(url).send().await?;
                Ok(resp.bytes().await?.to_vec())
            }
        }
    }
}

#[derive(Debug)]
pub struct SceneLayerPackage {
    zip_archive: zip::ZipArchive<std::fs::File>,
}

impl I3SFormat for SceneLayerPackage {}

impl SceneLayerPackage {
    pub fn get(&mut self, path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut zip_file = self.zip_archive.by_name(path)?;
        let mut buffer = Vec::new();
        zip_file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ZipFileReadError> {
        let file = std::fs::File::open(path)?;
        let zip_archive = zip::ZipArchive::new(file)?;
        Ok(SceneLayerPackage { zip_archive })
    }

    /*
    Every .slpk must have a 3dSceneLayer.json.gz file, so we return an error
    if the file is not found. Something is either wrong with the file or the code.
    */
    pub fn scene_layer_information(&mut self) -> Result<I3SInfo, Box<dyn Error>> {
        let mut buffer = self.get("3dSceneLayer.json.gz")?;
        buffer = io::decode_gzip_buffer(&buffer)?;
        unpack_scene_layer_information(&buffer)
    }

    /*
    metadata.json is unique to .slpk files, and is not required so we return
    None if the file is not found.
    */
    pub fn metadata(&mut self) -> Option<cmn::Metadata> {
        if let Ok(buffer) = self.get("metadata.json") {
            if let Ok(metadata) = serde_json::from_slice::<cmn::Metadata>(&buffer) {
                Some(metadata)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.zip_archive.is_empty()
    }
}

#[derive(Default, Debug)]
pub struct IntegratedMesh {
    pub node_pages: Vec<cmn::NodePage>,
    root_index: usize,
    nodes_per_page: usize,
}

impl I3SProfile for IntegratedMesh {}

impl IntegratedMesh {
    pub fn node(&self, index: &usize) -> &cmn::Node {
        let nodes_per_page: f32 = self.nodes_per_page as f32;
        let index = *index as f32;
        let node_page_index = get_node_page_index(&index, &nodes_per_page);
        let node_index = get_node_index_in_node_page(&index, &nodes_per_page);
        &self.node_pages[node_page_index].nodes[node_index]
    }

    pub fn nodes(&self, indices: &[usize]) -> Vec<&cmn::Node> {
        indices.iter().map(|i| self.node(i)).collect()
    }

    pub fn root(&self) -> &cmn::Node {
        self.node(&self.root_index)
    }
}

#[derive(Debug)]
pub struct DDDObject {
    pub statistics: cmn::AttributeStatistics,
    pub node_pages: Vec<cmn::NodePage>,
}

impl I3SProfile for DDDObject {}

#[derive(Debug)]
pub struct PointCloud {
    pub statistics: pcl::Statistics,
    pub node_pages: Vec<pcl::NodePage>,
}

impl I3SProfile for PointCloud {}

#[derive(Debug)]
pub struct Point {
    pub statistics: cmn::Statistics,
    pub node_pages: Vec<cmn::NodePage>,
}

impl I3SProfile for Point {}

#[derive(Debug)]
pub struct Building {
    pub statistics: bld::Statistics,
    pub sub_layer: bld::SubLayer,
}

impl I3SProfile for Building {}

pub struct SceneLayer<F, P>
where
    F: I3SFormat,
    P: I3SProfile,
{
    pub format: F,
    pub profile: P,
    pub information: I3SInfo,
}
