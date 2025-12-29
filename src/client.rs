use crate::constants::get_random_user_agent;
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
/// use shazamrs::Shazam;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
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
}

impl Shazam {
    /// Create a new `Shazam` client using the default
    /// segment duration (10 seconds).
    pub fn new() -> Self {
        Self {
            recognizer: Recognizer::new(None),
            client: Client::new(),
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
        }
    }

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
