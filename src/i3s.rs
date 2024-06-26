use bytes::Bytes;
use std::error::Error;
use std::path::Path;
use url::Url;

use crate::bld;
use crate::cmn;
use crate::io;
use crate::io::{find_node_page_paths, ZipFileReadError};
use crate::pcl;
use crate::psl;

pub fn get_node_page_index(node_index: &f32, nodes_per_page: &f32) -> usize {
    (node_index / nodes_per_page).floor() as usize
}

pub fn get_node_index_in_node_page(node_index: &f32, nodes_per_page: &f32) -> usize {
    (node_index % nodes_per_page) as usize
}

// pub trait Parse {
//     async fn parse(&mut self) -> Profile;
// }

// pub trait Get {
//     async fn get(&mut self, path: &str) -> Result<Bytes, Box<dyn Error>>;
// }

#[derive(Debug)]
struct ServiceBuilder {
    base: Url,
    client: reqwest::Client,
    scene_layer_info_path: &'static str,
}

impl ServiceBuilder {
    pub fn new(base: Url) -> Self {
        let client = reqwest::Client::new();
        Self {
            base,
            client,
            scene_layer_info_path: "layers/0",
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub fn scene_layer_info_path(mut self, path: &'static str) -> Self {
        self.scene_layer_info_path = path;
        self
    }

    pub fn build(self) -> Service {
        Service {
            base: self.base,
            client: self.client,
            scene_layer_info_path: self.scene_layer_info_path,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    pub base: Url,
    client: reqwest::Client,
    scene_layer_info_path: &'static str,
}

// impl Parse for Service {
//     async fn parse(&mut self) -> Profile {
//         let bytes = self.get(self.scene_layer_info_path).await.unwrap();
//         let parsed = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap();
//         if let Some(layer_type) = parsed.get("layerType").and_then(|v| v.as_str()) {
//             match layer_type {
//                 "IntegratedMesh" => {
//                     todo!()
//                 }
//                 "Point" => {
//                     todo!()
//                 }
//                 "3DObject" => {
//                     todo!()
//                 }
//                 "PointCloud" => {
//                     todo!()
//                 }
//                 "Building" => {
//                     todo!()
//                 }
//                 _ => todo!(),
//             }
//         } else {
//             unimplemented!("Layer type not supported");
//         }
//     }
// }

impl Service {
    pub fn connect(base: Url) -> Self {
        ServiceBuilder::new(base).build()
    }

    // TODO add error type
    pub async fn get(&self, path: &str) -> Result<Bytes, Box<dyn Error>> {
        match path {
            "" => {
                let url = self.base.clone();
                let resp = self.client.get(url).send().await?;
                Ok(resp.bytes().await?)
            }
            _ => {
                let url = self.base.join(path)?;
                let resp = self.client.get(url).send().await?;
                Ok(resp.bytes().await?)
            }
        }
    }
}

#[derive(Debug)]
pub struct SceneLayerPackage {
    zip_archive: zip::ZipArchive<std::fs::File>,
}

fn get_layer_type(scene_layer_info: &serde_json::Value) -> Result<&str, Box<dyn Error>> {
    let layer_type = scene_layer_info
        .get("layerType")
        .ok_or("Cannot parse layer type")?
        .as_str()
        .ok_or("Cannot parse layer type")?;
    Ok(layer_type)
}

fn parse_scene_layer_info(
    layer_type: &str,
    scene_layer_info: serde_json::Value,
) -> Result<SceneLayerInformation, Box<dyn Error>> {
    match layer_type {
        "IntegratedMesh" => Ok(SceneLayerInformation::IntegratedMesh(
            serde_json::from_value::<cmn::SceneLayerInformation>(scene_layer_info)?,
        )),
        "3DObject" => Ok(SceneLayerInformation::DDDObject(serde_json::from_value::<
            cmn::SceneLayerInformation,
        >(
            scene_layer_info
        )?)),
        "Point" => Ok(SceneLayerInformation::Point(serde_json::from_value::<
            psl::SceneLayerInformation,
        >(scene_layer_info)?)),
        "Building" => Ok(SceneLayerInformation::Building(serde_json::from_value::<
            bld::SceneLayerInformation,
        >(scene_layer_info)?)),
        "PointCloud" => Ok(SceneLayerInformation::PointCloud(serde_json::from_value::<
            pcl::SceneLayerInformation,
        >(
            scene_layer_info
        )?)),
        _ => Err("Unknown layer type".into()),
    }
}

// impl Get for SceneLayerPackage {
//     async fn get(&mut self, path: &str) -> Result<Bytes, Box<dyn Error>> {
//         let mut zip_file = self.zip_archive.by_name(path)?;
//         let mut buffer = Vec::new();
//         zip_file.read_to_end(&mut buffer)?;
//         Ok(Bytes::from(buffer))
//     }
// }

impl SceneLayerPackage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, ZipFileReadError> {
        let file = std::fs::File::open(path)?;
        let zip_archive = zip::ZipArchive::new(file)?;
        Ok(SceneLayerPackage { zip_archive })
    }

    pub fn node_page_paths(&self) -> Vec<String> {
        find_node_page_paths(&self.zip_archive)
    }

    // pub async fn metadata(&mut self) -> Result<cmn::Metadata, Box<dyn Error>> {
    //     let buffer = self.get("metadata.json").await?;
    //     let json = String::from_utf8(buffer.to_vec()).unwrap();
    //     Ok(serde_json::from_str::<cmn::Metadata>(&json)?)
    // }

    pub fn files(&self) -> Vec<String> {
        self.zip_archive
            .file_names()
            .map(|name| name.to_string())
            .collect::<Vec<String>>()
    }

    pub fn size(&self) -> usize {
        self.zip_archive.len()
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

impl IntegratedMesh {
    // pub async fn from_slpk(slpk: &mut SceneLayerPackage) -> Result<Self, Box<dyn Error>> {
    //     let mut integrated_mesh = Self::default();
    //     let scene_layer_info_zip = slpk.get("3dSceneLayer.json.gz").await?;
    //     integrated_mesh.scene_layer_info = SceneLayerInformation::try_from(scene_layer_info_zip)?;
    //     let node_page_paths = slpk.node_page_paths();
    //     for node_page_path in node_page_paths {
    //         let file = slpk.get(&node_page_path)?;
    //         let node_page = cmn::NodePage::from_zip_file(file)?;
    //         integrated_mesh.node_pages.push(node_page);
    //     }
    //     Ok(integrated_mesh)
    // }

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

    // TODO add error type
    pub async fn from_rest(stream: &mut Service) -> Result<Self, Box<dyn std::error::Error>> {
        let mut integrated_mesh = Self::default();
        let mut index: u32 = 0;
        loop {
            let node_page_path = format!("layers/0/nodepages/{}", index);
            let mut bytes = stream.get(node_page_path.as_str()).await?;
            bytes = io::decode_gzip_buffer(&bytes)?;
            let node_page = serde_json::from_slice::<cmn::NodePage>(&bytes);
            match node_page {
                Ok(node_page) => {
                    integrated_mesh.node_pages.push(node_page);
                    index += 1;
                }
                Err(_) => {
                    // esri REST API returns success even when it fails
                    break; // if we can't parse the response, we assume we've
                } // reached the end of the node pages
            }
        }
        Ok(integrated_mesh)
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

#[derive(Debug)]
pub struct PointCloud {
    pub statistics: pcl::Statistics,
    pub node_pages: Vec<pcl::NodePage>,
}

#[derive(Debug)]
pub struct Point {
    pub statistics: cmn::Statistics,
    pub node_pages: Vec<cmn::NodePage>,
}

#[derive(Debug)]
pub struct Building {
    pub statistics: bld::Statistics,
    pub sub_layer: bld::SubLayer,
}

#[derive(Debug)]
pub enum Format {
    REST(Service),
    SLPK(SceneLayerPackage),
    ESLPK(),
}

#[derive(Debug)]
pub enum Profile {
    IntegratedMesh,
    Point,
    DDDObject,
    PointCloud,
    Building,
}

pub enum SceneLayerInformation {
    IntegratedMesh(cmn::SceneLayerInformation),
    DDDObject(cmn::SceneLayerInformation),
    Point(psl::SceneLayerInformation),
    Building(bld::SceneLayerInformation),
    PointCloud(pcl::SceneLayerInformation),
}

impl SceneLayerInformation {
    pub fn layer_type(&self) -> &str {
        match self {
            SceneLayerInformation::IntegratedMesh(_) => "IntegratedMesh",
            SceneLayerInformation::DDDObject(_) => "3DObject",
            SceneLayerInformation::Point(_) => "Point",
            SceneLayerInformation::Building(_) => "Building",
            SceneLayerInformation::PointCloud(_) => "PointCloud",
        }
    }
}

pub struct SceneLayer {
    pub format: Format,
    pub profile: Profile,
    pub information: SceneLayerInformation,
}
