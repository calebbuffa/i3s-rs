use std::io::Read;

use crate::bld;
use crate::cmn;
use crate::io::{SLPKReader, SceneLayerPackage};
use crate::pcl;
use crate::psl;
use crate::Rest;

#[derive(Default, Debug)]
pub struct IntegratedMesh {
    pub scene_layer_info: cmn::SceneLayerInfo,
    pub metadata: cmn::Metadata,
    pub nodes: Vec<cmn::Node>,
}

impl IntegratedMesh {
    // TODO add error type
    pub fn from_slpk(slpk: &mut SceneLayerPackage) -> Result<Self, Box<dyn std::error::Error>> {
        let mut integrated_mesh = Self::default();
        {
            let scene_layer_zip_file = slpk.get("3dSceneLayer.json.gz")?;
            let scene_layer_info = cmn::SceneLayerInfo::from_slpk(scene_layer_zip_file)?;
            integrated_mesh.scene_layer_info = scene_layer_info;
        }
        {
            let mut metadata_zip_file = slpk.get("metadata.json")?;
            let mut metadata_string = String::new();
            metadata_zip_file.read_to_string(&mut metadata_string)?;
            let metadata: cmn::Metadata = serde_json::from_str(&metadata_string)?;
            integrated_mesh.metadata = metadata;
        }
        let node_page_paths = slpk.node_page_paths();
        for node_page_path in node_page_paths {
            let file = slpk.get(&node_page_path)?;
            let node_page = cmn::NodePage::from_slpk(file)?;
            integrated_mesh.nodes.extend(node_page.nodes);
        }
        Ok(integrated_mesh)
    }

    // TODO add error type
    pub async fn from_rest(stream: &mut Rest) -> Result<Self, Box<dyn std::error::Error>> {
        let mut integrated_mesh = Self::default();
        let scene_layer_info = cmn::SceneLayerInfo::from_rest(stream).await?;
        integrated_mesh.scene_layer_info = scene_layer_info;
        let mut index: u32 = 0;
        loop {
            let node_page_path = format!("layers/0/nodepages/{}", index);
            let resp = stream.get(&node_page_path.as_str()).await?;
            let node_page = resp.json::<cmn::NodePage>().await;
            match node_page {
                Ok(node_page) => {
                    integrated_mesh.nodes.extend(node_page.nodes);
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

    pub fn root_node(&self) -> &cmn::Node {
        let root_index = self.scene_layer_info.node_pages.root_index;
        &self.nodes[root_index]
    }
}

pub struct DDDObject {
    pub scene_layer_info: cmn::SceneLayerInfo,
    pub statistics: cmn::AttributeStatistics,
    pub nodes: Vec<cmn::Node>,
    pub metadata: cmn::Metadata,
}

pub struct PointCloud {
    pub scene_layer_info: pcl::SceneLayerInfo,
    pub nodes: Vec<pcl::Node>,
    pub statistics: pcl::Statistics,
}

pub struct Point {
    pub scene_layer: psl::SceneLayerInfo,
    pub nodes: Vec<cmn::Node>,
    pub statistics: cmn::Statistics,
    #[deprecated(note = "FeatureData is only inluded for 1.6 compatilibity.")]
    pub feature_data: cmn::FeatureData,
}

pub struct Building {
    pub scene_layer: bld::SceneLayerInfo,
    pub sub_layer: bld::SubLayer,
    pub statistics: bld::Statistics,
}

pub enum Formats {
    Rest(Rest),
    SLPK(SceneLayerPackage),
}
pub enum SceneLayers {
    IntegratedMesh,
    Point,
    DDDObject,
    PointCloud,
    Building,
}

const SCENE_LAYER_MAP: [(&str, SceneLayers); 5] = [
    ("IntegratedMesh", SceneLayers::IntegratedMesh),
    ("Point", SceneLayers::Point),
    ("3DObject", SceneLayers::DDDObject),
    ("PointCloud", SceneLayers::PointCloud),
    ("Building", SceneLayers::Building),
];

pub fn open(path: &str) -> Result<(Formats, SceneLayers), Box<dyn std::error::Error>> {
    if path.ends_with(".slpk") {
        unimplemented!("SLPK files are not yet supported.")
    } else {
        unimplemented!("Rest API is not yet supported.")
    }
}
