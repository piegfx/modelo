use crate::{Importer, Vec3, Vec2};

#[derive(Debug)]
pub struct FaceElement {
    pub vertex:    usize,
    pub tex_coord: usize,
    pub normal:    usize
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices:      Vec<Vec3>,
    pub normals:       Option<Vec<Vec3>>,
    pub tex_coords:    Option<Vec<Vec2>>,
    pub face_elements: Vec<FaceElement>,

    pub material:      Option<usize>
}

#[derive(Debug)]
pub struct Obj {
    pub meshes: Vec<Mesh>,
    //pub materials: Option<Vec<Material>>
}

impl Importer for Obj {
    fn import(path: &str) -> Result<Self, crate::ImportError> where Self : Sized {
        let text = std::fs::read_to_string(path).unwrap();

        /*let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut face_elements = Vec::new();

        let mut meshes = Vec::new();*/

        

        todo!()
    }

    fn export(path: &str) {
        todo!()
    }

    fn from_scene(scene: &crate::Scene) -> Self {
        todo!()
    }

    fn to_scene(&self) -> crate::Scene {
        todo!()
    }
}