use thiserror::Error;

#[derive(Error, Debug)]
pub enum DCKVError {
    #[error("Invalid VR.")]
    InvalidVR,

    #[error("Invalid SQ tag.")]
    InvalidSQTag,

    #[error("Invalid SQ item length.")]
    InvalidSQItemLength,
    
    #[error("Unsupported VR [{0}].")]
    UnsupportedVR(String),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
