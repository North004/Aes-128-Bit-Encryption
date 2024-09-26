const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const INVSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

pub fn block_encrypt(state: &mut [u8; 16], key: &[u8; 16]) {
    let round_keys = generate_round_keys(key);
    add_round_key(state, &round_keys[0]);
    for round in round_keys.iter().take(10).skip(1) {
        sub_bytes(state);
        shift_rows(state);
        mix_cols(state);
        add_round_key(state, round);
    }
    sub_bytes(state);
    shift_rows(state);
    add_round_key(state, &round_keys[10]);
}

pub fn block_decrypt(state: &mut [u8; 16], key: &[u8; 16]) {
    let round_keys = generate_round_keys(key);
    add_round_key(state, &round_keys[10]);
    for round in round_keys.iter().take(10).skip(1).rev() {
        inv_shift_rows(state);
        inv_sub_bytes(state);
        add_round_key(state, round);
        inv_mix_cols(state);
    }
    inv_shift_rows(state);
    inv_sub_bytes(state);
    add_round_key(state, &round_keys[0]);
}

fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
    for (s, k) in state.iter_mut().zip(key.iter()) {
        *s ^= k;
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = SBOX[*byte as usize];
    }
}

fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = INVSBOX[*byte as usize]
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let temp = *state;
    state[0] = temp[0];
    state[1] = temp[5];
    state[2] = temp[10];
    state[3] = temp[15];
    state[4] = temp[4];
    state[5] = temp[9];
    state[6] = temp[14];
    state[7] = temp[3];
    state[8] = temp[8];
    state[9] = temp[13];
    state[10] = temp[2];
    state[11] = temp[7];
    state[12] = temp[12];
    state[13] = temp[1];
    state[14] = temp[6];
    state[15] = temp[11];
}

fn inv_shift_rows(state: &mut [u8; 16]) {
    let temp = *state;
    state[0] = temp[0];
    state[4] = temp[4];
    state[8] = temp[8];
    state[12] = temp[12];
    state[1] = temp[13];
    state[2] = temp[10];
    state[3] = temp[7];
    state[5] = temp[1];
    state[6] = temp[14];
    state[7] = temp[11];
    state[9] = temp[5];
    state[10] = temp[2];
    state[11] = temp[15];
    state[13] = temp[9];
    state[14] = temp[6];
    state[15] = temp[3];
}

fn mix_cols(state: &mut [u8; 16]) {
    let temp = *state;
    for col in 0..4 {
        let offset = col * 4;
        let s = [
            temp[offset],
            temp[offset + 1],
            temp[offset + 2],
            temp[offset + 3],
        ];

        state[offset] = gal_mul(s[0], 0x02) ^ gal_mul(s[1], 0x03) ^ s[2] ^ s[3];
        state[offset + 1] = s[0] ^ gal_mul(s[1], 0x02) ^ gal_mul(s[2], 0x03) ^ s[3];
        state[offset + 2] = s[0] ^ s[1] ^ gal_mul(s[2], 0x02) ^ gal_mul(s[3], 0x03);
        state[offset + 3] = gal_mul(s[0], 0x03) ^ s[1] ^ s[2] ^ gal_mul(s[3], 0x02);
    }
}

fn inv_mix_cols(state: &mut [u8; 16]) {
    let temp = *state;
    for col in 0..4 {
        let offset = col * 4;
        let s = [
            temp[offset],
            temp[offset + 1],
            temp[offset + 2],
            temp[offset + 3],
        ];

        state[offset] =
            gal_mul(s[0], 0x0e) ^ gal_mul(s[1], 0x0b) ^ gal_mul(s[2], 0x0d) ^ gal_mul(s[3], 0x09);
        state[offset + 1] =
            gal_mul(s[0], 0x09) ^ gal_mul(s[1], 0x0e) ^ gal_mul(s[2], 0x0b) ^ gal_mul(s[3], 0x0d);
        state[offset + 2] =
            gal_mul(s[0], 0x0d) ^ gal_mul(s[1], 0x09) ^ gal_mul(s[2], 0x0e) ^ gal_mul(s[3], 0x0b);
        state[offset + 3] =
            gal_mul(s[0], 0x0b) ^ gal_mul(s[1], 0x0d) ^ gal_mul(s[2], 0x09) ^ gal_mul(s[3], 0x0e);
    }
}

