use crate::io;

use serde::Deserialize;
use serde_json;

fn default_texture_value() -> f32 {
    1.0
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: i32,
    pub layer_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub store: Store,
    pub geometry_definitions: Vec<GeometryDefinition>,
    pub name: Option<String>,
    pub full_extent: Option<FullExtent>,
    pub spatial_reference: Option<SpatialReference>,
    pub alias: Option<String>,
    pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
    pub height_model_info: Option<HeightModelInfo>,
    pub drawing_info: Option<DrawingInfo>,
    pub node_pages: Option<NodePageDefinition>,
    pub material_definitions: Option<Vec<MaterialDefinitions>>,
    pub texture_set_definitions: Option<Vec<TextureSetDefinition>>,
    pub description: Option<String>,
    pub copyright_text: Option<String>,
    pub href: Option<String>,
    pub cached_drawing_info: Option<CachedDrawingInfo>,
    #[serde(rename = "disablePopup")]
    pub disable_pop_up: Option<bool>,
    pub attribute_storage_info: Option<Vec<AttributeStorageInfo>>,
    pub statistics_info: Option<Vec<StatisticsInfo>>,
    pub z_factor: Option<f32>,
    #[serde(rename = "popupInfo")]
    pub pop_up_info: Option<PopUpInfo>,
    pub fields: Option<Vec<Field>>,
    pub elevation_info: Option<ElevationInfo>,
}

