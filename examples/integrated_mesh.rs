use i3s_rust::cmn::{Metadata, Node, NodePage, SceneLayerInfo};
use i3s_rust::io::ZipFileReader;
use std::{fs::File, io::Read};
use zip;

const SLPK_PATH: &str = r"C:\Users\cal11713\data\Catalina\DJI\Mesh\slpk\mesh_projected2.slpk";

fn find_node_page_paths(zip_archive: &mut zip::ZipArchive<File>) -> Vec<String> {
    zip_archive
        .file_names()
        .filter(|name| name.contains("nodepages"))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

fn main() {
    let file = File::open(SLPK_PATH).unwrap();
    let mut zip_archive = zip::ZipArchive::new(file).unwrap();
    {
        let mut scene_layer_zip_file = zip_archive.by_name("3dSceneLayer.json.gz").unwrap();
        let scene_layer = SceneLayerInfo::from_zip(&mut scene_layer_zip_file).unwrap();
        println!("{:?}\n", scene_layer);
    }
    {
        let mut metadata_zip_file = zip_archive.by_name("metadata.json").unwrap();
        let mut metadata_string = String::new();
        metadata_zip_file
            .read_to_string(&mut metadata_string)
            .unwrap();
        let metadata: Metadata = serde_json::from_str(&metadata_string).unwrap();
        println!("{:?}\n", metadata);
    }
    {
        let node_page_paths = find_node_page_paths(&mut zip_archive);
        let mut nodes: Vec<Node> = vec![];
        for node_page_path in node_page_paths {
            let mut file = zip_archive.by_name(&node_page_path).unwrap();
            let node_page = NodePage::from_zip(&mut file).unwrap();
            // println!("{:?}\n", node_page);
            nodes.extend(node_page.nodes);
        }
    }
}
