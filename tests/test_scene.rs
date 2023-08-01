use modelo::Scene;

#[test]
fn test_scene() {
    let scene = Scene::load("/home/skye/Documents/SpaceBox/Models/OldSB/Functional/IonThruster.gltf");
    println!("{scene:#?}");
}