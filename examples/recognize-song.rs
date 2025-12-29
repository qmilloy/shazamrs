use shazamrs::{Shazam, ShazamError};

#[tokio::main]
async fn main() -> Result<(), ShazamError> {
    let shazam = Shazam::with_segment_duration(5);

    let response = shazam
        .recognize_path("examples/data/MidnightSpecial.mp3")
        .await?;

    if let Some(track) = response.track {
        println!(
            "🎵 {} — {}",
            track.title.unwrap_or_default(),
            track.subtitle.unwrap_or_default()
        );
    }

    Ok(())
}
