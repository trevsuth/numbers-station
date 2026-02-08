# Rust Architecture

## Goal
Generate numbers-station-style audio and optionally embed/extract a hidden message using audio steganography.

This repository may contain other tooling later, but this document covers only the Rust crate.

## Pipeline
1. Generate numbers-station content (sequence)
2. Render content into raw PCM audio (cover)
3. Apply steganography to produce carrier audio
4. Write WAV output(s)

Data flow:
generator → audio → stego → wav

Steganography operates on PCM buffers and is agnostic to how the audio was generated.

## Modules

### generator
- Responsibility: deterministic sequence generation.
- API (target):
  - `generate_sequence(seed: u64) -> String`

### audio
- Responsibility: render sequences to PCM and read/write WAV.
- API (target):
  - `render_beeps(sequence: &str, sample_rate: u32) -> Vec<i16>`
  - `write_wav_mono_i16(path, sample_rate, samples) -> io::Result<()>`
  - `read_wav_mono_i16(path) -> io::Result<(u32, Vec<i16>)>`

### stego
- Responsibility: embed/extract framed payloads in PCM.
- Constraints:
  - Treat audio samples as the only carrier (no dependency on sequence generation).
- API (target):
  - `embed_lsb(samples: Vec<i16>, message: &[u8], key: Option<&[u8]>) -> Result<Vec<i16>, StegoError>`
  - `extract_lsb(samples: &[i16], key: Option<&[u8]>) -> Result<Vec<u8>, StegoError>`

### cli (main)
- Responsibility: thin wrapper over library modules.
- Notes:
  - CLI comes after core invariants are stable.

## Implementation Passes

### Pass 1 — Determinism + Framed Payload (in-memory)
Deliverables:
- Seeded generator (`generate_sequence(seed)`).
- Framed payload format (magic/version/len + optional checksum).
- Stego embed/extract works on framed payload.
- Unit/integration tests for:
  - generator determinism
  - embed→extract round-trip
  - invalid header rejection

### Pass 2 — WAV Read + Artifact Round-trip
Deliverables:
- Minimal WAV reader for mono i16 PCM.
- Integration test:
  - generate cover → embed → write WAV → read WAV → extract → assert equality

### Pass 3 — CLI + Usability + Upgrades
Deliverables:
- `clap` CLI with subcommands (render/encode/decode).
- Optional improvements:
  - keyed/strided embedding positions
  - station-like audio markers / noise bed
- Write both cover + carrier outputs to aid debugging/listening.

## Testing Strategy
- Core invariant: `extract(embed(samples, msg)) == msg`
- Artifact invariant: write/read preserves extractability
- Prefer few high-value integration tests over many tiny ones early.

