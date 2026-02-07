/// Extreemly basic LSB on i16 PCM
///
/// POC implementation
/// Encode a message into the lease significant bits of each sample
pub fn embed_lsb(mut samples: Vec<i16>, message: &[u8]) -> Result<Vec<i16>, &'static str> {
    let payload = build_payload(message);
    let required_bits = payload.len() * 8;

    if required_bits > samples.len() {
        return Err("Not enough samples to embed payload");
    }

    for (bit_idx, bit) in payload_bits(payload).enumerate() {
        let s = samples[bit_idx];
        let cleared = s & !1;
        samples[bit_idx] = cleared | (bit as i16);
    }

    Ok(samples)
}

pub fn extract_lsb(samples: &[i16]) -> Result<Vec<u8>, &'static str> {
    // First 32 bits = payload length
    if samples.len() < 32 {
        return Err("Not enough samples to read length header");
    }

    let len = read_u32_from_lsb(samples)?;
    let total_bytes = 4usize + (len as usize);
    let total_bits = total_bytes * 8;

    if samples.len() < total_bits {
        return Err("Not enough samples to read payload");
    }

    let payload = read_bytes_from_lsb(samples, total_bytes)?;
    Ok(payload[4..].to_vec())
}

fn build_payload(message: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + message.len());
    let len = message.len() as u32;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(message);
    out
}

fn payload_bits(payload: Vec<u8>) -> impl Iterator<Item = u8> {
    payload
        .into_iter()
        .flat_map(|byte| (0..8).map(move |i| (byte >> i) & 1))
}

fn read_u32_from_lsb(samples: &[i16]) -> Result<u32, &'static str> {
    let bytes = read_bytes_from_lsb(samples, 4)?;
    let mut arr = [0u8; 4];
    arr.copy_from_slice(&bytes);
    Ok(u32::from_le_bytes(arr))
}

fn read_bytes_from_lsb(samples: &[i16], num_bytes: usize) -> Result<Vec<u8>, &'static str> {
    let total_bits = num_bytes * 8;
    if samples.len() < total_bits {
        return Err("Not enough samples");
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