impl io::ZipFileReader for SceneLayerInfo {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    pub mode: Option<String>,
    pub offset: Option<f32>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub alias: Option<String>,
    pub domain: Option<Domain>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    #[serde(rename = "type")]
    pub domain_type: String,
    pub name: String,
    pub coded_values: Option<Vec<DomainCodedValue>>,
    pub description: Option<String>,
    pub range: Option<[f32; 2]>,
    pub field_type: Option<String>,
    pub merge_policy: Option<String>,
    pub split_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCodedValue {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopUpInfo {
    pub title: Option<String>,
    pub media_infos: Option<Vec<Option<serde_json::Value>>>,
    pub description: Option<String>,
    pub field_infos: Option<Vec<FieldInfo>>,
    pub popup_elements: Option<Vec<PopupElement>>,
    pub expression_infos: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub field_name: Option<String>,
    pub visible: Option<bool>,
    pub is_editable: Option<bool>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupElement {
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub popup_element_type: Option<String>,
    pub field_infos: Option<Vec<FieldInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsInfo {
    pub key: String,
    pub name: String,
    pub href: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStorageInfo {
    pub key: String,
    pub name: String,
    pub header: Vec<HeaderValue>,
    pub ordering: Option<Vec<String>>,
    pub attribute_values: Option<Value>,
    pub attribute_byte_counts: Option<Value>,
    pub object_ids: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {
    pub value_type: String,
    pub property: String,
}

fn default_time_encoding() -> String {
    "ECMA_ISO8601".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    pub encoding: Option<String>,
    #[serde(default = "default_time_encoding")]
    pub time_encoding: String,
    pub values_per_element: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CachedDrawingInfo {
    pub color: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
    pub scale_symbols: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Renderer {
    #[serde(rename = "type")]
    pub renderer_type: Option<String>,
    pub label: Option<String>,
    pub description: Option<String>,
    pub symbol: Option<Symbol>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
    pub symbol_layers: Option<Vec<SymbolLayer>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SymbolLayer {
    #[serde(rename = "type")]
    pub symbol_layer_type: Option<String>,
    pub material: Option<Material>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Material {
    pub color: Option<Vec<i32>>,
    pub transparency: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExtent {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub spatial_reference: Option<SpatialReference>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    pub latest_vcs_wkid: Option<i32>,
    pub latest_wkid: Option<i32>,
    pub vcs_wkid: Option<i32>,
    pub wkid: Option<i32>,
    pub wkt: Option<String>,
}

fn default_geometry_definition_topology() -> String {
    "triangle".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinition {
    pub geometry_buffers: Vec<GeometryBuffer>,
    #[serde(default = "default_geometry_definition_topology")]
    pub topology: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryPosition {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryPosition {
    pub fn new() -> Self {
        GeometryPosition {
            dtype: "Float32".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryPosition {
    fn default() -> Self {
        GeometryPosition::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryNormal {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryNormal {
    pub fn new() -> Self {
        GeometryNormal {
            dtype: "Float32".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryNormal {
    fn default() -> Self {
        GeometryNormal::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryUV {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryUV {
    pub fn new() -> Self {
        GeometryUV {
            dtype: "Float32".to_string(),
            component: 2,
        }
    }
}

impl Default for GeometryUV {
    fn default() -> Self {
        GeometryUV::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryColor {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
}

impl GeometryColor {
    pub fn new() -> Self {
        GeometryColor {
            dtype: "Uint8".to_string(),
            component: 3,
        }
    }
}

impl Default for GeometryColor {
    fn default() -> Self {
        GeometryColor::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFeatureID {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    pub binding: String,
}

impl GeometryFeatureID {
    pub fn new() -> Self {
        GeometryFeatureID {
            dtype: "UInt32".to_string(),
            component: 1,
            binding: "per-feature".to_string(),
        }
    }
}

impl Default for GeometryFeatureID {
    fn default() -> Self {
        GeometryFeatureID::new()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryFaceRange {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    pub binding: String,
}

impl GeometryFaceRange {
    pub fn new(component: Option<i32>) -> Self {
        if let Some(component) = component {
            return GeometryFaceRange {
                dtype: "Uint32".to_string(),
                component: component,
                binding: "per-feature".to_string(),
            };
        } else {
            return GeometryFaceRange {
                dtype: "Uint32".to_string(),
                component: -1,
                binding: "per-feature".to_string(),
            };
        }
    }
}

impl Default for GeometryFaceRange {
    fn default() -> Self {
        GeometryFaceRange::new(None)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBuffer {
    #[serde(default)]
    pub offset: i32,
    #[serde(default)]
    pub position: GeometryPosition,
    #[serde(default)]
    pub normal: GeometryNormal,
    #[serde(default)]
    pub uv0: GeometryUV,
    #[serde(default)]
    pub feature_id: GeometryFeatureID,
    #[serde(default)]
    pub face_range: GeometryFaceRange,
    #[serde(default)]
    pub compressed_attributes: CompressedAttributes,
}

fn default_compressed_attributes_encoding() -> String {
    "draco".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompressedAttributes {
    #[serde(default = "default_compressed_attributes_encoding")]
    pub encoding: String,
    pub attributes: Option<Vec<String>>,
}

impl Default for CompressedAttributes {
    fn default() -> Self {
        CompressedAttributes {
            encoding: default_compressed_attributes_encoding(),
            attributes: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeometryBufferMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    pub binding: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeightModelInfo {
    pub height_model: Option<String>,
    #[serde(rename = "vertCRS")]
    pub vert_crs: Option<String>,
    pub height_unit: Option<String>,
}

fn default_alpha_cutoff() -> f32 {
    0.25
}

fn default_cull_face() -> String {
    "none".to_string()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitions {
    #[serde(default)]
    pub pbr_metallic_roughness: PbrMetallicRoughness,
    pub normal_texture: Option<MaterialTexture>,
    pub occlusion_texture: Option<MaterialTexture>,
    pub emissive_texture: Option<MaterialTexture>,
    pub emissive_factor: Option<[f32; 3]>,
    pub alpha_mode: Option<String>,
    #[serde(default = "default_alpha_cutoff")]
    pub alpha_cutoff: f32,
    #[serde(default)]
    pub double_sided: bool,
    #[serde(default = "default_cull_face")]
    pub cull_face: String,
}

fn default_base_color_factor() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    #[serde(default = "default_base_color_factor")]
    pub base_color_factor: [f32; 4],
    #[serde(default = "default_texture_value")]
    pub metallic_factor: f32,
    #[serde(default = "default_texture_value")]
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<MaterialTexture>,
    pub base_color_texture: Option<MaterialTexture>,
}

impl Default for PbrMetallicRoughness {
    fn default() -> Self {
        PbrMetallicRoughness {
            base_color_factor: default_base_color_factor(),
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            base_color_texture: None,
            metallic_roughness_texture: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTexture {
    pub texture_set_definition_id: i32,
    #[serde(default = "default_texture_value")]
    pub factor: f32,
    #[serde(default)]
    #[deprecated]
    pub tex_coord: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePageDefinition {
    pub nodes_per_page: i32,
    pub lod_selection_metric_type: String,
    #[serde(default)]
    pub root_index: i32, // default is 0
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdateTimeStamp {
    pub last_update: i64, // Unix epoch from 1 January 1970 in milliseconds
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub profile: String,
    pub version: String,
    pub id: Option<String>,
    pub resource_pattern: Option<Vec<String>>,
    pub root_node: Option<String>,
    pub extent: Option<Vec<f64>>,
    #[serde(rename = "indexCRS")]
    pub index_crs: Option<String>,
    #[serde(rename = "vertexCRS")]
    pub vertex_crs: Option<String>,
    pub normal_reference_frame: Option<String>,
    pub default_geometry_schema: Option<DefaultGeometrySchema>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub nid_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub feature_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub geometry_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub attribute_encoding: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub texture_encoding: Option<Vec<String>>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub lod_type: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub lod_model: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub indexing_scheme: Option<String>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub default_texture_definition: Option<Vec<Texture>>,
    #[deprecated(note = "Deprecated in 1.7")]
    pub default_material_definition: Option<MaterialDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinition {
    #[deprecated(note = "MaterialDefinition was deprecated in 1.7")]
    pub identifier: MaterialDefinitionInfo,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitionInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub dtype: Option<String>,
    pub href: Option<String>,
    pub params: Option<MaterialParams>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialParams {
    pub render_mode: String,
    pub transparency: Option<i32>,
    pub reflectivity: Option<i32>,
    pub shininess: Option<i32>,
    pub ambient: Option<Vec<i32>>,
    pub diffuse: Option<Vec<i32>>,
    pub specular: Option<Vec<f32>>,
    pub cast_shadows: Option<bool>,
    pub receive_shadows: Option<bool>,
    pub cull_face: Option<String>,
    #[serde(default)]
    pub vertex_colors: bool,
    #[serde(default)]
    pub vertex_regions: bool,
    #[serde(default)]
    pub use_vertex_color_alpha: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    pub encoding: Option<Vec<String>>,
    pub wrap: Option<Vec<String>>,
    pub atlas: Option<bool>,
    pub uv_set: Option<String>,
    pub channels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    pub header: Vec<HeaderAttribute>,
    pub topology: String,
    pub ordering: Vec<String>,
    pub vertex_attributes: VertexAttribute,
    pub feature_attribute_order: Vec<String>,
    pub feature_attributes: FeatureAttribute,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureAttribute {
    pub id: Option<Value>,
    pub face_range: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryAttribute {
    pub value_type: String,
    pub values_per_element: i32,
    #[serde(default)]
    pub byte_offset: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeaderAttribute {
    pub property: String,
    #[serde(rename = "type")]
    pub dtype: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VertexAttribute {
    pub position: Option<GeometryAttribute>,
    pub uv0: Option<GeometryAttribute>,
    pub normal: Option<GeometryAttribute>,
    pub color: Option<GeometryAttribute>,
    pub region: Option<GeometryAttribute>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextureSetDefinition {
    pub formats: Vec<TextureSetDefinitionFormat>,
    pub atlas: Option<bool>,
}

impl TextureSetDefinition {
    pub fn has_compressed_textures(&self) -> bool {
        match self.formats.len() {
            1 => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextureSetDefinitionFormat {
    pub name: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}

impl io::ZipFileReader for NodePage {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub index: i32,
    pub obb: OBB,
    pub parent_index: Option<i32>,
    pub lod_threshold: Option<f32>,
    pub children: Option<Vec<i32>>,
    pub mesh: Option<Mesh>,
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OBB {
    pub center: [f32; 3],
    pub half_size: [f32; 3],
    pub quanternion: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mesh {
    pub material: Option<MeshMaterial>,
    pub geometry: Option<MeshGeometry>,
    pub attribute: Option<MeshAttribute>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshMaterial {
    definition: i32,
    resource: i32,
    texel_count_hint: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshGeometry {
    definition: i32,
    resource: i32,
    vertex_count: i32,
    feature_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeshAttribute {
    resource: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "I3SVersion")]
    pub i3s_version: String,
    pub node_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureData {
    pub id: i32,
    pub position: Vec<f64>,
    pub pivot_offset: [f64; 3],
    pub mbb: [f64; 6],
    pub layer: String,
    pub attributes: FeatureAttribute,
    pub geometries: Geometry,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub id: i32,
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub transformation: [f64; 16],
    pub params: GeometryParams,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GeometryParams {
    Reference(GeometryReferenceParams),
    Vested(VestedGeometryParams),
    SingleComponent(SingleComponentParams),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryReferenceParams {
    pub href: String,
    #[serde(rename = "type")]
    pub geometry_type: Option<String>,
    pub face_range: Option<Vec<i32>>,
    pub lod_geometry: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VestedGeometryParams {
    pub geometry_type: String,
    pub topology: String,
    pub vertex_attributes: VertexAttribute,
    pub faces: GeometryAttribute,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleComponentParams {
    pub id: i32,
    pub material: Option<String>,
    pub texture: Option<String>,
    pub material_id: Option<i32>,
    pub texture_id: Option<[i32; 1]>,
    pub region_id: Option<[i32; 1]>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub stats: AttributeStatistics,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStatistics {
    pub total_values_count: Option<i32>,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub min_time_str: Option<String>,
    pub max_time_str: Option<String>,
    pub count: Option<i32>,
    pub sum: Option<f32>,
    pub avg: Option<f32>,
    pub stddev: Option<f32>,
    pub variance: Option<f32>,
    pub histogram: Option<Histogram>,
    pub most_frequent_values: Option<Vec<ValueCount>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    minimum: f64,
    maximum: f64,
    counts: Vec<u16>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueCount {
    value: String,
    count: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeIndexDocument {}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedResource {}
