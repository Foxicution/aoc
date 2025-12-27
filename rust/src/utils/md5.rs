// Per-round shift amounts
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

// Integer part of 4294967296 * abs(sin(i)), where i is in radians
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[inline(always)]
fn f(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

#[inline(always)]
fn g(b: u32, c: u32, d: u32) -> u32 {
    (b & d) | (c & !d)
}

#[inline(always)]
fn h(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

#[inline(always)]
fn i(b: u32, c: u32, d: u32) -> u32 {
    c ^ (b | !d)
}

#[inline(always)]
fn step(
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    word: u32,
    k: u32,
    s: u32,
    f: fn(u32, u32, u32) -> u32,
) -> u32 {
    // a = b + ((a + F(b, c, d) + X[k] + T[i]) <<< s)
    b.wrapping_add(
        a.wrapping_add(f(b, c, d))
            .wrapping_add(word)
            .wrapping_add(k)
            .rotate_left(s),
    )
}

fn proc_block(input: &[u8; 64], a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
    let mut x = [0u32; 16];
    for (i, chunk) in input.chunks(4).enumerate() {
        x[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    let a0 = *a;
    let b0 = *b;
    let c0 = *c;
    let d0 = *d;

    // We iterate 0..16. The index of the word 'g' is just 'i'.
    for i in 0..16 {
        *a = step(*a, *b, *c, *d, x[i], K[i], S[i], f);
        // Rotate variables: (D, A, B, C) becomes new (A, B, C, D) for the next step
        let temp = *d;
        *d = *c;
        *c = *b;
        *b = *a;
        *a = temp;
    }

    // 4. Round 2
    // Word index pattern: (5*i + 1) % 16
    for i in 16..32 {
        *a = step(*a, *b, *c, *d, x[(5 * i + 1) % 16], K[i], S[i], g); // Use function g
        let temp = *d;
        *d = *c;
        *c = *b;
        *b = *a;
        *a = temp;
    }

    // 5. Round 3
    // Word index pattern: (3*i + 5) % 16
    for i in 32..48 {
        let g = (3 * i + 5) % 16;
        *a = step(*a, *b, *c, *d, x[g], K[i], S[i], h); // Use function h
        let temp = *d;
        *d = *c;
        *c = *b;
        *b = *a;
        *a = temp;
    }

    // 6. Round 4
    // Word index pattern: (7*i) % 16
    for i_ in 48..64 {
        let g = (7 * i_) % 16;
        *a = step(*a, *b, *c, *d, x[g], K[i_], S[i_], i); // Use function i
        let temp = *d;
        *d = *c;
        *c = *b;
        *b = *a;
        *a = temp;
    }

    // 7. Add back to original state
    *a = a.wrapping_add(a0);
    *b = b.wrapping_add(b0);
    *c = c.wrapping_add(c0);
    *d = d.wrapping_add(d0);
}

pub fn hash<T: AsRef<[u8]>>(input: T) -> [u8; 16] {
    // state
    let mut a: u32 = 0x67452301;
    let mut b: u32 = 0xefcdab89;
    let mut c: u32 = 0x98badcfe;
    let mut d: u32 = 0x10325476;

    let input = input.as_ref();
    let original_bit_length = (input.len() as u64) * 8;

    let mut padded = input.to_vec();
    padded.push(0x80);

    while padded.len() % 64 != 56 {
        padded.push(0);
    }

    padded.extend_from_slice(&original_bit_length.to_le_bytes());

    for chunk in padded.chunks_exact(64) {
        let block: &[u8; 64] = chunk.try_into().unwrap();
        proc_block(block, &mut a, &mut b, &mut c, &mut d);
    }

    // Convert back to bytes (Little Endian)
    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&a.to_le_bytes());
    result[4..8].copy_from_slice(&b.to_le_bytes());
    result[8..12].copy_from_slice(&c.to_le_bytes());
    result[12..16].copy_from_slice(&d.to_le_bytes());

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"hello world!", "fc3ff98e8c6a0d3087d515c0473f8677")]
    fn test_hash(#[case] input: &[u8], #[case] expected_hex: &str) {
        let result_hex: String = hash(input).iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(result_hex, expected_hex)
    }
}
