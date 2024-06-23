use tokio;
use url::Url;
use i3s::{IntegratedMesh, Rest};

// url must end with SceneServer/
const BASE_URL: &str = "https://tiles.arcgis.com/tiles/z2tnIkrLQ2BRzr6P/arcgis/rest/services/Rancho_Mesh_v18/SceneServer/";

#[tokio::main]
async fn main() {
    let mut stream  = Rest::new(Url::parse(BASE_URL).unwrap());
    let integrated_mesh = IntegratedMesh::from_rest(&mut stream).await.unwrap();
    let root_node = integrated_mesh.root_node();
    let children = root_node.children(&integrated_mesh.nodes);
    for child in children {
        println!("{:?}", child);
    }
}