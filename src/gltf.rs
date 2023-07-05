use serde_json::Value;

use crate::Importer;

#[derive(Debug)]
pub struct Asset {
    pub version: String,
    pub copyright: Option<String>,
    pub generator: Option<String>,
    pub min_version: Option<String>
}

#[derive(Debug)]
pub struct Gltf {
    pub asset: Asset
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

        Ok(Gltf { asset })
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