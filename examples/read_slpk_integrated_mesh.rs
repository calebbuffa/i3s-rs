use i3s::{IntegratedMesh, SceneLayerPackage};

const SLPK_PATH: &str = "";

fn main() {
    let mut slpk = SceneLayerPackage::new(SLPK_PATH).unwrap();
    let integrated_mesh = IntegratedMesh::from_slpk(&mut slpk).unwrap();
    let root_node = integrated_mesh.root_node();
    let children = root_node.children(&integrated_mesh.nodes);
    for child in children {
        println!("{:?}", child);
    }
}
