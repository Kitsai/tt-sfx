# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

tt-sfx is a networked sound effect player and sync engine for tabletop RPG sessions, written in Rust. It is currently in early skeleton form — the GUI app (`TtSfxApp`) has an unimplemented `ui()` method, and the networking/audio/MIDI functionality described by the dependencies has not been built yet.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Check (fast compile check without codegen): `cargo check`
- Test: `cargo test`
- Run a single test: `cargo test <test_name>`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Architecture

The crate is split into a library (`tt_sfx`, `src/lib.rs`) and a thin binary (`src/main.rs`) that just calls into the library. All application logic belongs in the library so it stays testable; `main.rs` should remain a minimal entry point.

- `src/main.rs` — binary entry point, calls `tt_sfx::TtSfxApp::run()`.
- `src/lib.rs` — crate root, re-exports the `app` module.
- `src/app.rs` — `TtSfxApp`, the `eframe`/`egui` application struct and its `App` trait implementation (rendering, native window setup).

### Network model: host/client sync

The sync engine follows a host-authoritative model, not audio streaming:

- **Host (DM)** runs the session and is the source of truth for the soundboard/asset library. It owns the WebSocket server that clients connect to.
- **Clients (players)** connect to the host and receive both assets and events, but never stream audio — they play sounds locally from files already synced to disk.

This "ship the file, then fire an event" design is deliberate: playback timing only depends on a lightweight event message arriving, not on sustained low-latency audio streaming, so it stays robust on mediocre connections (the common case for a remote TTRPG session).

Three message categories flow from host to clients over the WebSocket connection:

1. **Asset sync** — the host pushes new/changed sound files (and metadata, e.g. via `blake3` hashes to detect what a client is missing/stale on) to clients ahead of time, so the file is already on disk locally before any event references it.
2. **Global broadcast events** — e.g. "play sound X", "stop all", sent to every connected client at once (ambient/scene SFX everyone should hear).
3. **Targeted player events** — sent to a specific client only (e.g. a private cue or a per-player effect), rather than broadcast to the whole session.

Expect this to shape the eventual module layout: a server/host side (WebSocket server, asset distribution, broadcast + per-client targeting) and a client side (WebSocket client, asset cache, local playback via `rodio` triggered by incoming events) as distinct modules, with `serde`/`bincode` defining the shared message types between them.

### Dependencies

- `eframe` — the native application shell for `egui` (window creation, event loop, native options). This is what `TtSfxApp::run()` calls into.
- `egui` — the immediate-mode GUI library used to build the actual UI (buttons, layout, widgets) inside `TtSfxApp::ui()`.
- `rodio` — audio decoding and playback; this is the engine that will actually play SFX files through an output device.
- `midir` — low-level MIDI I/O (listing devices, opening input/output ports, receiving raw MIDI messages), for hardware controllers triggering SFX.
- `midi-control` — parses raw MIDI bytes from `midir` into structured messages (note on/off, control change, etc.) instead of hand-rolling MIDI parsing.
- `tokio` — async runtime underpinning the networking layer (and anything else that needs async I/O, timers, or tasks).
- `tokio-tungstenite` — WebSocket client/server on top of `tokio`, the transport for the multi-client sync engine (broadcasting play/stop/volume events between sessions).
- `futures-util` (`sink` feature) — combinator/sink utilities for working with the async streams `tokio-tungstenite` produces (e.g. sending on a `WebSocket` sink).
- `async-trait` — enables `async fn` in traits, useful once there's an abstraction over transports or backends that needs async methods.
- `notify` — cross-platform filesystem watching, likely for auto-reloading a soundboard/asset folder when files are added or changed without restarting the app.
- `serde` (`derive`) — derives `Serialize`/`Deserialize` for config, soundboard definitions, and network messages.
- `serde_json` — human-readable serialization, likely for on-disk config/soundboard files a user can hand-edit.
- `bincode` — compact binary serialization, likely for the wire format of sync messages sent over WebSocket (smaller/faster than JSON).
- `blake3` — fast content hashing, likely for identifying/deduplicating audio assets or verifying both sides of a sync session have the same file.
- `thiserror` — ergonomic `#[derive(Error)]` for defining the crate's error enums instead of hand-writing `Display`/`Error` impls.
- `tempfile` — creates temporary files/directories, likely useful for tests or staging downloaded/received assets before committing them to the library.

As networking, audio, and MIDI features are implemented, expect new modules alongside `app.rs` (e.g. for the sync protocol, audio engine, and MIDI input handling) rather than growing `app.rs` into a monolith.

## Working style

When asked to add or change functionality in this project, prioritize explaining the relevant concepts and providing illustrative code snippets over writing the full, final implementation. This is a learning-oriented project — favor short examples and rationale that the user can extend themselves, and only write complete production code when explicitly asked to implement something.
