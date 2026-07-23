use crate::constants::{DEFAULT_BASE_URL, get_random_user_agent};
use reqwest::Client;
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HeaderMap, HeaderName, HeaderValue, USER_AGENT,
};
use shazamrs_core::Recognizer;

/// High-level Shazam client.
///
/// This struct provides async methods to identify audio
/// from files or raw audio bytes.
///
/// # Examples
///
/// ```no_run
/// use shazamrs::{Shazam, ShazamError};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), ShazamError> {
/// let shazam = Shazam::new();
///
/// let result = shazam.recognize_path("song.mp3").await?;
/// if let Some(track) = result.track {
///     println!("{} — {}", track.title.unwrap(), track.subtitle.unwrap());
/// }
/// # Ok(())
/// # }
/// ```
pub struct Shazam {
    pub(crate) recognizer: Recognizer,
    pub(crate) client: Client,
    /// Base URL for Shazam's discovery API. Always [`DEFAULT_BASE_URL`]
    /// outside of tests; overridable internally so tests can point
    /// requests at a mock server instead of the real Shazam backend.
    pub(crate) base_url: String,
}

impl Shazam {
    /// Create a new `Shazam` client using the default
    /// segment duration (10 seconds).
    pub fn new() -> Self {
        Self {
            recognizer: Recognizer::new(None),
            client: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Create a `Shazam` client with a custom
    /// audio segment duration.
    ///
    /// Larger values may improve recognition accuracy
    /// at the cost of processing time.
    pub fn with_segment_duration(seconds: u32) -> Self {
        Self {
            recognizer: Recognizer::new(Some(seconds)),
            client: Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Build the HTTP headers required by Shazam's discovery API.
    ///
    /// Mimics an iPhone client: sets `x-shazam-platform`,
    /// `x-shazam-appversion`, and a randomly-chosen `User-Agent` (see
    /// [`crate::constants`]) alongside standard `Accept*` headers.
    pub fn generate_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-shazam-platform"),
            HeaderValue::from_static("IPHONE"),
        );
        headers.insert(
            HeaderName::from_static("x-shazam-appversion"),
            HeaderValue::from_static("14.1.0"),
        );
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(get_random_user_agent()),
        );
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::USER_AGENTS;

    #[test]
    fn generate_headers_sets_expected_values() {
        let headers = Shazam::generate_headers();

        assert_eq!(headers.get("x-shazam-platform").unwrap(), "IPHONE");
        assert_eq!(headers.get("x-shazam-appversion").unwrap(), "14.1.0");
        assert_eq!(headers.get(ACCEPT).unwrap(), "*/*");
        assert_eq!(headers.get(ACCEPT_LANGUAGE).unwrap(), "en-US");
        assert_eq!(headers.get(ACCEPT_ENCODING).unwrap(), "gzip, deflate");

        let user_agent = headers.get(USER_AGENT).unwrap().to_str().unwrap();
        assert!(USER_AGENTS.contains(&user_agent));
    }
}
