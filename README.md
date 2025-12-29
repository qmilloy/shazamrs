# shazamrs

A pure-Rust client inspired by the python library [ShazamIO](https://github.com/shazamio/ShazamIO) for identifying music using Shazam’s public discovery API.
This crate is composed of two layers:

- **`shazamrs-core`** — audio fingerprinting and signature generation leveraging a rust-only fork of [shazamio-core](https://github.com/shazamio/shazamio-core)
- **`shazamrs`** — network client, response models, and high-level API

## Features

- Fully async (`tokio`)
- Typed, schema-safe responses
- Minimal public API

## Quick start

```rust
use shazamrs::Shazam;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let shazam = Shazam::new();
    let response = shazam.recognize_path("song.mp3").await?;
    if let Some(track) = response.track {
        println!("🎵 {} — {}",
            track.title.unwrap_or_default(),
            track.subtitle.unwrap_or_default()
        );
    }
    Ok(())
}
```

## Examples

You can build and run the song recognition example with:

```bash
cargo run --example recognize-song
```

## Architecture

```text
Audio (file/bytes)
  ↓
shazamio-core (fingerprinting)
  ↓
HTTP (Shazam discovery API)
  ↓
Typed Rust response models
```

## Limitations

- This crate relies on Shazam’s **undocumented public API**
- Response schemas may change over time
- No guarantees of long-term API stability

## Legal notice

This project is **not affiliated with or endorsed by Apple or Shazam**.
Use at your own risk and comply with applicable laws and terms of service.
