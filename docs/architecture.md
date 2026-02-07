# Rust Architecture (Initial)

This crate is structured as a small pipeline:

1. Generate numbers-station content
2. Render content into raw audio
3. Apply steganographic encoding
4. Write final audio output

## Modules (initial)

- generator
  - Produces sequences of numbers or symbols
- audio
  - Converts sequences into PCM audio
- stego
  - Hides and extracts messages from PCM audio
- cli
  - Thin command-line interface

## Data Flow

generator → audio → stego → output

Steganography operates on raw audio buffers and is agnostic to how the
audio was generated.
