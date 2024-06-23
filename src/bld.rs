use serde::Deserialize;
use crate::cmn;

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubLayer {
    pub id: usize,
    pub name: String,
    pub layer_type: String,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub discipline: String,
    #[serde(default)]
    pub model_name: String,
    #[serde(default)]
    pub visibility: Option<bool>,
    #[serde(rename="sublayers")]
    pub sub_layers: Option<Vec<SubLayer>>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub summary: Vec<AttributeStats>,
}

#[derive(Debug, Deserialize)]
pub enum MostFrequentValueTypeOptions {
    Str(String),
    Int(i32),
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStats {
    pub field_name: String,
    pub sub_layer_ids: Vec<i32>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub model_name: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub most_frequent_values: Option<Vec<MostFrequentValueTypeOptions>>
}

fn default_layer_type() -> String {
    "Building".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: usize,
    pub name: String,
    pub version: String,
    pub alias: String,
    #[serde(default = "default_layer_type")]
    pub layer_type: String,
    pub filters: Vec<Filter>,
    #[serde(rename="sublayers")]
    pub sub_layers: Vec<SubLayer>,
    pub full_extent: cmn::FullExtent,
    pub spatial_reference: cmn::SpatialReference,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub copyright_text: String,
    pub height_model_info: Option<cmn::HeightModelInfo>,
    #[serde(default)]
    pub active_filter_id: String,
    #[serde(rename="statisticsHRef", default)]
    pub statistics_href: String,
}

impl Default for SceneLayerInfo {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            version: String::new(),
            alias: String::new(),
            layer_type: default_layer_type(),
            filters: vec![],
            sub_layers: vec![],
            full_extent: cmn::FullExtent::default(),
            spatial_reference: cmn::SpatialReference::default(),
            description: String::new(),
            copyright_text: String::new(),
            height_model_info: None,
            active_filter_id: String::new(),
            statistics_href: String::new(),
        }
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub description: String,
    pub filter_bloccks: Vec<FilterBlock>,
    pub is_default_filter: Option<bool>,
    pub is_visible: Option<bool>,
    pub filter_authoring_info: Option<FilterAuthoringInfo>,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterBlock {
    pub title: String,
    pub filter_mode: FilterMode,
    pub filter_expression: String,
}


#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterAuthoringInfo {}


fn default_solid_filter_mode_type() -> String {
    "solid".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeSolid {
    #[serde(rename="type", default = "default_solid_filter_mode_type")]
    pub filter_type: String,
}

impl Default for FilterModeSolid {
    fn default() -> Self {
        Self {
            filter_type: default_solid_filter_mode_type(),
        }
    }
}

fn default_wire_frame_filter_mode_type() -> String {
    "wireFrame".to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterModeWireFrame {
    #[serde(rename="type", default = "default_wire_frame_filter_mode_type")]
    pub filter_type: String,
    pub edges: Option<Edges>,
}

impl Default for FilterModeWireFrame {
    fn default() -> Self {
        Self {
            filter_type: default_wire_frame_filter_mode_type(),
            edges: None,
        }
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edges {
    #[serde(rename = "type")]
    pub type_field: String,
    pub color: Option<Vec<i64>>,
    pub size: Option<f64>,
    pub transparency: Option<i64>,
    pub extension_length: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Solid(FilterModeSolid),
    WireFrame(FilterModeWireFrame),
}

impl Default for FilterMode {
    fn default() -> Self {
        Self::Solid(FilterModeSolid::default())
    }
}