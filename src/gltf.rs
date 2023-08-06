use std::path::Path;

use serde_json::Value;

use crate::{Importer, Vec4, Vec3, Mat4, Quat, Vec2, Vertex, utils};

#[derive(Debug)]
pub struct Asset {
    pub version:     String,
    pub copyright:   Option<String>,
    pub generator:   Option<String>,
    pub min_version: Option<String>
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float
}

impl EnumConvert for ComponentType {
    fn from_u64(value: u64) -> Self {
        match value {
            5120 => Self::Byte,
            5121 => Self::UnsignedByte,
            5122 => Self::Short,
            5123 => Self::UnsignedShort,
            5125 => Self::UnsignedInt,
            5126 => Self::Float,

            ct => panic!("Unrecognized component type {ct}")
        }
    }
}

#[derive(Debug)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4
}

#[derive(Debug)]
pub struct Accessor {
    pub buffer_view:    Option<u64>,
    pub byte_offset:    u64,
    pub component_type: ComponentType,
    pub normalized:     bool,
    pub count:          u64,
    pub a_type:         AccessorType,
    pub max:            Option<Vec<f32>>,
    pub min:            Option<Vec<f32>>,
    // TODO: Sparse
    // pub sparse:      Option<AccessorSparse>
}

#[derive(Debug)]
pub struct Buffer {
    pub uri:         Option<String>,
    pub byte_length: u64
}

#[derive(Debug)]
pub enum BufferTarget {
    ArrayBuffer,
    ElementArrayBuffer
}

impl EnumConvert for BufferTarget {
    fn from_u64(value: u64) -> Self {
        match value {
            34962 => Self::ArrayBuffer,
            34963 => Self::ElementArrayBuffer,

            bt => panic!("Unrecognized buffer target {bt}.")
        }
    }
}

#[derive(Debug)]
pub struct BufferView {
    pub buffer:      u64,
    pub byte_offset: u64,
    pub byte_length: u64,
    pub byte_stride: Option<u64>,
    pub target:      Option<BufferTarget>
}

#[derive(Debug)]
pub struct Image {
    pub uri:         Option<String>,

    /// The spec says that the only officially supported mime types are
    /// `image/jpeg` and `image/png`.
    /// 
    /// However, modelo also supports `image/bmp` as well, although it
    /// will not export using this mime type as it is not in the spec.
    pub mime_type:   Option<String>,
    pub buffer_view: Option<u64>,
}

#[derive(Debug)]
pub struct TextureInfo {
    pub index:     u64,
    pub tex_coord: u64,
    pub scale:     Option<f32>
}

#[derive(Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor:          Vec4,
    pub base_color_texture:         Option<TextureInfo>,
    pub metallic_factor:            f32,
    pub roughness_factor:           f32,
    pub metallic_roughness_texture: Option<TextureInfo>
}

#[derive(Debug)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend
}

#[derive(Debug)]
pub struct Material {
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,
    pub normal_texture:         Option<TextureInfo>,
    pub occlusion_texture:      Option<TextureInfo>,
    pub emissive_texture:       Option<TextureInfo>,
    pub emissive_factor:        Vec3,
    pub alpha_mode:             AlphaMode,
    pub alpha_cutoff:           f32,
    pub double_sided:           bool
}

#[derive(Debug)]
pub enum PrimitiveTopology {
    Points,
    Lines,
    LineLoop,
    LineStrip,
    Triangles,
    TriangleStrip,
    TriangleFan
}

impl EnumConvert for PrimitiveTopology {
    fn from_u64(value: u64) -> Self {
        match value {
            0 => Self::Points,
            1 => Self::Lines,
            2 => Self::LineLoop,
            3 => Self::LineStrip,
            4 => Self::Triangles,
            5 => Self::TriangleStrip,
            6 => Self::TriangleFan,

            pt => panic!("Unrecognized primitive topology {pt}")
        }
    }
}

#[derive(Debug)]
pub struct MeshPrimitive {
    pub attributes: Vec<(String, u64)>,
    pub indices:    Option<u64>,
    pub material:   Option<u64>,
    pub mode:       PrimitiveTopology,
    // pub targets: 
}

