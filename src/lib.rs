/// as the name imply, it transforms a [u8;4] into a u32 (which has 4 bytes)
macro_rules! bytes_to_u32 {
    ($w:expr) => {
        u32::from_be_bytes(
            $w.try_into()
                .expect("Couldn't convert &[u8] to [u8;4], check your len!"),
        )
    };
}

/// core function of sha1, perfom bitwise operation to gain entropy in information, enventually lost due to wrapping operation
fn logical_function(i: u32, b: u32, c: u32, d: u32) -> (u32, u32) {
    if i <= 19 {
        ((b & c) | ((!b) & d), 0x5A827999)
    } else if i <= 39 {
        (b ^ c ^ d, 0x6ED9EBA1)
    } else if i <= 59 {
        ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
    } else if i <= 79 {
        (b ^ c ^ d, 0xCA62C1D6)
    } else {
        panic!("i should be between 0 and 79")
    }
}

pub fn chat1(message: Vec<u8>) -> String {
    let original_message_length = 8 * message.len() as u64; // in bits

    /* 1. pre process the message so that its have a good shape for the following step */
    let mut pre_processed_message = message;
    pre_processed_message.push(0x80); // add 1 then pad with 0 the remaining 7 bit into the message (0x80 in hex = 10000000 in binary)

    // pad with 0 until size%512==448
    {
        let current_size = original_message_length + 8;
        let padding_size = if current_size % 512 <= 448 {
            448 - current_size % 512
        } else {
            /*512 and 448 are between brackets to avoid underflowing with 'current_size % 512' (CS) since '448-CS < 0' in our case
            note that rust execute from left to right, so even without bracket it will works fine, but if we change the order or put
            brackets between 448 and CS it will panic, so don't touch them*/
            (512 + 448) - current_size % 512
        };
        pre_processed_message.resize(((current_size + padding_size) / 8) as usize, 0);
    }
    assert_eq!((pre_processed_message.len() * 8) % 512, 448); // for debug purpose

    // add the length of the original message (in bits) into the message
    pre_processed_message.extend(original_message_length.to_be_bytes());
    assert_eq!((pre_processed_message.len() * 8) % 512, 0); // for debug purpose

    /* 2. Process the message */
    // define 5 states with random hex values
    let (mut h0, mut h1, mut h2, mut h3, mut h4) = (
        0x67452301_u32,
        0xEFCDAB89_u32,
        0x98BADCFE_u32,
        0x10325476_u32,
        0xC3D2E1F0_u32,
    );

    // break the message into 512 bit chuncks: (512 bits = 64 bytes)
    for msg_chunck in pre_processed_message.chunks(64) {
        let mut w = msg_chunck
            .chunks(4)
            .map(|word| bytes_to_u32!(word)) // all the words are of len 4, so should be safe
            .collect::<Vec<_>>(); // 64/4=16, hence initially w has 16 elements
        w.reserve_exact(64); // at the end of the next step w will have exactly 80 elements, so we reserve them: 16+64=80
        for i in 16..80 {
            // sha1 spec
            w.push((w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1));
        }

        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        // sha1 spec
        for i in 0..80 {
            let (f, k) = logical_function(i, b, c, d);
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i as usize]);
            (e, d, c, b, a) = (d, c, b.rotate_left(30), a, temp);
        }

        // write result
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use nanorand::{Rng, WyRand};
    use once_cell::sync::Lazy;
    use sha1::{Digest, Sha1};

    use crate::chat1;

    /// debug function, that prints the in binary representation the bytes of a string
    fn bytes_to_hex(bytes: &[u8]) -> String {
        let mut in_hex = "".to_string();
        for h in bytes {
            in_hex += &format!("{:02x}", h);
        }
        in_hex
    }

    fn reference_sha1(msg: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(msg.as_bytes());
        let result = hasher.finalize().to_vec();
        bytes_to_hex(&result)
    }

    const CHARSSTR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~`!@#$%^&*()_+-={}[]\\|;':\"<>,./?";
    static CHARS: Lazy<Mutex<Vec<char>>> =
        Lazy::new(|| CHARSSTR.chars().collect::<Vec<_>>().into());

    fn random_string(size: usize) -> String {
        let chars_data = CHARS.lock().unwrap();

        let mut rng = WyRand::new();

        let mut result = String::new();
        for _ in 0..size {
            let i = rng.generate_range(0_usize..chars_data.len());
            result.push(chars_data[i]);
        }
        result
    }

    #[test]
    fn chat1_test() {
        let mut rng = WyRand::new();

        for _ in 0..10_000 {
            let message = random_string(rng.generate_range(0_usize..10_000));
            assert_eq!(
                chat1(message.clone().into_bytes()),
                reference_sha1(&message)
            )
        }
    }
}

/* DEAD CODE */

// perform a circular shift operation, deprecated: I didn't know rust already have this function in the std...
// fn rotl(x: u32, n: u8) -> u32 {
//     assert!(n <= 32); // for debug purpose
//     (x << n) | (x >> (32 - n))
// }

// debug function, that prints the in binary representation the bytes of a string
// fn debug_string_bytes(bytes: &[u8]) {
//     let mut in_binary = "".to_string();
//     for b in bytes {
//         in_binary += &format!("{:08b} ", b);
//     }
//     println!("{in_binary}");
// }
