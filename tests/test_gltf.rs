use modelo::{gltf::Gltf, Importer};

#[test]
fn load_from_file() {
    let gltf = Gltf::from_path("/home/skye/Downloads/ionthrusterconcept01.gltf").unwrap();
    //let gltf = Gltf::from_path("/home/skye/Downloads/Fox.gltf").unwrap();

    println!("{:#?}", gltf);
}