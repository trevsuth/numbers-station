use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44_100u32;

    let seq = numbers_station::generator::generate_sequence();
    let pcm = numbers_station::audio::render_beeps(&seq, sample_rate);

    // fixed secret for POC
    let secret = b"HELLO FROM THE NUMBERS STATION";
    let encoded = numbers_station::stego::embed_lsb(pcm, secret)
        .map_err(|e| format!("stego embed failed {e}"))?;

    let out_path = PathBuf::from("out_station.wav");
    numbers_station::audio::write_wav_mono_i16(out_path, sample_rate, &encoded)?;

    println!("Wrote out_station.wav");
    Ok(())
}
