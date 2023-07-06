use std::ffi::{c_char, CStr};

use crate::{VertexPositionColorTextureNormalTangent, Scene};

pub struct MdMesh {
    pub vertices:     *mut VertexPositionColorTextureNormalTangent,
    pub num_vertices: usize,

    pub indices:      *mut u32,
    pub num_indices:  usize,

    pub material:     u64
}

impl Drop for MdMesh {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.vertices, self.num_vertices, self.num_vertices);

            println!("Drop mesh");
        }
    }
}

pub struct MdScene {
    pub meshes:     *mut MdMesh,
    pub num_meshes: usize
}

impl Drop for MdScene {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.meshes, self.num_meshes, self.num_meshes);

            println!("Drop scene");
        }
    }
}

#[no_mangle]
pub unsafe extern fn mdLoad(path: *const c_char) -> *mut MdScene {
    let path = CStr::from_ptr(path).to_str().unwrap();
    let scene = Scene::load(path);

    let mut meshes = Vec::with_capacity(scene.meshes.len());
    
    for mut mesh in scene.meshes {
        let vertices = mesh.vertices.as_mut_ptr();
        let num_vertices = mesh.vertices.len();
        std::mem::forget(mesh.vertices);

        let indices = mesh.indices.as_mut_ptr();
        let num_indices = mesh.indices.len();
        std::mem::forget(mesh.indices);

        let material = mesh.material.unwrap_or_default();

        meshes.push(MdMesh {
            vertices,
            num_vertices,
            indices,
            num_indices,
            material,
        });
    }

    let mesh_ptr = meshes.as_mut_ptr();
    let num_meshes = meshes.len();
    std::mem::forget(meshes);

    let scene = MdScene {
        meshes: mesh_ptr,
        num_meshes,
    };

    Box::into_raw(Box::new(scene))
}