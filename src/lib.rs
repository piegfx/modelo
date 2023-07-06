use gltf::Gltf;

pub mod gltf;

pub mod native;

#[derive(Debug)]
pub enum ImportErrorType {
    FileNotFound,
    StringParseError,

    Other
}

#[derive(Debug)]
pub struct ImportError {
    pub e_type: ImportErrorType,
    pub message: String
}

impl ImportError {
    pub fn new<T: ToString>(e_type: ImportErrorType, message: T) -> Self {
        Self {
            e_type,
            message: message.to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexPositionColorTextureNormalTangent {
    pub position:  Vec3,
    pub color:     Vec4,
    pub tex_coord: Vec2,
    pub normal:    Vec3,
    pub tangent:   Vec3
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<VertexPositionColorTextureNormalTangent>,
    pub indices:  Vec<u32>,
    pub material: Option<u64>
}

#[derive(Debug)]
pub struct Scene {
    pub meshes: Vec<Mesh>
}

impl Scene {
    pub fn load(path: &str) -> Self {
        let gltf = Gltf::import(path).unwrap();

        gltf.to_scene()
    }
}

pub trait Importer {
    fn import(path: &str) -> Result<Self, ImportError> where Self : Sized;

    fn export(path: &str);

    fn from_scene(scene: &Scene) -> Self;

    fn to_scene(&self) -> Scene;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x,
            y,
            z,
            w
        }
    }
}

pub type Quat = Vec4;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Mat4 {
    pub row0: Vec4,
    pub row1: Vec4,
    pub row2: Vec4,
    pub row3: Vec4
}

impl Mat4 {
    pub fn new(row0: Vec4, row1: Vec4, row2: Vec4, row3: Vec4) -> Self {
        Self {
            row0,
            row1,
            row2,
            row3,
        }
    }

    pub fn identity() -> Self {
        Self {
            row0: Vec4::new(1.0, 0.0, 0.0, 0.0),
            row1: Vec4::new(0.0, 1.0, 0.0, 0.0),
            row2: Vec4::new(0.0, 0.0, 1.0, 0.0),
            row3: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}