// src/lib.rs
//
// Main entry point for the OpenMenuStandard library

// Re-export all public items
pub use crate::types::*;
pub use crate::document::*;
pub use crate::validation::*;
pub use crate::url::*;
pub use crate::utils::*;
pub use crate::builder::*;


#[cfg(feature = "tap-to-order")]
pub use crate::tap_to_order::*;

// Module declarations
mod types;
mod document;
mod validation;
mod url;
mod utils;
mod builder;

#[cfg(feature = "tap-to-order")]
mod tap_to_order;

/// Current version of the OpenMenuStandard
pub const OMS_VERSION: &str = "1.0";

/// OpenMenuStandard MIME type for HTTP transmission
pub const OMS_MIME_TYPE: &str = "application/vnd.openmenu+json";

/// OpenMenuStandard file extension
pub const OMS_FILE_EXTENSION: &str = "omenu";

/// OpenMenuStandard URL scheme
pub const OMS_URL_SCHEME: &str = "omenu://";

/// General error type for the OMS library
#[derive(Debug, thiserror::Error)]
pub enum OmsError {
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    
    #[error("Invalid customization type: {0}")]
    InvalidCustomizationType(String),
    
    #[error("Invalid vendor type: {0}")]
    InvalidVendorType(String),
    
    #[error("URL error: {0}")]
    UrlError(String),
    
    #[error("Invalid OMS URL: {0}")]
    InvalidOmsUrl(String),
    
    #[error("Missing required field: {0}")]
    MissingRequiredField(String),
    
    #[error("Invalid field value: {0}")]
    InvalidFieldValue(String),
    
    #[cfg(feature = "network")]
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for OpenMenuStandard operations
pub type OmsResult<T> = Result<T, OmsError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert_eq!(OMS_VERSION, "1.0");
    }
}