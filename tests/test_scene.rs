use modelo::Scene;

#[test]
fn test_scene() {
    //let scene = Scene::load("/home/skye/Downloads/Cubebs/IMyDefaultCube2GLTFseparate.gltf");
    let scene = Scene::load("/home/skye/Downloads/Fox.gltf");
    println!("{scene:#?}");

    //println!("{}", scene.meshes[0].indices.as_ref().unwrap().len());
}