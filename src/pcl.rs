use crate::cmn::{Field, HeightModelInfo, ServiceUpdateTimeStamp, SpatialReference, OBB};
use crate::io::ZipFileReader;
use serde::Deserialize;

fn default_layer_type() -> String {
    "PointCloud".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInformation {
    pub id: usize,
    #[serde(default = "default_layer_type")]
    pub layer_type: String,
    pub name: String,
    pub store: Store,
    pub attribute_storage_info: Vec<AttributeInfo>,
    pub spatial_reference: Option<SpatialReference>,
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

impl Default for SceneLayerInformation {
    fn default() -> Self {
        Self {
            id: 0,
            layer_type: default_layer_type(),
            name: String::new(),
            spatial_reference: None,
            store: Store::default(),
            attribute_storage_info: vec![],
            alias: None,
            desc: None,
            copyright_text: None,
            capabilities: None,
            height_model_info: None,
            service_update_time_stamp: None,
            drawing_info: None,
            elevation_info: None,
            fields: None,
        }
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeInfo {
    pub key: String,
    pub name: String,
    pub ordering: Option<Vec<String>>,
    pub encoding: Option<String>,
    pub attribute_values: Option<Value>,
}

fn default_nodes_per_page() -> usize {
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
    pub node_version: usize,
    #[serde(default = "default_nodes_per_page")]
    pub nodes_per_page: usize,
    #[serde(default = "default_bounding_volume_type")]
    pub bounding_volume_type: String,
    #[serde(default = "default_lod_selection_metric_type")]
    pub lod_selection_metric_type: String,
    pub href: Option<String>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            node_version: 0,
            nodes_per_page: default_nodes_per_page(),
            bounding_volume_type: default_bounding_volume_type(),
            lod_selection_metric_type: default_lod_selection_metric_type(),
            href: None,
        }
    }
}

#[derive(Default, Debug, Deserialize)]
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

fn default_encoding() -> String {
    "lepcc-xyz".to_string()
}

#[derive(Debug, Deserialize)]
pub struct GeometryType(String);

impl From<GeometryType> for String {
    fn from(geometry_type: GeometryType) -> Self {
        geometry_type.0
    }
}

impl Default for GeometryType {
    fn default() -> Self {
        GeometryType(String::from("Points"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Topology(String);

impl From<Topology> for String {
    fn from(topology: Topology) -> Self {
        topology.0
    }
}

impl Default for Topology {
    fn default() -> Self {
        Topology(String::from("PerAttributeArray"))
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    pub vertex_attributes: VertexAttributes,
    pub geometry_type: GeometryType,
    pub topology: Topology,
    #[serde(default = "default_encoding")]
    pub encoding: String,
    pub ordering: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VertexAttributes {
    pub position: Value,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    pub values_per_element: usize,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    pub mode: String,
    pub offset: Option<f64>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    pub algorithm: Option<Algorithm>,
    pub points_per_inch: f64,
    pub field: String,
    pub field_transform_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub stops: Option<Vec<Stop>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Algorithm {
    #[serde(rename = "type")]
    pub type_field: String,
    pub scale_factor: f64,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self {
            type_field: String::new(),
            scale_factor: 1.0,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub value: f64,
    pub color: Vec<i64>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub attribute: Option<String>,
    #[serde(rename = "stats")]
    pub attribute_statistics: Option<AttributeStatistics>,
    pub labels: Option<Labels>,
}

#[derive(Default, Debug, Deserialize)]
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

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub minimum: f64,
    pub maximum: f64,
    pub counts: Vec<usize>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    pub value: i32,
    pub count: usize,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub labels: Option<Vec<Label>>,
    pub bitfield_labels: Option<Vec<BitfieldLabel>>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub value: i32,
    pub label: String,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitfieldLabel {
    pub bit_number: i32,
    pub label: String,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub resource_id: usize,
    pub first_child: usize,
    pub child_count: usize,
    pub obb: OBB,
    pub vertex_count: Option<usize>,
    pub lod_threshold: Option<f64>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}

impl ZipFileReader for NodePage {}
