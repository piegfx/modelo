use std::ffi::{c_char, CStr, CString};

use crate::{Vertex, Scene, Vec4, AlphaMode};

#[repr(C)]
pub struct MdMesh {
    pub vertices:     *mut Vertex,
    pub num_vertices: usize,

    pub indices:      *mut u32,
    pub num_indices:  usize,

    pub material:     usize
}

impl Drop for MdMesh {
    fn drop(&mut self) {
        unsafe {
            Vec::from_raw_parts(self.vertices, self.num_vertices, self.num_vertices);

            println!("Drop mesh");
        }
    }
}

#[repr(C)]
pub struct MdMaterial {
    pub albedo_color:      Vec4,
    pub albedo_texture:    usize,

    pub normal_texture:    usize,

    pub metallic:          f32,
    pub metallic_texture:  usize,

    pub roughness:         f32,
    pub roughness_texture: usize,

    pub occlusion_texture: usize,

    pub emissive_texture:  usize,

    pub alpha_mode:        AlphaMode,
    pub alpha_cutoff:      f32,

    pub double_sided:      bool
}

#[repr(C)]
pub struct MdImage {
    pub path:        *mut c_char,

    pub data_type:   crate::ImageDataType,
    pub data:        *mut u8,
    pub data_length: usize
}

impl Drop for MdImage {
    fn drop(&mut self) {
        unsafe {
            if self.path != std::ptr::null_mut() {
                drop(CString::from_raw(self.path));
            }

            if self.data != std::ptr::null_mut() {
                Vec::from_raw_parts(self.data, self.data_length, self.data_length);
            }
        }
    }
}

#[repr(C)]
pub struct MdScene {
    pub meshes:        *mut MdMesh,
    pub num_meshes:    usize,

    pub materials:     *mut MdMaterial,
    pub num_materials: usize,

    pub images:        *mut MdImage,
    pub num_images:    usize
}

impl Drop for MdScene {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping meshes.");
            Vec::from_raw_parts(self.meshes, self.num_meshes, self.num_meshes);

            println!("Dropping materials.");
            Vec::from_raw_parts(self.materials, self.num_meshes, self.num_meshes);

            println!("Dropping images.");
            Vec::from_raw_parts(self.images, self.num_images, self.num_images);

            println!("Dropped scene");
        }
    }
}

#[no_mangle]
pub unsafe extern fn mdLoad(path: *const c_char, scene: *mut *mut MdScene) {
    let path = CStr::from_ptr(path).to_str().unwrap();
    let scene_safe = Scene::load(path);

    let mut meshes = Vec::with_capacity(scene_safe.meshes.len());
    
    for mut mesh in scene_safe.meshes {
        let vertices = mesh.vertices.as_mut_ptr();
        let num_vertices = mesh.vertices.len();
        std::mem::forget(mesh.vertices);

        let (indices, num_indices) = if let Some(mut indices) = mesh.indices {
            let num_indices = indices.len();
            let indices_raw = indices.as_mut_ptr();

            std::mem::forget(indices);

            (indices_raw, num_indices)
        } else {
            (std::ptr::null_mut(), 0)
        };

        let material = mesh.material.unwrap_or(usize::MAX);

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

    let (material_ptr, num_materials) = if let Some(scene_materials) = scene_safe.materials {
        let mut materials = Vec::with_capacity(scene_materials.len());

        for material in scene_materials {
            materials.push(MdMaterial {
                albedo_color: material.albedo_color,
                albedo_texture: material.albedo_texture.unwrap_or(usize::MAX),
                normal_texture: material.normal_texture.unwrap_or(usize::MAX),
                metallic: material.metallic,
                metallic_texture: material.metallic_texture.unwrap_or(usize::MAX),
                roughness: material.roughness,
                roughness_texture: material.roughness_texture.unwrap_or(usize::MAX),
                occlusion_texture: material.occlusion_texture.unwrap_or(usize::MAX),
                emissive_texture: material.emissive_texture.unwrap_or(usize::MAX),
                alpha_mode: material.alpha_mode,
                alpha_cutoff: material.alpha_cutoff,
                double_sided: material.double_sided,
            });
        }

        let material_ptr = materials.as_mut_ptr();
        let num_materials = materials.len();
        std::mem::forget(materials);

        (material_ptr, num_materials)
    } else {
        (std::ptr::null_mut(), 0)
    };

    let (image_ptr, num_images) = if let Some(scene_images) = scene_safe.images {
        let mut images = Vec::with_capacity(scene_images.len());

        for image in scene_images {
            let path = if let Some(path) = image.path {
                let path = CString::new(path).unwrap();
                let path_ptr = path.as_ptr() as *mut i8;
                std::mem::forget(path);

                path_ptr
            } else {
                std::ptr::null_mut()
            };

            images.push(MdImage {
                path,
                data_type: image.data_type.unwrap_or(crate::ImageDataType::Unknown),
                data: std::ptr::null_mut(),
                data_length: 0,
            });
        }

        let image_ptr = images.as_mut_ptr();
        let num_images = images.len();
        std::mem::forget(images);

        (image_ptr, num_images)
    } else {
        (std::ptr::null_mut(), 0)
    };

    let scene_unsafe = MdScene {
        meshes: mesh_ptr,
        num_meshes,

        materials: material_ptr,
        num_materials,

        images: image_ptr,
        num_images
    };

    *scene = Box::into_raw(Box::new(scene_unsafe))
}

#[no_mangle]
pub unsafe extern fn mdFree(scene: *mut MdScene) {
    println!("start drop");
    drop(Box::from_raw(scene));
    println!("finish drop");
}