#[derive(Debug)]
pub struct Mesh {
    pub primitives: Vec<MeshPrimitive>,
    pub weights:    Option<Vec<f32>>
}

#[derive(Debug)]
pub struct Node {
    pub camera:      Option<u64>,
    pub children:    Option<Vec<u64>>,
    pub skin:        Option<u64>,
    pub matrix:      Mat4,
    pub mesh:        Option<u64>,
    pub rotation:    Quat,
    pub scale:       Vec3,
    pub translation: Vec3,
    pub weights:     Option<Vec<f32>>
}

#[derive(Debug)]
pub enum TextureFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear
}

impl EnumConvert for TextureFilter {
    fn from_u64(value: u64) -> Self {
        match value {
            9728 => Self::Nearest,
            9729 => Self::Linear,
            9984 => Self::NearestMipmapNearest,
            9985 => Self::LinearMipmapNearest,
            9986 => Self::NearestMipmapLinear,
            9987 => Self::LinearMipmapLinear,

            tf => panic!("Unrecognized texture filter {tf}.")
        }
    }
}

#[derive(Debug)]
pub enum TextureWrapMode {
    ClampToEdge,
    MirroredRepeat,
    Repeat
}

impl EnumConvert for TextureWrapMode {
    fn from_u64(value: u64) -> Self {
        match value {
            33071 => Self::ClampToEdge,
            33648 => Self::MirroredRepeat,
            10497 => Self::Repeat,

            tm => panic!("Unrecognized texture wrap mode {tm}")
        }
    }
}

#[derive(Debug)]
pub struct Sampler {
    pub mag_filter: Option<TextureFilter>,
    pub min_filter: Option<TextureFilter>,
    pub wrap_s:     TextureWrapMode,
    pub wrap_t:     TextureWrapMode
}

#[derive(Debug)]
pub struct Scene {
    pub nodes: Option<Vec<u64>>
}

#[derive(Debug)]
pub struct Texture {
    pub sampler: Option<u64>,
    pub source:  Option<u64>
}

#[derive(Debug)]
pub struct Gltf {
    pub accessors:    Option<Vec<Accessor>>,
    pub asset:        Asset,
    pub buffers:      Option<Vec<Buffer>>,
    pub buffer_views: Option<Vec<BufferView>>,
    pub images:       Option<Vec<Image>>,
    pub materials:    Option<Vec<Material>>,
    pub meshes:       Option<Vec<Mesh>>,
    pub nodes:        Option<Vec<Node>>,
    pub samplers:     Option<Vec<Sampler>>,
    pub scene:        Option<u64>,
    pub scenes:       Option<Vec<Scene>>,
    pub textures:     Option<Vec<Texture>>,

    /// If the file is a GLB, this field should be filled.
    pub glb_data:     Option<Vec<u8>>

    // TODO: These
    //pub animations:   Option<Vec<Animation>>,
    //pub cameras:      Option<Vec<Camera>>,
    //pub skins:        Option<Vec<Skin>>
}

