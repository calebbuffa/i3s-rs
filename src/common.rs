use crate::stream;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneLayerInfo {
    pub id: i32,
    pub layer_type: String,
    pub version: String,
    pub capabilities: Vec<String>, // array of strings
    pub store: Store,
    pub geometry_definitions: Vec<GeometryDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_extent: Option<FullExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_reference: Option<SpatialReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_time_stamp: Option<ServiceUpdateTimeStamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_model_info: Option<HeightModelInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawing_info: Option<DrawingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_pages: Option<NodePageDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_definitions: Option<Vec<MaterialDefinitions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_set_definitions: Option<Vec<TextureSetDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_drawing_info: Option<CachedDrawingInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "disablePopup")]
    pub disable_pop_up: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_storage_info: Option<Vec<AttributeStorageInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_info: Option<Vec<StatisticsInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_factor: Option<f32>,
    #[serde(rename = "popupInfo", skip_serializing_if = "Option::is_none")]
    pub pop_up_info: Option<PopUpInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation_info: Option<ElevationInfo>,
}

impl stream::ZipFileReader for SceneLayerInfo {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    #[serde(rename = "type")]
    pub domain_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coded_values: Option<Vec<DomainCodedValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCodedValue {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopUpInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_infos: Option<Vec<Option<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_infos: Option<Vec<FieldInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popup_elements: Option<Vec<PopupElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_infos: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopupElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub popup_element_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_infos: Option<Vec<FieldInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsInfo {
    pub key: String,
    pub name: String,
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeStorageInfo {
    pub key: String,
    pub name: String,
    pub header: Vec<HeaderValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_byte_counts: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ids: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {
    pub value_type: String,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_per_element: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedDrawingInfo {
    pub color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawingInfo {
    pub renderer: Renderer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_symbols: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderer {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_layers: Option<Vec<SymbolLayer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLayer {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub symbol_layer_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullExtent {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_reference: Option<SpatialReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_vcs_wkid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_wkid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs_wkid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wkid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wkt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDefinition {
    pub geometry_buffers: Vec<GeometryBuffer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryBuffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<GeometryBufferMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv0: Option<GeometryBufferMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<GeometryBufferMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_range: Option<GeometryBufferMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compressed_attributes: Option<CompressedAttributes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryBufferMetadata {
    #[serde(rename = "type")]
    pub dtype: String,
    pub component: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeightModelInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_model: Option<String>,
    #[serde(rename = "vertCRS", skip_serializing_if = "Option::is_none")]
    pub vert_crs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialDefinitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<MaterialTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<MaterialTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<MaterialTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<[f32; 3]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cull_face: Option<String>, // 'none', 'front', 'back'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_factor: Option<[f32; 4]>, // default is [1.0, 1.0, 1.0, 1.0]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<MaterialTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_factor: Option<f32>,  // default is 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughness_factor: Option<f32>, // default is 1.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<MaterialTexture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialTexture {
    pub texture_set_definition_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<i32>, // default is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<f32>,    // default is 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePageDefinition {
    pub nodes_per_page: i32,
    pub lod_selection_metric_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_index: Option<i32>, // default is 0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpdateTimeStamp {
    pub last_update: i64, // Unix epoch from 1 January 1970 in milliseconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    pub profile: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pattern: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extent: Option<Vec<f64>>,
    #[serde(rename = "indexCRS", skip_serializing_if = "Option::is_none")]
    pub index_crs: Option<String>,
    #[serde(rename = "vertexCRS", skip_serializing_if = "Option::is_none")]
    pub vertex_crs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_reference_frame: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_geometry_schema: Option<DefaultGeometrySchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nid_encoding: Option<String>,          // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_encoding: Option<String>,      // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_encoding: Option<String>,     // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_encoding: Option<String>,    // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture_encoding: Option<Vec<String>>, // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_type: Option<String>,              // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_model: Option<String>,             // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_scheme: Option<String>,       // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_texture_definition: Option<Vec<Texture>>, // deprecated in 1.7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_material_definition: Option<MaterialDefinition>, // deprecated in 1.7
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// deprecated in 1.7
pub struct MaterialDefinition {
    pub identifier: MaterialDefinitionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// deprecated in 1.7
pub struct MaterialDefinitionInfo {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub dtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<MaterialParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialParams {
    pub render_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflectivity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shininess: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffuse: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specular: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_shadows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_shadows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cull_face: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_colors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertex_regions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_vertex_color_alpha: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atlas: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_set: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeometrySchema {
    pub header: Vec<HeaderAttribute>,
    pub topology: String,
    pub ordering: Vec<String>,
    pub vertex_attributes: VertexAttribute,
    pub feature_attribute_order: Vec<String>,
    pub feature_attributes: FeatureAttribute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_range: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeometryAttribute {
    pub value_type: String,
    pub values_per_element: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderAttribute {
    pub property: String,
    #[serde(rename = "type")]
    pub dtype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexAttribute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<GeometryAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv0: Option<GeometryAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<GeometryAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<GeometryAttribute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<GeometryAttribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureSetDefinition {
    pub formats: Vec<TextureSetDefinitionFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextureSetDefinitionFormat {
    pub name: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePage {
    pub nodes: Vec<Node>,
}

impl stream::ZipFileReader for NodePage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub index: i32,
    pub obb: OBB,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lod_threshold: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<Mesh>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncompressed_geometry_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_geometry_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncompressed_textures_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_textures_path: Option<String>,
}

impl Node {
    pub fn set_level(&mut self, level: i32) {
        self.level = Some(level);
    }

    pub fn get_level(&self) -> Option<i32> {
        self.level
    }

    pub fn is_root(&self) -> bool {
        self.parent_index.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OBB {
    pub center: [f32; 3],
    pub half_size: [f32; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quanternion: Option<[f32; 4]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<MeshMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<MeshGeometry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<MeshAttribute>,
}

impl Mesh {
    pub fn has_material(&self) -> bool {
        !self.material.is_none()
    }

    pub fn has_geometry(&self) -> bool {
        !self.geometry.is_none()
    }

    pub fn has_attribute(&self) -> bool {
        !self.attribute.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshMaterial {
    definition: i32,
    resource: i32,
    texel_count_hint: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshGeometry {
    definition: i32,
    resource: i32,
    vertex_count: i32,
    feature_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAttribute {
    resource: i32,
}
