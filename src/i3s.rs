use crate::cmn;
use crate::bld;
use crate::pcl;
use crate::psl;

pub enum I3S {
    IntegratedMesh,
    Point,
    DDDObject,
    PointCloud,
    Building,
}

pub struct IntegratedMesh {
    pub scene_layer_info: cmn::SceneLayerInfo,
    pub node_pages: Vec<cmn::NodePage>,
    pub metadata: cmn::Metadata,
}

pub struct DDDObject {
    pub scene_layer_info: cmn::SceneLayerInfo,
    pub node_pages: Vec<cmn::NodePage>,
    pub statistics: cmn::AttributeStatistics,
    pub metadata: cmn::Metadata,
}

pub struct PointCloud {
    pub scene_layer_info: pcl::SceneLayerInfo,
    pub node_pages: Vec<pcl::NodePage>,
    pub statistics: pcl::Statistics,
}

pub struct Point {
    pub scene_layer: psl::SceneLayerInfo,
    pub node_pages: Vec<cmn::NodePage>,
    pub statistics: cmn::Statistics,
    #[deprecated(note = "FeatureData is only inluded for 1.6 compatilibity.")]
    pub feature_data: cmn::FeatureData,
}

pub struct Building {
    pub scene_layer: bld::SceneLayerInfo,
    pub sub_layer: bld::SubLayer,
    pub statistics: bld::Statistics,
}