# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`shazamrs` is a pure-Rust client for identifying music using Shazam's undocumented public discovery API (inspired by the Python `ShazamIO` library). It relies on Shazam's private API, so response schemas may change without notice and there are no long-term API stability guarantees.

## Repo layout

This is a two-crate workspace-like setup, but `shazamrs-core` is a **git submodule** (`vendor/shazamrs-core`, remote `git@github.com:qmilloy/shazamrs-core.git`), not a workspace member — it's pulled in via a `path` dependency in `Cargo.toml`. If `vendor/shazamrs-core` looks empty, run `git submodule update --init --recursive`.

- **`shazamrs-core`** (`vendor/shazamrs-core`) — low-level audio fingerprinting / Shazam signature generation (a Rust-only fork of `shazamio-core`). Exposes `Recognizer`, `Signature`, `SearchParams`, `SignatureError`.
- **`shazamrs`** (`src/`) — the public crate: HTTP client, request signing/headers, and typed response models built on top of `shazamrs-core`.

Changes to fingerprinting/signature-generation logic belong in the submodule; changes to networking, headers, or response parsing belong in the top-level crate.

## Commands

No `cargo`/`rustc` toolchain is available in this sandboxed environment — verify build/test changes have correct syntax by careful reading, and let the user run these where they have a toolchain:

```bash
cargo build
cargo test                                    # unit + integration tests (top-level crate)
cargo test -p shazamrs-core                   # tests for the submodule
cargo test --test recognizer                  # single integration test file (in vendor/shazamrs-core/tests)
cargo run --example recognize-song            # end-to-end demo against examples/data/Gloria.ogg
```

## Architecture

Request flow, top to bottom:

```
Audio (file path or raw bytes)
  → shazamrs_core::Recognizer.recognize_path/recognize_bytes  (src/recognize.rs)
  → shazamrs_core::Signature                                   (fingerprint of the audio)
  → POST to Shazam's discovery API                              (src/recognize.rs: send_signature)
  → serde-deserialized into models::RecognizeResponse           (src/models/)
```

Key files:
- `src/client.rs` — defines `Shazam` (holds a `shazamrs_core::Recognizer` + `reqwest::Client`), and `Shazam::generate_headers()`, which builds the Shazam-specific header set (`x-shazam-platform`, `x-shazam-appversion`, a randomized `User-Agent`, etc.) required for the API to accept requests.
- `src/recognize.rs` — the two public entry points (`recognize_path`, `recognize_bytes`) and `send_signature`, which builds the discovery API URL (random device, fresh UUIDv4 tag IDs per request) and POSTs the signature.
- `src/constants.rs` — `DEVICES` (iphone/android/web) and a large `USER_AGENTS` pool, with `get_random_device()` / `get_random_user_agent()` used to vary request fingerprints between calls.
- `src/error.rs` — `ShazamError`, a `thiserror` enum wrapping `shazamrs_core::SignatureError`, `reqwest::Error`, and an `InvalidResponse` variant.
- `src/models/` — one file per response concept (`track`, `artist`, `images`, `metadata`, `sections`, `actions`, `response`), all re-exported from `models::mod`. `RecognizeResponse` is the top-level deserialization target; all fields across models are optional/defaulted (`#[serde(default)]`) and structs are `#[non_exhaustive]` since Shazam's schema is undocumented and can change.

Audio decoding within `shazamrs-core` supports whatever `rodio` supports (MP3, WAV, FLAC, OGG, etc.). Segment duration (how much audio is fingerprinted) defaults to 10s and is configurable via `Shazam::with_segment_duration(seconds)`.
