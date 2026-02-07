use std::f32::consts::PI;
use std::io::{self, Write};
use std::path::Path;

/// Render a simple "beep perdigit" PCM buffer
/// - sample format: i16
/// - sample rate: 44_100 Hz
///
/// For each charactor in the sequence:
/// emit a tone per digit
/// emit silence for spaces
pub fn render_beeps(sequence: &str, sample_rate: u32) -> Vec<i16> {
    let tone_ms = 120u32;
    let gap_ms = 60u32;

    let tone_len = (sample_rate as u64 * tone_ms as u64 / 1000) as usize;
    let gap_len = (sample_rate as u64 * gap_ms as u64 / 1000) as usize;

    let mut out = Vec::new();

    for ch in sequence.chars() {
        match ch {
            '0'..='9' => {
                let digit = ch.to_digit(10).unwrap() as f32;

                // Map digits to a decently plesant freq range
                let freq = 440.0 + (digit * 40.0);

                out.extend(tone_sine(sample_rate, freq, tone_len, 0.25));
                out.extend(std::iter::repeat(0i16).take(gap_len));
            }
            _ => {
                // Treat everything else as silence
                out.extend(std::iter::repeat(0i16).take(gap_len));
            }
        }
    }

    out
}

fn tone_sine(sample_rate: u32, freq_hz: f32, len: usize, amp: f32) -> Vec<i16> {
    let sr = sample_rate as f32;
    let mut v = Vec::with_capacity(len);

    for n in 0..len {
        let t = n as f32 / sr;
        let sample = (2.0 * PI * freq_hz * t).sin() * amp;
        let s = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        v.push(s);
    }

    v
}

/// Write PCM i16 mono as a WAV file (PCM, 16-bit, little-endian).
///
/// Minimal writer to avoid addingdeps as POC
pub fn write_wav_mono_i16<P: AsRef<Path>>(
    path: P,
    sample_rate: u32,
    samples: &[i16],
) -> io::Result<()> {
    let mut f = std::fs::File::create(path)?;

    let num_channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let byte_rate: u32 = sample_rate * num_channels as u32 * (bits_per_sample as u32 / 8);
    let block_align: u16 = num_channels * (bits_per_sample / 8);

    let data_bytes = (samples.len() * 2) as u32;
    let riff_chunk_size = 36 + data_bytes;

    // RIFF header
    f.write_all(b"RIFF")?;
    f.write_all(&riff_chunk_size.to_le_bytes())?;
    f.write_all(b"WAVE")?;

    // fmt chunk
    f.write_all(b"fmt ")?;
    f.write_all(&16u32.to_le_bytes())?; // PCM fmt chunk size
    f.write_all(&1u16.to_le_bytes())?; // audio format: 1 = PCM
    f.write_all(&num_channels.to_le_bytes())?;
    f.write_all(&sample_rate.to_le_bytes())?;
    f.write_all(&byte_rate.to_le_bytes())?;
    f.write_all(&block_align.to_le_bytes())?;
    f.write_all(&bits_per_sample.to_le_bytes())?;

    // data chunk
    f.write_all(b"data")?;
    f.write_all(&data_bytes.to_le_bytes())?;

    // samples
    for &s in samples {
        f.write_all(&s.to_le_bytes())?;
    }

    Ok(())
}
