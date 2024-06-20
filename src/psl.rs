use crate::cmn;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: i32,
    pub layer_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub store: Store,
    pub href: Option<String>,
    pub spatial_reference: Option<cmn::SpatialReference>,
    pub height_model_info: Option<cmn::HeightModelInfo>,
    pub name: Option<String>,
    pub service_update_time_stamp: Option<cmn::ServiceUpdateTimeStamp>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub copyright_text: Option<String>,
    pub z_factor: Option<f64>,
    pub cached_drawing_info: Option<cmn::CachedDrawingInfo>,
    pub drawing_info: Option<cmn::DrawingInfo>,
    pub elevation_info: Option<cmn::ElevationInfo>,
    #[serde(rename = "popupInfo")]
    pub pop_up_info: Option<cmn::PopUpInfo>,
    #[serde(rename = "disablePopup")]
    pub disable_pop_up: Option<bool>,
    pub fields: Option<Vec<cmn::Field>>,
    pub attribute_storage_info: Option<cmn::AttributeStorageInfo>,
    pub statistics_info: Option<cmn::StatisticsInfo>,
    pub point_node_pages: Option<cmn::NodePageDefinition>,
    pub full_extent: Option<cmn::FullExtent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub profile: String,
    pub version: String,
    pub id: Option<String>,
    pub resource_pattern: Option<Vec<String>>,
    pub root_node: Option<String>,
    pub extent: Option<[f64; 4]>,
    #[serde(rename = "indexCRS")]
    pub index_crs: Option<String>,
    #[serde(rename = "vertexCRS")]
    pub vertex_crs: Option<String>,
    pub normal_reference_frame: Option<String>,
    pub nid_encoding: Option<String>,
    pub feature_encoding: Option<String>,
    pub geometry_encoding: Option<String>,
    pub attribute_encoding: Option<String>,
    pub texture_encoding: Option<Vec<String>>,
    pub lod_type: Option<String>,
    pub lod_model: Option<String>,
    pub indexing_scheme: Option<String>,
    pub default_geometry_schema: Option<cmn::DefaultGeometrySchema>,
    pub default_texture_definition: Option<Vec<cmn::Texture>>,
    pub default_material_definition: Option<cmn::MaterialDefinition>,
}

fn default_topology() -> String {
    "point".to_string()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinition {
    #[serde(default = "default_topology")]
    pub topology: String,
    pub geometry_buffers: [GeometryBuffer; 1],
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBuffer {
    compressed_attributes: cmn::CompressedAttributes,
}
