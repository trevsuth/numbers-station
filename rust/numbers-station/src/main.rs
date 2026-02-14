fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44_100u32;
    let seed = 42u64;

    let seq = numbers_station::generator::generate_sequence(seed);
    let pcm = numbers_station::audio::render_beeps(&seq, sample_rate);

    // fixed secret, framed paylaod
    let secret = b"HELLO FROM THE NUMBERS STATION";
    let encoded = numbers_station::stego::embed_lsb(pcm, secret)
        .map_err(|e| format!("stego embed failed {e:?}"))?;

    numbers_station::audio::write_wav_mono_i16("outstation.wav", sample_rate, &encoded)?;

    // In-memory sanity checks
    let recovered = numbers_station::stego::extract_lsb(&encoded)
        .map_err(|e| format!("stego extract failed: {e:?}"))?;
    println!("Recovered: {}", String::from_utf8_lossy(&recovered));

    println!("Wrote out_station.wav");
    Ok(())
}
