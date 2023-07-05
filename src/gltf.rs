use serde_json::Value;

use crate::Importer;

#[derive(Debug)]
pub struct Asset {
    pub version:     String,
    pub copyright:   Option<String>,
    pub generator:   Option<String>,
    pub min_version: Option<String>
}

#[derive(Debug)]
pub enum ComponentType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    UnsignedInt,
    Float
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
pub struct Gltf {
    pub accessors:    Option<Vec<Accessor>>,
    pub asset:        Asset,
    pub buffers:      Option<Vec<Buffer>>,
    pub buffer_views: Option<Vec<BufferView>>,
    pub images:       Option<Vec<Image>>
}

impl Importer for Gltf {
    fn from_path(path: &str) -> Result<Self, crate::ImportError> where Self : Sized {
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

                let component_type = accessor.get("componentType").unwrap();
                let component_type = match component_type.as_u64().unwrap() {
                    5120 => ComponentType::Byte,
                    5121 => ComponentType::UnsignedByte,
                    5122 => ComponentType::Short,
                    5123 => ComponentType::UnsignedShort,
                    5125 => ComponentType::UnsignedInt,
                    5126 => ComponentType::Float,
                    ct => panic!("Unsupported component type {ct}")
                };

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
                    byte_length,
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

                let target = if let Some(target) = view.get("target") {
                    Some(match target.as_u64().unwrap() {
                        34962 => BufferTarget::ArrayBuffer,
                        34963 => BufferTarget::ElementArrayBuffer,

                        tg => panic!("Unrecognized target {tg}")
                    })
                } else {
                    None
                };

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

        Ok(Gltf {
            asset,
            accessors,
            buffers,
            buffer_views,
            images
        })
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