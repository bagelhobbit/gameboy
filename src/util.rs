/// Returns the bits of a `u8` as an array
pub fn get_as_bits(byte: u8) -> [u8; 8] {
    let mut bits = [0; 8];
    bits[0] = (byte & 0b1000_0000) >> 7;
    bits[1] = (byte & 0b0100_0000) >> 6;
    bits[2] = (byte & 0b0010_0000) >> 5;
    bits[3] = (byte & 0b0001_0000) >> 4;
    bits[4] = (byte & 0b0000_1000) >> 3;
    bits[5] = (byte & 0b0000_0100) >> 2;
    bits[6] = (byte & 0b0000_0010) >> 1;
    bits[7] = byte & 0b0000_0001;

    bits
}

/// Returns the top 4 bits of a `u8`
pub fn get_upper_bits(byte: u8) -> u8 {
    (byte & 0b1111_0000) >> 4
}

/// Returns the lower 4 bits of a `u8`
pub fn get_lower_bits(byte: u8) -> u8 {
    byte & 0b0000_1111
}

/// Returns the top 8 bits of a `u16`
pub fn get_upper_byte(data: u16) -> u8 {
    (data >> 8) as u8
}

/// Returns the lower 8 bits of a `u16`
pub fn get_lower_byte(data: u16) -> u8 {
    (data & 0x00FF) as u8
}

/// Combines two `u8`s to create a new `u16`
pub fn combine_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) + low as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_as_bits() {
        let byte = 0b1010_1010;
        assert_eq!(get_as_bits(byte), [1, 0, 1, 0, 1, 0, 1, 0])
    }

    #[test]
    fn test_get_upper_bits() {
        let byte = 0xF5;
        assert_eq!(get_upper_bits(byte), 0x0F)
    }

    #[test]
    fn test_get_lower_bits() {
        let byte = 0xF5;
        assert_eq!(get_lower_bits(byte), 0x05)
    }

    #[test]
    fn test_get_upper_byte() {
        let data = 0xABCD;
        assert_eq!(get_upper_byte(data), 0xAB);
    }

    #[test]
    fn test_get_lower_byte() {
        let data = 0xABCD;
        assert_eq!(get_lower_byte(data), 0xCD);
    }

    #[test]
    fn test_combine_bytes() {
        assert_eq!(combine_bytes(0xAB, 0xCD), 0xABCD)
    }
}
