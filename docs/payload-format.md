# Payload Format (Framing)

## Purpose
Provide a stable, versioned payload that extraction can validate and evolve.

## Framed Payload Layout (v1)
All fields little-endian.

- magic: 4 bytes  = "NS01"
- version: u8     = 1
- flags: u8       = reserved (0 for now)
- len: u32        = message length in bytes
- message: [len]  = raw bytes
- checksum: u32   = optional (recommended: CRC32 of message), can be added in v2

Notes:
- v1 can omit checksum for speed of iteration if desired.
- Extraction must reject:
  - wrong magic
  - unsupported version
  - len that exceeds carrier capacity

## Capacity
LSB embedding provides 1 bit per sample.
Total capacity (bytes) ≈ samples / 8 (minus header).