impl Importer for Gltf {
    fn import(path: &str) -> Result<Self, crate::ImportError> where Self : Sized {
        // TODO: GLB support.

        let json = match std::fs::read_to_string(path) {
            Ok(json) => json,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err(crate::ImportError::new(crate::ImportErrorType::FileNotFound, "The given file was not found."));
                } else {
                    return Err(crate::ImportError::new(crate::ImportErrorType::Other, err.to_string()));
                }
            },
        };

        let json = match serde_json::from_str::<Value>(&json) {
            Ok(gltf) => gltf,
            Err(err) => {
                return Err(crate::ImportError::new(crate::ImportErrorType::Other, "Parsing error occurred."));
            }
        };

        let asset = if let Some(asset) = json.get("asset") {
            let version = if let Some(version) = asset.get("version") {
                value_to_string(version)
            } else {
                return Err(crate::ImportError::new(crate::ImportErrorType::Other, "Missing version string."));
            };

            let copyright = to_string_or_none(asset.get("copyright"));
            let generator = to_string_or_none(asset.get("generator"));
            let min_version = to_string_or_none(asset.get("minVersion"));

            Asset {
                version,
                copyright,
                generator,
                min_version,
            }
        } else {
            return Err(crate::ImportError::new(crate::ImportErrorType::Other, "Missing \"Asset\" from glTF file."));
        };

        let accessors = if let Some(accessors) = json.get("accessors") {
            let accessors = accessors.as_array().unwrap();
            let mut acc_vec = Vec::with_capacity(accessors.len());

            for accessor in accessors {
                let buffer_view = to_u64_or_none(accessor.get("bufferView"));

                let byte_offset = to_u64_or_default(accessor.get("byteOffset"), 0);

                let component_type = ComponentType::from_u64(accessor.get("componentType").unwrap().as_u64().unwrap());

                let normalized = if let Some(norm) = accessor.get("normalized") {
                    norm.as_bool().unwrap()
                } else {
                    false
                };

                let count = accessor.get("count").unwrap().as_u64().unwrap();

                let a_type = accessor.get("type").unwrap();
                let a_type = match a_type.as_str().unwrap() {
                    "SCALAR" => AccessorType::Scalar,
                    "VEC2" => AccessorType::Vec2,
                    "VEC3" => AccessorType::Vec3,
                    "VEC4" => AccessorType::Vec4,
                    "MAT2" => AccessorType::Mat2,
                    "MAT3" => AccessorType::Mat3,
                    "MAT4" => AccessorType::Mat4,
                    at => panic!("Unrecognized acccessor type \"{at}\"")
                };

                let max = if let Some(max) = accessor.get("max") {
                    let max = max.as_array().unwrap();

                    Some(max
                        .iter()
                        .map(|value| value.as_f64().unwrap() as f32)
                        .collect())
                } else {
                    None
                };

                let min = if let Some(min) = accessor.get("min") {
                    let min = min.as_array().unwrap();

                    Some(min
                        .iter()
                        .map(|value| value.as_f64().unwrap() as f32)
                        .collect())
                } else {
                    None
                };

                acc_vec.push(Accessor {
                    buffer_view,
                    byte_offset,
                    component_type,
                    normalized,
                    count,
                    a_type,
                    max,
                    min,
                });
            }

            Some(acc_vec)
        } else {
            None
        };

        let buffers = if let Some(buffers) = json.get("buffers") {
            let buffers = buffers.as_array().unwrap();
            let mut buf_vec = Vec::with_capacity(buffers.len());

            for buffer in buffers {
                let uri = to_string_or_none(buffer.get("uri"));

                let byte_length = buffer.get("byteLength").unwrap().as_u64().unwrap();

                buf_vec.push(Buffer {
                    uri,
                    byte_length
                });
            }

            Some(buf_vec)
        } else {
            None
        };

        let buffer_views = if let Some(views) = json.get("bufferViews") {
            let views = views.as_array().unwrap();
            let mut view_vec = Vec::with_capacity(views.len());

            for view in views {
                let buffer = view.get("buffer").unwrap().as_u64().unwrap();

                let byte_offset = to_u64_or_default(view.get("byteOffset"), 0);

                let byte_length = view.get("byteLength").unwrap().as_u64().unwrap();

                let byte_stride = to_u64_or_none(view.get("byteStride"));

                let target = to_enum_or_none(view.get("target"));

                view_vec.push(BufferView {
                    buffer,
                    byte_offset,
                    byte_length,
                    byte_stride,
                    target
                });
            }

            Some(view_vec)
        } else {
            None
        };

        let images = if let Some(images) = json.get("images") {
            let images = images.as_array().unwrap();
            let mut img_vec = Vec::with_capacity(images.len());

            for image in images {
                let uri = to_string_or_none(image.get("uri"));

                let mime_type = to_string_or_none(image.get("mimeType"));

                let buffer_view = to_u64_or_none(image.get("bufferView"));

                img_vec.push(Image {
                    uri,
                    mime_type,
                    buffer_view,
                });
            }

            Some(img_vec)
        } else {
            None
        };

        let materials = if let Some(materials) = json.get("materials") {
            let materials = materials.as_array().unwrap();
            let mut mat_vec = Vec::with_capacity(materials.len());

            for material in materials {
                let pbr_metallic_roughness = if let Some(pmr) = material.get("pbrMetallicRoughness") {
                    let base_color_factor = if let Some(bcf) = pmr.get("baseColorFactor") {
                        Vec4 {
                            x: bcf[0].as_f64().unwrap() as f32,
                            y: bcf[1].as_f64().unwrap() as f32,
                            z: bcf[2].as_f64().unwrap() as f32,
                            w: bcf[3].as_f64().unwrap() as f32
                        }
                    } else {
                        Vec4 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                            w: 1.0
                        }
                    };

                    let base_color_texture = to_texture_info_or_none(pmr.get("baseColorTexture"));

                    let metallic_factor = to_f32_or_default(pmr.get("metallicFactor"), 1.0);
                    let roughness_factor = to_f32_or_default(pmr.get("roughnessFactor"), 1.0);

                    let metallic_roughness_texture = to_texture_info_or_none(pmr.get("metallicRoughnessTexture"));

                    Some(PbrMetallicRoughness {
                        base_color_factor,
                        base_color_texture,
                        metallic_factor,
                        roughness_factor,
                        metallic_roughness_texture,
                    })
                } else {
                    None
                };

                let normal_texture = to_texture_info_or_none(material.get("normalTexture"));

                let occlusion_texture = to_texture_info_or_none(material.get("occlusionTexture"));

                let emissive_texture = to_texture_info_or_none(material.get("emissiveTexture"));

                let emissive_factor = if let Some(ef) = material.get("emissiveFactor") {
                    Vec3 {
                        x: ef[0].as_f64().unwrap() as f32,
                        y: ef[1].as_f64().unwrap() as f32,
                        z: ef[2].as_f64().unwrap() as f32
                    }
                } else {
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                };

                let alpha_mode = if let Some(am) = material.get("alphaMode") {
                    match am.as_str().unwrap() {
                        "OPAQUE" => AlphaMode::Opaque,
                        "MASK" => AlphaMode::Mask,
                        "BLEND" => AlphaMode::Blend,

                        mode => panic!("Unrecognized alpha mode \"{mode}\"")
                    }
                } else {
                    AlphaMode::Opaque
                };

                let alpha_cutoff = to_f32_or_default(material.get("alphaCutoff"), 0.5);

                let double_sided = to_bool_or_default(material.get("doubleSided"), false);

                mat_vec.push(Material {
                    pbr_metallic_roughness,
                    normal_texture,
                    occlusion_texture,
                    emissive_texture,
                    emissive_factor,
                    alpha_mode,
                    alpha_cutoff,
                    double_sided,
                });
            }

            Some(mat_vec)
        } else {
            None
        };

        let meshes = if let Some(meshes) = json.get("meshes") {
            let meshes = meshes.as_array().unwrap();
            let mut mesh_vec = Vec::with_capacity(meshes.len());

            for mesh in meshes {
                let primitives = mesh.get("primitives").unwrap().as_array().unwrap();
                let mut prim_vec = Vec::with_capacity(primitives.len());

                for primitive in primitives {
                    let attributes = primitive.get("attributes").unwrap()
                        .as_object().unwrap()
                        .iter()
                        .map(|value| {
                            (value.0.clone(), value.1.as_u64().unwrap())
                        })
                        .collect();

                    let indices = to_u64_or_none(primitive.get("indices"));

                    let material = to_u64_or_none(primitive.get("material"));

                    let mode = to_enum_or_default(primitive.get("mode"), PrimitiveTopology::Triangles);

                    prim_vec.push(MeshPrimitive {
                        attributes,
                        indices,
                        material,
                        mode,
                    });
                }

                let weights = if let Some(weights) = mesh.get("weights") {
                    Some(weights
                        .as_array().unwrap()
                        .iter()
                        .map(|value| value.as_f64().unwrap() as f32)
                        .collect())
                } else {
                    None
                };

                mesh_vec.push(Mesh {
                    primitives: prim_vec,
                    weights,
                });
            }

            Some(mesh_vec)
        } else {
            None
        };

        let nodes = if let Some(nodes) = json.get("nodes") {
            let nodes = nodes.as_array().unwrap();
            let mut node_vec = Vec::with_capacity(nodes.len());

            for node in nodes {
                let camera = to_u64_or_none(node.get("camera"));

                let children = if let Some(children) = node.get("children") {
                    Some(children
                        .as_array().unwrap()
                        .iter()
                        .map(|value| value.as_u64().unwrap())
                        .collect())
                } else {
                    None
                };

                let skin = to_u64_or_none(node.get("skin"));

                let matrix = if let Some(matrix) = node.get("matrix") {
                    // OH GOD MAKE IT STOP
                    // glTF matrices are in column-major order, however modelo matrices are
                    // in row major order, so we do the conversion here. This explains the
                    // weird array indices.
                    Mat4 {
                        row0: Vec4 {
                            x: matrix[0].as_f64().unwrap() as f32,
                            y: matrix[4].as_f64().unwrap() as f32,
                            z: matrix[8].as_f64().unwrap() as f32,
                            w: matrix[12].as_f64().unwrap() as f32,
                        },
                        row1: Vec4 {
                            x: matrix[1].as_f64().unwrap() as f32,
                            y: matrix[5].as_f64().unwrap() as f32,
                            z: matrix[9].as_f64().unwrap() as f32,
                            w: matrix[13].as_f64().unwrap() as f32,
                        },
                        row2: Vec4 {
                            x: matrix[2].as_f64().unwrap() as f32,
                            y: matrix[6].as_f64().unwrap() as f32,
                            z: matrix[10].as_f64().unwrap() as f32,
                            w: matrix[14].as_f64().unwrap() as f32,
                        },
                        row3: Vec4 {
                            x: matrix[3].as_f64().unwrap() as f32,
                            y: matrix[7].as_f64().unwrap() as f32,
                            z: matrix[11].as_f64().unwrap() as f32,
                            w: matrix[15].as_f64().unwrap() as f32,
                        }
                    }
                } else {
                    Mat4::identity()
                };

                let mesh = to_u64_or_none(node.get("mesh"));

                let rotation = if let Some(rotation) = node.get("rotation") {
                    Quat {
                        x: rotation[0].as_f64().unwrap() as f32,
                        y: rotation[1].as_f64().unwrap() as f32,
                        z: rotation[2].as_f64().unwrap() as f32,
                        w: rotation[3].as_f64().unwrap() as f32,
                    }
                } else {
                    Quat::new(0.0, 0.0, 0.0, 1.0)
                };

                let scale = if let Some(scale) = node.get("scale") {
                    Vec3 {
                        x: scale[0].as_f64().unwrap() as f32,
                        y: scale[1].as_f64().unwrap() as f32,
                        z: scale[2].as_f64().unwrap() as f32,
                    }
                } else {
                    Vec3::new(1.0, 1.0, 1.0)
                };

                let translation = if let Some(translation) = node.get("translation") {
                    Vec3 {
                        x: translation[0].as_f64().unwrap() as f32,
                        y: translation[1].as_f64().unwrap() as f32,
                        z: translation[2].as_f64().unwrap() as f32,
                    }
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                };

                let weights = if let Some(weights) = node.get("weights") {
                    Some(weights
                        .as_array().unwrap()
                        .iter()
                        .map(|value| value.as_f64().unwrap() as f32)
                        .collect())
                } else {
                    None
                };

                node_vec.push(Node {
                    camera,
                    children,
                    skin,
                    matrix,
                    mesh,
                    rotation,
                    scale,
                    translation,
                    weights,
                });
            }

            Some(node_vec)
        } else {
            None
        };

        let samplers = if let Some(samplers) = json.get("samplers") {
            let samplers = samplers.as_array().unwrap();
            let mut samp_vec = Vec::with_capacity(samplers.len());

            for sampler in samplers {
                let mag_filter = to_enum_or_none(sampler.get("magFilter"));

                let min_filter = to_enum_or_none(sampler.get("minFilter"));

                let wrap_s = to_enum_or_default(sampler.get("wrapS"), TextureWrapMode::Repeat);

                let wrap_t = to_enum_or_default(sampler.get("wrapT"), TextureWrapMode::Repeat);

                samp_vec.push(Sampler {
                    mag_filter,
                    min_filter,
                    wrap_s,
                    wrap_t,
                });
            }

            Some(samp_vec)
        } else {
            None
        };

        let scene = to_u64_or_none(json.get("scene"));

        let scenes = if let Some(scenes) = json.get("scenes") {
            let scenes = scenes.as_array().unwrap();
            let mut scene_vec = Vec::with_capacity(scenes.len());

            for scene in scenes {
                let nodes = if let Some(nodes) = scene.get("nodes") {
                    Some(nodes
                        .as_array().unwrap()
                        .iter()
                        .map(|value| value.as_u64().unwrap())
                        .collect())
                } else {
                    None
                };

                scene_vec.push(Scene {
                    nodes,
                });
            }

            Some(scene_vec)
        } else {
            None
        };

        let textures = if let Some(textures) = json.get("textures") {
            let textures = textures.as_array().unwrap();
            let mut tex_vec = Vec::with_capacity(textures.len());

            for texture in textures {
                let sampler = to_u64_or_none(texture.get("sampler"));
                
                let source = to_u64_or_none(texture.get("source"));

                tex_vec.push(Texture {
                    sampler,
                    source,
                });
            }

            Some(tex_vec)
        } else {
            None
        };

        Ok(Gltf {
            accessors,
            asset,
            buffers,
            buffer_views,
            images,
            materials,
            meshes,
            nodes,
            samplers,
            scene,
            scenes,
            textures,


            glb_data: None
        })
    }

    fn export(path: &str) {
        todo!()
    }

    fn from_scene(scene: &crate::Scene) -> Self {
        todo!()
    }

    fn to_scene(&self, directory: &Path) -> crate::Scene {
        let gltf_buffers = self.buffers.as_ref().unwrap();
        let gltf_meshes = self.meshes.as_ref().unwrap();
        let gltf_accessors = self.accessors.as_ref().unwrap();
        let gltf_views = self.buffer_views.as_ref().unwrap();

        let gltf_materials = &self.materials;
        let gltf_images = &self.images;

        let buffers = {
            let mut bufs = Vec::new();

            for buffer in gltf_buffers {
                if let Some(uri) = &buffer.uri {
                    bufs.push(std::fs::read(directory.join(uri)).unwrap());
                }
            }

            bufs
        };

        let mut positions = Vec::new();
        let mut tex_coords = Vec::new();
        let mut normals = Vec::new();

        let mut meshes = Vec::new();

        for mesh in gltf_meshes {
            for primitive in &mesh.primitives {
                positions.clear();
                tex_coords.clear();
                normals.clear();

                for (name, index) in &primitive.attributes {
                    let accessor = &gltf_accessors[*index as usize];

                    let name = name.to_lowercase();
                    let name = name.as_str();

                    let view = &gltf_views[accessor.buffer_view.unwrap() as usize];

                    let buffer = &buffers[view.buffer as usize];

                    let start = view.byte_offset as usize + accessor.byte_offset as usize;
                    let end = start + view.byte_length as usize;

                    let buffer = &buffer[start..end];

                    let stride = match view.byte_stride {
                        Some(stride) => stride as usize,
                        None => 0
                    };

                    match name {
                        "position" => {
                            if stride != 0 && stride != std::mem::size_of::<Vec3>() {
                                todo!();
                            } else {
                                positions = unsafe { std::slice::from_raw_parts::<Vec3>(buffer.as_ptr() as *const _, buffer.len() / std::mem::size_of::<Vec3>()).to_vec() };
                            }
                        },

                        // TODO: Handle multiple texture coordinates.
                        "texcoord_0" => {
                            if stride != 0 && stride != std::mem::size_of::<Vec2>() {
                                todo!();
                            } else {
                                tex_coords = unsafe { std::slice::from_raw_parts::<Vec2>(buffer.as_ptr() as *const _, buffer.len() / std::mem::size_of::<Vec2>()).to_vec() };
                            }
                        },

                        "normal" => {
                            if stride != 0 && stride != std::mem::size_of::<Vec3>() {
                                todo!();
                            } else {
                                normals = unsafe { std::slice::from_raw_parts::<Vec3>(buffer.as_ptr() as *const _, buffer.len() / std::mem::size_of::<Vec3>()).to_vec() };
                            }
                        }

                        _ => {}
                    }
                }

                let mut vertices = Vec::new();

                // According to the glTF spec, we can rely on the fact that every Vec will be the same length:
                // "All attribute accessors for a given primitive MUST have the same count."
                for i in 0..positions.len() {
                    let normal = match normals.get(i) {
                        Some(normal) => *normal,
                        None => Vec3 { x: 0.0, y: 0.0, z: 0.0 }
                    };

                    let vertex = Vertex {
                        position: positions[i],
                        color: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
                        tex_coord: tex_coords[i],
                        normal,
                        tangent: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                    };

                    vertices.push(vertex);
                }

                let indices = if let Some(indices) = primitive.indices {
                    let accessor = &gltf_accessors[indices as usize];

                    let view = &gltf_views[accessor.buffer_view.unwrap() as usize];

                    let buffer = &buffers[view.buffer as usize];

                    let start = view.byte_offset as usize + accessor.byte_offset as usize;
                    let end = start + view.byte_length as usize;

                    let buffer = &buffer[start..end];

                    Some(unsafe {
                        match accessor.component_type {
                            ComponentType::Byte => {
                                let buffer = utils::reinterpret_cast_slice::<i8>(buffer);
                                utils::cast_slice_to_type::<i8, u32>(buffer)
                            },
                            ComponentType::UnsignedByte => {
                                utils::cast_slice_to_type::<u8, u32>(buffer)
                            },
                            ComponentType::Short => {
                                let buffer = utils::reinterpret_cast_slice::<i16>(buffer);
                                utils::cast_slice_to_type::<i16, u32>(buffer)
                            },
                            ComponentType::UnsignedShort => {
                                let buffer = utils::reinterpret_cast_slice::<u16>(buffer);
                                utils::cast_slice_to_type::<u16, u32>(buffer)
                            },
                            // If the component type is UnsignedInt, this is the fastest to load, as no conversion
                            // needs to be applied.
                            ComponentType::UnsignedInt => utils::reinterpret_cast_slice::<u32>(buffer).to_vec(),
                            ComponentType::Float => {
                                unimplemented!()
                            },
                        }
                    })
                } else {
                    None
                };

                let material = if let Some(material) = primitive.material {
                    Some(material as usize)
                } else {
                    None
                };

                meshes.push(crate::Mesh {
                    vertices,
                    indices,
                    material
                });
            }
        }

        let materials = if let Some(gltf_materials) = gltf_materials {
            let mut materials = Vec::with_capacity(gltf_materials.len());

            for material in gltf_materials {
                let pbr = match &material.pbr_metallic_roughness {
                    Some(pbr) => pbr,
                    None => &PbrMetallicRoughness {
                        base_color_factor: Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 },
                        base_color_texture: None,
                        metallic_factor: 1.0,
                        roughness_factor: 1.0,
                        metallic_roughness_texture: None,
                    }
                };

                let albedo_texture = if let Some(texture) = &pbr.base_color_texture {
                    Some(texture.index as usize)
                } else {
                    None
                };

                let metallic_roughness_texture = if let Some(texture) = &pbr.metallic_roughness_texture {
                    Some(texture.index as usize)
                } else {
                    None
                };

                let normal_texture = if let Some(texture) = &material.normal_texture {
                    Some(texture.index as usize)
                } else {
                    None
                };

                let occlusion_texture = if let Some(texture) = &material.occlusion_texture {
                    Some(texture.index as usize)
                } else {
                    None
                };

                let emissive_texture = if let Some(texture) = &material.emissive_texture {
                    Some(texture.index as usize)
                } else {
                    None
                };

                let alpha_mode = match material.alpha_mode {
                    AlphaMode::Opaque => crate::AlphaMode::Opaque,
                    AlphaMode::Mask => crate::AlphaMode::Cutoff,
                    AlphaMode::Blend => crate::AlphaMode::Blend,
                };

                materials.push(crate::Material {
                    albedo_color: pbr.base_color_factor,
                    albedo_texture,
                    normal_texture,
                    metallic: pbr.metallic_factor,
                    metallic_texture: metallic_roughness_texture,
                    roughness: pbr.roughness_factor,
                    roughness_texture: metallic_roughness_texture,
                    occlusion_texture,
                    emissive_texture,
                    alpha_mode,
                    alpha_cutoff: material.alpha_cutoff,
                    double_sided: material.double_sided,
                });
            }

            Some(materials)
        } else {
            None
        };

        let images = if let Some(gltf_images) = gltf_images {
            let mut images = Vec::with_capacity(gltf_images.len());

            for image in gltf_images {
                let path = if let Some(uri) = &image.uri {
                    // TODO: Handle cases where texture data is directly stored in the URI.
                    // These are base64 strings and start with `data:`

                    // ugghhh I hate that glTF can include %xy unicode strings
                    // TODO: Find a better way to handle these.
                    Some(uri.clone().replace("%20", " "))
                } else {
                    todo!()
                };

                images.push(crate::Image {
                    path,
                    data_type: None,
                    data: None
                });
            }

            Some(images)
        } else {
            None
        };

        crate::Scene {
            meshes,
            materials,
            images
        }
    }
}

