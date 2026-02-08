# Workflow

## Commands (from repo root)
- `just build`
- `just test`
- `just run`
- `just fmt`
- `just clippy`

## Pass discipline
- Pass 1: all in-memory; do not add WAV reader or CLI yet.
- Pass 2: add WAV reader + file round-trip test.
- Pass 3: add CLI + optional improvements.

## Outputs
- v0: `out_station.wav` (carrier only)
- later: `cover.wav` and `carrier.wav`
