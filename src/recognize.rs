use crate::client::Shazam;
use crate::{ShazamError, models::RecognizeResponse};
use shazamrs_core::Signature;

impl Shazam {
    /// Recognize a song from an audio file path.
    ///
    /// Supported formats depend on `rodio` and include
    /// common formats such as MP3, WAV, and FLAC.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - Signature generation fails
    /// - The Shazam API returns an error
    pub async fn recognize_path(
        &self,
        path: impl AsRef<str>,
    ) -> Result<RecognizeResponse, ShazamError> {
        let signature = self
            .recognizer
            .recognize_path(path.as_ref().to_string(), None)
            .await?;

        self.send_signature(signature).await
    }

    /// Recognize a song from raw audio bytes.
    ///
    /// This is useful for in-memory audio streams
    /// or embedded environments.
    pub async fn recognize_bytes(&self, audio: Vec<u8>) -> Result<RecognizeResponse, ShazamError> {
        let signature = self.recognizer.recognize_bytes(audio, None).await?;

        self.send_signature(signature).await
    }

    /// Send a generated signature to the Shazam backend
    /// and deserialize the response.
    async fn send_signature(&self, signature: Signature) -> Result<RecognizeResponse, ShazamError> {
        let url = "https://amp.shazam.com/discovery/v5/en/US/android/-/tag";

        let response = self
            .client
            .post(url)
            .json(&signature)
            .send()
            .await?
            .error_for_status()?
            .json::<RecognizeResponse>()
            .await?;

        Ok(response)
    }
}
