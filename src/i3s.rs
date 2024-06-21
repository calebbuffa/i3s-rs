use std::{fs::File, io::Read};

use crate::bld;
use crate::cmn;
use crate::io::ZipFileReader;
use crate::pcl;
use crate::psl;

fn find_node_page_paths(zip_archive: &zip::ZipArchive<File>) -> Vec<String> {
    zip_archive
        .file_names()
        .filter(|name| name.contains("nodepages"))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

enum I3S {
    IntegratedMesh,
    Point,
    DDDObject,
    PointCloud,
    Building,
}

#[derive(Default, Debug)]
pub struct IntegratedMesh {
    pub scene_layer_info: cmn::SceneLayerInfo,
    pub metadata: cmn::Metadata,
    pub nodes: Vec<cmn::Node>,
}

impl IntegratedMesh {
    pub fn from_slpk(zip_archive: &mut zip::ZipArchive<std::fs::File>) -> Self {
        let mut integrated_mesh = Self::default();
        {
            let scene_layer_zip_file = zip_archive.by_name("3dSceneLayer.json.gz").unwrap();
            let scene_layer_info = cmn::SceneLayerInfo::from_zip(scene_layer_zip_file).unwrap();
            integrated_mesh.scene_layer_info = scene_layer_info;
        }
        {
            let mut metadata_zip_file = zip_archive.by_name("metadata.json").unwrap();
            let mut metadata_string = String::new();
            metadata_zip_file
                .read_to_string(&mut metadata_string)
                .unwrap();
            let metadata: cmn::Metadata = serde_json::from_str(&metadata_string).unwrap();
            integrated_mesh.metadata = metadata;
        }
        let node_page_paths = find_node_page_paths(zip_archive);
        for node_page_path in node_page_paths {
            let file = zip_archive.by_name(&node_page_path).unwrap();
            let node_page = cmn::NodePage::from_zip(file).unwrap();
            integrated_mesh.nodes.extend(node_page.nodes);
        }
        integrated_mesh
    }

    pub fn get_root_node(&self) -> &cmn::Node {
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
