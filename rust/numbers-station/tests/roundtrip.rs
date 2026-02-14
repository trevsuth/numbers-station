use numbers_station::{audio, generator, stego};

#[test]
fn generator_is_deterministic_for_same_seed() {
    let a = generator::generate_sequence(123);
    let b = generator::generate_sequence(123);
    assert_eq!(a, b);

    let c = generator::generate_sequence(124);
    assert_ne!(a, c);
}
#[test]
fn lsb_roundtrip_recovers_original_messge() {
    let sample_rate = 44_100u32;

    // generate some cover audio
    let seq = generator::generate_sequence(999);
    let pcm = audio::render_beeps(&seq, sample_rate);

    // hide and recover a set message
    let secret = b"ROUNDTRIP: the message must survive";

    // embed then extract
    let encoded = stego::embed_lsb(pcm, secret).expect("embed should succeed");
    let recovered = stego::extract_lsb(&encoded).expect("extract should succeed");

    assert_eq!(recovered, secret);
}

#[test]
fn extract_rejects_bad_magic() {
    let sample_rate = 44_100u32;

    let seq = generator::generate_sequence(1);
    let pcm = audio::render_beeps(&seq, sample_rate);

    let secret = b"VALID PAYLOAD";
    let mut encoded = stego::embed_lsb(pcm, secret).expect("embed should succeed");

    // Corrupt the first byte of the frame ("NS01") vy flipping the first bit
    // This alters the extracted magic and should trigger BadMagic
    encoded[0] ^= 1;

    let err = stego::extract_lsb(&encoded).unwrap_err();
    assert_eq!(err, stego::StegoError::BadMagic);
}

#[test]
fn lsb_embed_fails_when_payload_exceeds_capacity() {
    let sample_rate = 44_100u32;

    let seq = generator::generate_sequence(2);
    let pcm = audio::render_beeps(&seq, sample_rate);

    // make payload too large for num samples
    // capacity is 1 bit per sample, so byte capacity is samples /8 (minus header).
    let too_big = vec![0u8; (pcm.len() / 8) + 10_000];

    let err = stego::embed_lsb(pcm, &too_big).unwrap_err();
    assert_eq!(err, stego::StegoError::NotEnoughSamples);
}
