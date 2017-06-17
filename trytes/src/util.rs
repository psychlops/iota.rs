use constants::*;
use mappings::*;

pub fn tryte_to_trits(trit: char) -> [Trit; TRITS_PER_TRYTE] {
    TRYTE_TO_TRITS_MAPPINGS[TRYTE_ALPHABET.find(trit).unwrap()]
}

pub fn trits_to_byte(trits: &[Trit]) -> i8 {
    assert!(trits.len() <= TRITS_PER_BYTE);

    let mut value: Trit = 0;
    for j in trits {
        value = value * RADIX + j;
    }

    value as i8
}

pub fn byte_to_trits(b: i8) -> Vec<Trit> {
    let mut out: Vec<Trit> = Vec::new();

    let bpos: usize = (if b < 0 {
                           (b as isize) + (HASH_LENGTH as isize)
                       } else {
                           b as isize
                       }) as usize;

    out.extend_from_slice(&BYTE_TO_TRITS_MAPPINGS[bpos]);
    out
}

pub fn trits_to_char(t: &[Trit]) -> char {
    match TRYTE_TO_TRITS_MAPPINGS.iter().position(|&x| x == t) {
        Some(p) => TRYTE_ALPHABET.chars().nth(p).unwrap(),
        None => '-',
    }
}

#[cfg(test)]
mod test {
    use trinary::*;
    use super::*;
    #[test]
    fn test_char_to_trit() {
        for (i, c) in TRYTE_ALPHABET.chars().enumerate() {
            let ts = TRYTE_TO_TRITS_MAPPINGS[i];
            let m = tryte_to_trits(c);

            assert_eq!(ts, m);

        }
    }


    #[test]
    fn bytes_to_trits() {
        let bytes: [i8; 6] = [20, 25, -14, -2, 83, 1];
        let exp: [Trit; 27] = [
            0,
            1,
            -1,
            1,
            -1,
            0,
            1,
            0,
            -1,
            1,
            0,
            -1,
            1,
            1,
            1,
            0,
            0,
            0,
            -1,
            1,
            1,
            0,
            0,
            1,
            -1,
            0,
            1,
        ];

        let trinary = Trinary::new(bytes.iter().cloned().collect(), 27);
        let out = trinary.trits();

        assert_eq!(&exp, out.as_slice());

    }
}
