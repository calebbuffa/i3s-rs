mod common;
mod stream;
mod i3s;

use common::{Node, NodePage, SceneLayerInfo};
use stream::ZipFileReader;
use std::fs::File;

const SLPK_PATH: &str = r"path";

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
        let mut scene_layer_info_zip_file = zip_archive.by_name("3dSceneLayer.json.gz").unwrap();
        let scene_layer_info = SceneLayerInfo::from_zip(&mut scene_layer_info_zip_file).unwrap();
        println!("{:?}", scene_layer_info.store.default_material_definition);
    }
    {
        let node_page_paths = find_node_page_paths(&mut zip_archive);
        let mut nodes: Vec<Node> = vec![];
        for node_page_path in node_page_paths {
            let mut file = zip_archive.by_name(&node_page_path).unwrap();
            let node_page = NodePage::from_zip(&mut file).unwrap();
            nodes.extend(node_page.nodes);
        }
        let root_node = nodes.iter().find(|node| node.is_root()).unwrap();
        println!("{:?}", root_node);
    }
}
