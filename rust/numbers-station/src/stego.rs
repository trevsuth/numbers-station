/// Simple LSB on i16 PCM with a framed payload
///
/// Framed payload layout (little-endian):
/// - magic:    4 bytes =   "NS01"
/// - version:  u8      =   1
/// - flags:    u8      =   0 (reserved)
/// - len:      u32     =   message length in bytes
/// - message: [len]

const MAGIC: &[u8; 4] = b"NS01";
const VERSION: u8 = 1;
const HEADER_LEN: usize = 4 + 1 + 1 + 4; // magic + version + flags + len

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StegoError {
    NotEnoughSamples,
    BadMagic,
    UnsupportedVersion(u8),
    LengthOutOfRange,
}

/// Embed a framed payload into the LSB of the PCM samples
///
/// Capacity: 1 bit per sample => bytes_capacity ~= samples.len() / 8
pub fn embed_lsb(mut samples: Vec<i16>, message: &[u8]) -> Result<Vec<i16>, StegoError> {
    let frame = build_frame(message);
    let required_bits = frame.len() * 8;

    if required_bits > samples.len() {
        return Err(StegoError::NotEnoughSamples);
    }

    for (bit_idx, bit) in frame_bits(&frame).enumerate() {
        let s = samples[bit_idx];
        let cleared = s & !1;
        samples[bit_idx] = cleared | (bit as i16);
    }

    Ok(samples)
}

/// Extract and validate framed payload from the LSB of the PCM samples
///
/// THis reads the fixed-size header first, then reads 'len' message bytes
pub fn extract_lsb(samples: &[i16]) -> Result<Vec<u8>, StegoError> {
    // Need enough bits for header bytes
    let header_bits = HEADER_LEN * 8;
    if samples.len() < header_bits {
        return Err(StegoError::NotEnoughSamples);
    }

    let header = read_bytes_from_lsb(samples, HEADER_LEN)?;
    validate_magic(&header[0..4])?;
    validate_version(header[4])?;
    // flags at header[5] currently ignored/reserves
    let len = u32::from_le_bytes([header[6], header[7], header[8], header[9]]) as usize;

    // Compute total frame size and ensure we have enough samples
    let total_bytes = HEADER_LEN
        .checked_add(len)
        .ok_or(StegoError::LengthOutOfRange)?;
    let total_bits = total_bytes * 8;

    if samples.len() < total_bits {
        return Err(StegoError::NotEnoughSamples);
    }

    let frame = read_bytes_from_lsb(samples, total_bytes)?;
    validate_magic(&frame[0..4])?;
    validate_version(frame[4])?;

    // flags = frame[5] (reserved)
    let declared_len = u32::from_le_bytes([frame[6], frame[7], frame[8], frame[9]]) as usize;

    if declared_len != len {
        // This indicated header/body mismatch (i.e. corruption)
        return Err(StegoError::LengthOutOfRange);
    }

    Ok(frame[HEADER_LEN..].to_vec())
}

fn build_frame(message: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + message.len());
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    out.push(0u8); // flags (reserved)
    out.extend_from_slice(&(message.len() as u32).to_le_bytes());
    out.extend_from_slice(message);
    out
}

fn frame_bits(frame: &[u8]) -> impl Iterator<Item = u8> + '_ {
    frame
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> i) & 1))
}

fn read_bytes_from_lsb(samples: &[i16], num_bytes: usize) -> Result<Vec<u8>, StegoError> {
    let total_bits = num_bytes * 8;
    if samples.len() < total_bits {
        return Err(StegoError::NotEnoughSamples);
    }

    let mut out = vec![0u8; num_bytes];
    for bit_idx in 0..total_bits {
        let bit = (samples[bit_idx] & 1) as u8;
        let byte_idx = bit_idx / 8;
        let bit_in_byte = bit_idx % 8;
        out[byte_idx] |= bit << bit_in_byte;
    }
    Ok(out)
}

fn validate_magic(m: &[u8]) -> Result<(), StegoError> {
    if m == MAGIC {
        Ok(())
    } else {
        Err(StegoError::BadMagic)
    }
}

fn validate_version(v: u8) -> Result<(), StegoError> {
    if v == VERSION {
        Ok(())
    } else {
        Err(StegoError::UnsupportedVersion(v))
    }
}
