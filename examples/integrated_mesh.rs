use i3s_rust::i3s::IntegratedMesh;
use std::fs::File;
use zip;

const SLPK_PATH: &str = r"";

fn main() {
    let file = File::open(SLPK_PATH).unwrap();
    let mut zip_archive = zip::ZipArchive::new(file).unwrap();
    let integrated_mesh = IntegratedMesh::from_slpk(&mut zip_archive);
    println!("{:?}", integrated_mesh.scene_layer_info.texture_set_definitions.unwrap()[0].formats);
}
