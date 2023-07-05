pub mod gltf;

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

pub trait Importer {
    fn from_path(path: &str) -> Result<Self, ImportError> where Self : Sized;
}