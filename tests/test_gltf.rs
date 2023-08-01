use modelo::{gltf::Gltf, Importer};

#[test]
fn load_from_file() {
    let gltf = Gltf::import("/home/skye/Documents/SpaceBox/Models/OldSB/Functional/IonThruster.gltf").unwrap();
    //let gltf = Gltf::from_path("/home/skye/Downloads/Fox.gltf").unwrap();

    println!("{:#?}", gltf);
}