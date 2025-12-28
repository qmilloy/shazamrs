use crate::client::Shazam;
use crate::constants::get_random_device;
use crate::{ShazamError, models::RecognizeResponse};
use shazamrs_core::Signature;
use uuid::Uuid;

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
        let url = format!(
            "https://amp.shazam.com/discovery/v5/{language}/{endpoint_country}/{device}/-/tag/{uuid_1}/{uuid_2}?sync=true&webv3=true&sampling=true&connected=&shazamapiversion=v3&sharehub=true&hubv5minorversion=v5.1&hidelb=true&video=v3",
            language = "en-US",
            endpoint_country = "GB",
            device = get_random_device(),
            uuid_1 = Uuid::new_v4().to_string().to_uppercase(),
            uuid_2 = Uuid::new_v4().to_string().to_uppercase()
        );

        let response = self
            .client
            .post(url)
            .headers(Shazam::generate_headers())
            .json(&signature)
            .send()
            .await?
            .error_for_status()?
            .json::<RecognizeResponse>()
            .await?;

        Ok(response)
    }
}
