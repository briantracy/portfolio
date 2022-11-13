

pub fn interpret_big_endian_word(b1: u8, b2: u8, b3: u8, b4: u8) -> i32 {
    (b1 as i32) << 24 |
    (b2 as i32) << 16 |
    (b3 as i32) << 8  |
    (b4 as i32) << 0
}

pub fn interpret_big_endian_slice(bytes: &[u8]) -> i32 {
    assert_eq!(bytes.len(), 4);
    interpret_big_endian_word(*&bytes[0], *&bytes[1], *&bytes[2], *&bytes[3])
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word() {
        assert_eq!(interpret_big_endian_word(0, 0, 0, 0), 0);
        assert_eq!(interpret_big_endian_word(0, 0, 0, 1), 1);
        assert_eq!(interpret_big_endian_word(0, 0, 0, 0xFF), 255);
        assert_eq!(interpret_big_endian_word(0xFF, 0xFF, 0xFF, 0xFF), -1);
        assert_eq!(interpret_big_endian_word(0b0010_1001, 0b0100_1010, 0b0010_0100, 0b1000_0010), 692724866);
    }

    #[test]
    fn valid_slice() {
        assert_eq!(interpret_big_endian_slice(&[0, 0, 0, 0]), 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn invalid_slice() {
        assert_eq!(interpret_big_endian_slice(&[0, 0, 0, 0, 0, 0, 0]), 0);
    }
}