pub trait EnumConvert {
    fn from_u64(value: u64) -> Self;
}

fn to_enum_or_none<T: EnumConvert>(option: Option<&Value>) -> Option<T> {
    if let Some(value) = option {
        Some(T::from_u64(value.as_u64().unwrap()))
    } else {
        None
    }
}

fn to_enum_or_default<T: EnumConvert>(option: Option<&Value>, default: T) -> T {
    if let Some(value) = option {
        T::from_u64(value.as_u64().unwrap())
    } else {
        default
    }
}

#[inline(always)]
fn value_to_string(value: &Value) -> String {
    String::from(value.as_str().unwrap())
}

#[inline(always)]
fn to_string_or_none(option: Option<&Value>) -> Option<String> {
    if let Some(value) = option {
        Some(value_to_string(value))
    } else {
        None
    }
}

#[inline(always)]
fn to_u64_or_none(option: Option<&Value>) -> Option<u64> {
    if let Some(value) = option {
        Some(value.as_u64().unwrap())
    } else {
        None
    }
}

#[inline(always)]
fn to_u64_or_default(option: Option<&Value>, default: u64) -> u64 {
    if let Some(value) = option {
        value.as_u64().unwrap()
    } else {
        default
    }
}

#[inline(always)]
fn to_f32_or_default(option: Option<&Value>, default: f32) -> f32 {
    if let Some(value) = option {
        value.as_f64().unwrap() as f32
    } else {
        default
    }
}

#[inline(always)]
fn to_bool_or_default(option: Option<&Value>, default: bool) -> bool {
    if let Some(value) = option {
        value.as_bool().unwrap()
    } else {
        default
    }
}

fn value_to_texture_info(value: &Value) -> TextureInfo {
    let index = value.get("index").unwrap().as_u64().unwrap();
    
    let tex_coord = to_u64_or_default(value.get("texCoord"), 0);

    let scale = if let Some(scale) = value.get("scale") {
        Some(scale.as_f64().unwrap() as f32)
    } else if let Some(strength) = value.get("strength") {
        Some(strength.as_f64().unwrap() as f32)
    } else {
        None
    };

    TextureInfo {
        index,
        tex_coord,
        scale
    }
}

#[inline(always)]
fn to_texture_info_or_none(option: Option<&Value>) -> Option<TextureInfo> {
    if let Some(value) = option {
        Some(value_to_texture_info(value))
    } else {
        None
    }
}