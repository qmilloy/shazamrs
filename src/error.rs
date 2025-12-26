use thiserror::Error;

/// Errors that may occur while recognizing audio.
#[derive(Debug, Error)]
pub enum ShazamError {
    /// Failure during audio fingerprint generation.
    #[error("signature generation failed: {0}")]
    Signature(#[from] shazamrs_core::SignatureError),

    /// HTTP or network error.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// The Shazam backend returned an unexpected response.
    #[error("invalid or unexpected response from Shazam backend")]
    InvalidResponse,
}
