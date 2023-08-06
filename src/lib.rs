use std::{path::Path, collections::HashMap};

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

pub mod load_flags {
    pub const NONE:             u32 = 0;
    pub const GENERATE_INDICES: u32 = 1 << 0;
    pub const GENERATE_NORMALS: u32 = 1 << 1;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position:  Vec3,
    pub tex_coord: Vec2,
    pub color:     Vec4,
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
    pub fn load(path: &str, flags: u32) -> Self {
        let directory = Path::new(path).parent().unwrap();

        let gltf = Gltf::import(path).unwrap();

        let mut scene = gltf.to_scene(directory);

        // Generates indices if they are not present, and deduplicates them while it's at it.
        if (flags & load_flags::GENERATE_INDICES) != 0 {
            // Stores a list of all vertices, of type HashableVertex as floats can't be easily hashed.
            let mut vertex_cache = HashMap::new();

            for mut mesh in &mut scene.meshes {
                if mesh.indices.is_some() {
                    continue;
                }

                vertex_cache.clear();

                // I'm not a fan of all the allocation but it's the only way I can see of doing it.
                let mut vertices = Vec::new();
                let mut indices = Vec::with_capacity(mesh.vertices.len());

                let mut id = 0u32;

                for vertex in &mesh.vertices {
                    let h_vertex: HashableVertex = unsafe { std::mem::transmute(*vertex) };

                    // If there is a duplicate, add it to the indices list.
                    if let Some(index) = vertex_cache.get(&h_vertex) {
                        indices.push(*index);
                    } else {
                        vertex_cache.insert(h_vertex, id);
                        vertices.push(*vertex);
                        indices.push(id);

                        id += 1;
                    }
                }

                mesh.vertices = vertices;
                mesh.indices = Some(indices);
            }
        }

        // TODO: I'm not 100% sure this is entirely working correctly. Some of the normals look a bit off.
        if (flags & load_flags::GENERATE_NORMALS) != 0 {
            for mesh in &mut scene.meshes {
                let indices = mesh.indices.as_ref();
                let vertices = &mut mesh.vertices;

                // As normals *should* always have a magnitude of precisely 1, we just check
                // to see if the magnitude is greater than 0.5.
                // If no normals have been generated, the magnitude will be 0/NaN, so we generate them.
                // Otherwise, don't bother.
                // TODO: This does need testing to make sure it works properly.
                if vertices[0].normal.magnitude() > 0.5 {
                    continue;
                }

                println!("Generating normals for mesh!");

                if let Some(indices) = indices {
                    for i in (0..indices.len()).step_by(3) {
                        let i1 = indices[i + 0];
                        let i2 = indices[i + 1];
                        let i3 = indices[i + 2];

                        let v1 = &vertices[i1 as usize];
                        let v2 = &vertices[i2 as usize];
                        let v3 = &vertices[i3 as usize];

                        let e1 = v1.position - v2.position;
                        let e2 = v3.position - v2.position;
                        let mut no = Vec3::cross(&e1, &e2);

                        no.x = -no.x;
                        no.y = -no.y;
                        no.z = -no.z;

                        vertices[i1 as usize].normal += no;
                        vertices[i2 as usize].normal += no;
                        vertices[i3 as usize].normal += no;
                    }
                } else {
                    // TODO: For some reason this only generates flat-shaded normals, instead of smooth-shaded.
                    for i in (0..vertices.len()).step_by(3) {
                        let v1 = &vertices[i + 0];
                        let v2 = &vertices[i + 1];
                        let v3 = &vertices[i + 2];

                        let e1 = v1.position - v2.position;
                        let e2 = v3.position - v2.position;
                        let mut no = Vec3::cross(&e1, &e2);

                        no.x = -no.x;
                        no.y = -no.y;
                        no.z = -no.z;

                        vertices[i + 0].normal += no;
                        vertices[i + 1].normal += no;
                        vertices[i + 2].normal += no;
                    }
                }

                for vertex in vertices {
                    vertex.normal.normalize();
                }
            }
        }

        scene
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

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn dot(&self, vec: &Self) -> f32 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Self) -> Self {
        Self {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(C)]
struct HashableVec2 {
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(C)]
struct HashableVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(C)]
struct HashableVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(C)]
struct HashableVertex {
    pub position:  HashableVec3,
    pub color:     HashableVec4,
    pub tex_coord: HashableVec2,
    pub normal:    HashableVec3,
    pub tangent:   HashableVec3
}