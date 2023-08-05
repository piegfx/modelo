use std::path::Path;

use gltf::Gltf;

pub mod utils;
pub mod gltf;
pub mod obj;

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
pub struct Vertex {
    pub position:  Vec3,
    pub color:     Vec4,
    pub tex_coord: Vec2,
    pub normal:    Vec3,
    pub tangent:   Vec3
}

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices:  Option<Vec<u32>>,
    pub material: Option<usize>
}

#[derive(Debug)]
#[repr(C)]
pub enum ImageDataType {
    Unknown,
    Png,
    Jpg,
    Bmp,
    Dds
}

#[derive(Debug)]
#[repr(C)]
pub enum AlphaMode {
    Opaque,
    Cutoff,
    Blend
}

#[derive(Debug)]
pub struct Material {
    pub albedo_color:      Vec4,
    pub albedo_texture:    Option<usize>,

    pub normal_texture:    Option<usize>,

    pub metallic:          f32,
    pub metallic_texture:  Option<usize>,

    pub roughness:         f32,
    pub roughness_texture: Option<usize>,

    pub occlusion_texture: Option<usize>,

    pub emissive_texture:  Option<usize>,

    pub alpha_mode:        AlphaMode,
    pub alpha_cutoff:      f32,

    pub double_sided:      bool
}

#[derive(Debug)]
pub struct Image {
    pub path:      Option<String>,

    pub data_type: Option<ImageDataType>,
    pub data:      Option<Vec<u8>>
}

#[derive(Debug)]
pub struct Scene {
    pub meshes:    Vec<Mesh>,
    pub materials: Option<Vec<Material>>,
    pub images:    Option<Vec<Image>>
}

impl Scene {
    pub fn load(path: &str) -> Self {
        let directory = Path::new(path).parent().unwrap();

        let gltf = Gltf::import(path).unwrap();

        gltf.to_scene(directory)
    }
}

pub trait Importer {
    fn import(path: &str) -> Result<Self, ImportError> where Self : Sized;

    fn export(path: &str);

    fn from_scene(scene: &Scene) -> Self;

    fn to_scene(&self, directory: &Path) -> Scene;
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