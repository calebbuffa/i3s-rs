use crate::cmn::{Field, HeightModelInfo, ServiceUpdateTimeStamp, SpatialReference, OBB};
use serde::{Deserialize};

fn default_layer_type() -> String {
    "PointCloud".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: i32,
    #[serde(default = "default_layer_type")]
    pub layer_type: String,
    pub name: String,
    pub spatial_reference: SpatialReference,
    pub store: Store,
    pub attribute_storage_info: Vec<AttributeInfo>,
    pub alias: Option<String>,
    pub desc: Option<String>,
    pub copyright_text: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub height_model_info: Option<HeightModelInfo>,
    pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
    pub drawing_info: Option<DrawingInfo>,
    pub elevation_info: Option<ElevationInfo>,
    pub fields: Option<Vec<Field>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeInfo {
    pub key: String,
    pub name: String,
    pub ordering: Option<Vec<String>>,
    pub encoding: Option<String>,
    pub attribute_values: Option<Value>,
}

fn default_nodes_per_page() -> i32 {
    64
}

fn default_bounding_volume_type() -> String {
    "obb".to_string()
}

fn default_lod_selection_metric_type() -> String {
    "density-threshold".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    pub node_version: i32,
    #[serde(default = "default_nodes_per_page")]
    pub nodes_per_page: i32,
    #[serde(default = "default_bounding_volume_type")]
    pub bounding_volume_type: String,
    #[serde(default = "default_lod_selection_metric_type")]
    pub lod_selection_metric_type: String,
    pub href: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub profile: String,
    pub version: String,
    pub extent: [f64; 4],
    pub index: Index,
    pub default_geometry_schema: DefaultGeometrySchema,
    pub id: Option<String>,
    pub geometry_encoding: Option<String>,
    pub attribute_encoding: Option<String>,
}

fn default_geometry_type() -> String {
    "points".to_string()
}

fn default_topology() -> String {
    "PerAttributeArray".to_string()
}

fn default_encoding() -> String {
    "lepcc-xyz".to_string()
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    pub vertex_attributes: VertexAttributes,
    #[serde(default = "default_geometry_type")]
    pub geometry_type: String,
    pub header: Option<()>, // TODO: this is just an empty array
    #[serde(default = "default_topology")]
    pub topology: String,
    #[serde(default = "default_encoding")]
    pub encoding: String,
    pub ordering: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VertexAttributes {
    pub position: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    pub values_per_element: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    pub mode: String,
    pub offset: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    pub algorithm: Algorithm,
    pub points_per_inch: f64,
    pub field: String,
    pub field_transform_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub stops: Vec<Stop>,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Algorithm {
    #[serde(rename = "type")]
    pub type_field: String,
    pub scale_factor: f64,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub value: f64,
    pub color: Vec<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub attribute: Option<String>,
    #[serde(rename = "stats")]
    pub attribute_statistics: Option<AttributeStatistics>,
    pub labels: Option<Labels>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStatistics {
    pub min: f64,
    pub max: f64,
    pub count: f64,
    pub sum: Option<f64>,
    pub avg: Option<f64>,
    pub stddev: Option<f64>,
    pub variance: Option<f64>,
    pub histogram: Option<Histogram>,
    pub most_frequent_values: Option<Vec<ValueCount>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub minimum: f64,
    pub maximum: f64,
    pub counts: Vec<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    pub value: i32,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "bitfieldLabels")]
    pub bit_field_labels: Option<Vec<BitFieldLabel>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub value: i32,
    pub label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitFieldLabel {
    pub bit_number: i32,
    pub label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub resource_id: i32,
    pub first_child: i32,
    pub child_count: i32,
    pub obb: OBB,
    pub vertex_count: Option<i32>,
    pub lod_threshold: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}
