use numbers_station::{audio, generator, stego};

#[test]
fn lsb_roundtrip_recovers_original_messge() {
    let sample_rate = 44_100u32;

    // generate some cover audio
    let seq = generator::generate_sequence();
    let pcm = audio::render_beeps(&seq, sample_rate);

    // hide and recover a set message
    let secret = b"ROUNDTRIP: the message must survive";

    // embed then extract
    let encoded = stego::embed_lsb(pcm, secret).expect("embed should succeed");
    let recovered = stego::extract_lsb(&encoded).expect("extract should succeed");

    assert_eq!(recovered, secret);
}

#[test]
fn lsb_embed_fails_when_payload_exceeds_capacity() {
    let sample_rate = 44_100u32;

    // small cover audio on purpose
    let seq = "1";
    let pcm = audio::render_beeps(seq, sample_rate);

    // make payload too large for num samples
    // capacity is 1 bit per sample, so byte capacity is samples /8 (minus header).
    let too_big = vec![0u8; (pcm.len() / 8) + 100];

    let err = stego::embed_lsb(pcm, &too_big).unwrap_err();
    assert_eq!(err, "Not enough samples to embed payload");
}
