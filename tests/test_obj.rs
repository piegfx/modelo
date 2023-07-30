use modelo::{obj::Obj, Importer};

#[test]
fn load_from_file() {
    let obj = Obj::import("/home/skye/Documents/spaceboxlogo.obj").unwrap();

    println!("{obj:#?}");
}