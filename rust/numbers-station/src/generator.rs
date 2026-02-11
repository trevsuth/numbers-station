/// Generate a simple "numbers station sequence as text"
///
/// Current state:
/// - deterministic output from a seed
/// - no external RNG dependency
///
/// Output style:
/// - groups of 5 digits separated by spaces
/// - groups separated by double-space
pub fn generate_sequence(seed: u64) -> String {
    let mut rng = XorShift64::new(seed);

    let groups = 6usize;
    let group_len = 5usize;

    let mut out = String::new();

    for g in 0..groups {
        if g > 0 {
            out.push_str("  ");
        }
        for i in 0..group_len {
            if i > 0 {
                out.push(' ');
            }
            let digit = (rng.next_u32() % 10) as u8;
            out.push(char::from(b'0' + digit));
        }
    }

    out
}

/// Tiny deterministic RNG
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let s = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };
        Self { state: s }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 32) as u32
    }
}
