use reqwest::Client;
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
}
