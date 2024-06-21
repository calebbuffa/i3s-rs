use crate::cmn::{Field, HeightModelInfo, ServiceUpdateTimeStamp, SpatialReference, OBB};
use crate::io::ZipFileReader;
use serde::Deserialize;

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

impl Default for SceneLayerInfo {
    fn default() -> Self {
        Self {
            id: 0,
            layer_type: default_layer_type(),
            name: String::new(),
            spatial_reference: SpatialReference::default(),
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

fn default_geometry_type() -> String {
    "points".to_string()
}

fn default_topology() -> String {
    "PerAttributeArray".to_string()
}

fn default_encoding() -> String {
    "lepcc-xyz".to_string()
}


#[derive(Default, Debug, Deserialize)]
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

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VertexAttributes {
    pub position: Value,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    pub values_per_element: i32,
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

impl Default for Renderer {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::default(),
            points_per_inch: 1.0,
            field: String::new(),
            field_transform_type: String::new(),
            type_field: String::new(),
            stops: vec![],
        }
    }
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub value: f64,
    pub color: Vec<i64>,
}

impl Default for Stop {
    fn default() -> Self {
        Self {
            value: -1.0,
            color: vec![],
        }
    }

}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub attribute: Option<String>,
    #[serde(rename = "stats")]
    pub attribute_statistics: Option<AttributeStatistics>,
    pub labels: Option<Labels>,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            attribute: None,
            attribute_statistics: None,
            labels: None,
        }
    }
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

impl Default for AttributeStatistics {
    fn default() -> Self {
        Self {
            min: -1.0,
            max: -1.0,
            count: -1.0,
            sum: None,
            avg: None,
            stddev: None,
            variance: None,
            histogram: None,
            most_frequent_values: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub minimum: f64,
    pub maximum: f64,
    pub counts: Vec<u16>,
}

impl Default for Histogram {
    fn default() -> Self {
        Self {
            minimum: -1.0,
            maximum: -1.0,
            counts: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    pub value: i32,
    pub count: i64,
}

impl Default for ValueCount {
    fn default() -> Self {
        Self {
            value: -1,
            count: -1,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "bitfieldLabels")]
    pub bit_field_labels: Option<Vec<BitFieldLabel>>,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            labels: None,
            bit_field_labels: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub value: i32,
    pub label: String,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            value: -1,
            label: String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitFieldLabel {
    pub bit_number: i32,
    pub label: String,
}

impl Default for BitFieldLabel {
    fn default() -> Self {
        Self {
            bit_number: -1,
            label: String::new(),
        }
    }

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

impl Default for Node {
    fn default() -> Self {
        Self {
            resource_id: 0,
            first_child: 0,
            child_count: 0,
            obb: OBB::default(),
            vertex_count: None,
            lod_threshold: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}


impl Default for NodePage {
    fn default() -> Self {
        Self {
            nodes: vec![],
        }
    }
}
impl ZipFileReader for NodePage {}