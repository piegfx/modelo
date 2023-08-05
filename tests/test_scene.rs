use modelo::Scene;

#[test]
fn test_scene() {
    let scene = Scene::load("/home/skye/Downloads/Cubebs/IMyDefaultCube2GLTFseparate.gltf");
    println!("{scene:#?}");
}