fn generate_round_keys(key: &[u8; 16]) -> [[u8; 16]; 11] {
    let mut round_keys = [[0u8; 16]; 11];
    round_keys[0].copy_from_slice(key);
    let mut temp = [0u8; 4];
    for i in 1..11 {
        temp.copy_from_slice(&round_keys[i - 1][12..16]);
        temp.rotate_left(1);

        for byte in &mut temp {
            *byte = SBOX[*byte as usize];
        }

        temp[0] ^= RCON[i - 1];

        for (j, t) in temp.iter().enumerate() {
            round_keys[i][j] = round_keys[i - 1][j] ^ t;
            round_keys[i][j + 4] = round_keys[i - 1][j + 4] ^ round_keys[i][j];
            round_keys[i][j + 8] = round_keys[i - 1][j + 8] ^ round_keys[i][j + 4];
            round_keys[i][j + 12] = round_keys[i - 1][j + 12] ^ round_keys[i][j + 8];
        }
    }

    round_keys
}

fn gal_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0u8;
    while b > 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        let car = a & 0x80;
        a <<= 1;
        if car != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
    }
    result
}

pub fn pk7_pad(data: &mut Vec<u8>) {
    let len = data.len();
    let pad_len = 16 - (len % 16);
    data.extend(vec![pad_len as u8; pad_len]);
}

pub fn pk7_unpad(data: &mut Vec<u8>) {
    if let Some(&pad_len) = data.last() {
        let pad_len = pad_len as usize;
        if pad_len > 0 && pad_len <= 16 {
            data.truncate(data.len() - pad_len);
        }
    }
}

#[cfg(test)]
mod tests {
    use hex_lit::hex;

    use super::*;
    #[test]
    fn test_shift_rows() {
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let expected = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        shift_rows(&mut state);
        assert_eq!(state, expected);
        inv_shift_rows(&mut state);
        assert_eq!(state, original);
    }
    #[test]
    fn test_byte_sub() {
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        sub_bytes(&mut state);
        inv_sub_bytes(&mut state);
        assert_eq!(state, original);
    }
    #[test]
    fn test_gal_mul() {
        //verified by hand
        assert_eq!(gal_mul(2, 2), 4);
        assert_eq!(gal_mul(6, 3), 10);
        assert_eq!(gal_mul(7, 12), 36);
        assert_eq!(gal_mul(12, 12), 80);
    }
    #[test]
    fn test_col_shift() {
        //Note this works i calculated the matrix by hand
        let mut state = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        let original = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4];
        mix_cols(&mut state);
        inv_mix_cols(&mut state);
        assert_eq!(state, original);
    }
    #[test]
    fn test_encryption() {
        //values taken from test vector https://csrc.nist.gov/files/pubs/fips/197/final/docs/fips-197.pdf
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let mut plaintext = hex!("00112233445566778899aabbccddeeff");
        let expected = hex!("69c4e0d86a7b0430d8cdb78070b4c55a");
        block_encrypt(&mut plaintext, &key);
        assert_eq!(plaintext, expected);
    }
    #[test]
    fn test_decryption() {
        //values taken from test vector https://csrc.nist.gov/files/pubs/fips/197/final/docs/fips-197.pdf
        let key = hex!("000102030405060708090a0b0c0d0e0f");
        let mut ciphertext = hex!("69c4e0d86a7b0430d8cdb78070b4c55a");
        let expected = hex!("00112233445566778899aabbccddeeff");
        block_decrypt(&mut ciphertext, &key);
        assert_eq!(ciphertext, expected);
    }
}
