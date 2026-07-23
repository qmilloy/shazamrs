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
            "{base_url}/discovery/v5/{language}/{endpoint_country}/{device}/-/tag/{uuid_1}/{uuid_2}?sync=true&webv3=true&sampling=true&connected=&shazamapiversion=v3&sharehub=true&hubv5minorversion=v5.1&hidelb=true&video=v3",
            base_url = self.base_url,
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

#[cfg(test)]
mod tests {
    use crate::ShazamError;
    use crate::client::Shazam;
    use shazamrs_core::{Geolocation, Signature, SignatureSong};
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn sample_signature() -> Signature {
        Signature::new(
            Geolocation::new(300, 45, 2),
            SignatureSong::new(
                10000,
                1700000000,
                "data:audio/vnd.shazam.sig;base64,AA==".to_string(),
            ),
            1700000000,
            "Europe/Paris".to_string(),
        )
    }

    async fn shazam_pointed_at(mock_server: &MockServer) -> Shazam {
        let mut shazam = Shazam::new();
        shazam.base_url = mock_server.uri();
        shazam
    }

    #[tokio::test]
    async fn send_signature_posts_and_parses_recognized_track() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "track": { "title": "Song Title", "subtitle": "Artist Name" },
                "tagid": "ABCDEF",
                "timestamp": 1700000000u64
            })))
            .expect(1)
            .mount(&mock_server)
            .await;

        let shazam = shazam_pointed_at(&mock_server).await;
        let response = shazam.send_signature(sample_signature()).await.unwrap();

        let track = response.track.expect("track should be present");
        assert_eq!(track.title.as_deref(), Some("Song Title"));
        assert_eq!(track.subtitle.as_deref(), Some("Artist Name"));
        assert_eq!(response.tagid.as_deref(), Some("ABCDEF"));
    }

    #[tokio::test]
    async fn send_signature_handles_unrecognized_audio() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "timestamp": 1700000000u64
            })))
            .mount(&mock_server)
            .await;

        let shazam = shazam_pointed_at(&mock_server).await;
        let response = shazam.send_signature(sample_signature()).await.unwrap();

        assert!(response.track.is_none());
    }

    #[tokio::test]
    async fn send_signature_returns_http_error_on_failure_status() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let shazam = shazam_pointed_at(&mock_server).await;
        let result = shazam.send_signature(sample_signature()).await;

        assert!(matches!(result, Err(ShazamError::Http(_))));
    }
}
