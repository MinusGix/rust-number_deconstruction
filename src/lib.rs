pub fn transform_u16_to_u8_array (val: u16) -> [u8; 2] {
    [
        (val >> 8) as u8, // first 8bits
        (val & 0b00000000_11111111) as u8 // last eight bits
    ]
}
pub fn transform_u32_to_u16_array (val: u32) -> [u16; 2] {
    [
        (val >> 16) as u16, // first 16bits
        (val & 0b0000000000000000_1111111111111111) as u16 // last 16bits
    ]
}
pub fn transform_u32_to_u8_array (val: u32) -> [u8; 4] {
    let [high, low] = transform_u32_to_u16_array(val);
    let [high1, high2] = transform_u16_to_u8_array(high);
    let [low1, low2] = transform_u16_to_u8_array(low);
    [high1, high2, low1, low2]
}
pub fn transform_u64_to_u32_array (val: u64) -> [u32; 2] {
    [
        (val >> 32) as u32, // first 32 bits
        (val & 0b00000000000000000000000000000000_11111111111111111111111111111111) as u32 // last 32 bits
    ]
}
pub fn transform_u64_to_u16_array (val: u64) -> [u16; 4] {
    let [high, low] = transform_u64_to_u32_array(val);
    let [high1, high2] = transform_u32_to_u16_array(high);
    let [low1, low2] = transform_u32_to_u16_array(low);
    [high1, high2, low1, low2]
}
pub fn transform_u64_to_u8_array (val: u64) -> [u8; 8] {
    let [highest, high, low, lowest] = transform_u64_to_u16_array(val);
    let [highest1, highest2] = transform_u16_to_u8_array(highest);
    let [high1, high2] = transform_u16_to_u8_array(high);
    let [low1, low2] = transform_u16_to_u8_array(low);
    let [lowest1, lowest2] = transform_u16_to_u8_array(lowest);
    [highest1, highest2, high1, high2, low1, low2, lowest1, lowest2]
}

pub fn transform_2u8_to_u16 (high: u8, low: u8) -> u16 {
    (u16::from(high) << 8) | u16::from(low)
}
pub fn transform_2u16_to_u32 (high: u16, low: u16) -> u32 {
    (u32::from(high) << 16) | u32::from(low)
}
pub fn transform_4u8_to_u32 (highest: u8, high: u8, low: u8, lowest: u8) -> u32 {
    transform_2u16_to_u32(transform_2u8_to_u16(highest, high), transform_2u8_to_u16(low, lowest))
}
pub fn transform_2u32_to_u64 (high: u32, low: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low)
}
pub fn transform_4u16_to_u64 (highest: u16, high: u16, low: u16, lowest: u16) -> u64 {
    transform_2u32_to_u64(transform_2u16_to_u32(highest, high), transform_2u16_to_u32(low, lowest))
}
pub fn transform_8u8_to_u64 (v1: u8, v2: u8, v3: u8, v4: u8, v5: u8, v6: u8, v7: u8, v8: u8) -> u64 {
    transform_4u16_to_u64(
        transform_2u8_to_u16(v1, v2),
        transform_2u8_to_u16(v3, v4),
        transform_2u8_to_u16(v5, v6),
        transform_2u8_to_u16(v7, v8)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_u16_to_u8_array () {
        assert_eq!(transform_u16_to_u8_array(624), [0b0000_0010, 0b0111_0000]);
        assert_eq!(transform_u16_to_u8_array(2495), [0b0000_1001, 0b1011_1111]);
    }
    #[test]
    fn test_transform_u32_to_u16_array () {
        assert_eq!(
            transform_u32_to_u16_array(0x77777778),
            [0b0111_0111_0111_0111, 0b0111_0111_0111_1000]
        )
    }
    #[test]
    fn test_transform_u32_to_u8_array () {
        assert_eq!(
            transform_u32_to_u8_array(0x77777778),
            [0b0111_0111, 0b0111_0111, 0b0111_0111, 0b0111_1000]
        )
    }
    #[test]
    fn test_transform_u64_to_u32_array () {
        assert_eq!(
            transform_u64_to_u32_array(8673372035854775807),
            [0b01111000_01011110_00000001_11101001, 0b11000001_11101110_00110101_11111111]
        )
    }
    #[test]
    fn test_transform_u64_to_u16_array () {
        assert_eq!(
            transform_u64_to_u16_array(8673372035854775807),
            [0b01111000_01011110, 0b00000001_11101001, 0b11000001_11101110, 0b00110101_11111111]
        )
    }
    #[test]
    fn test_transform_u64_to_u8_array () {
        assert_eq!(
            transform_u64_to_u8_array(8673372035854775807),
            [0b01111000, 0b01011110, 0b00000001, 0b11101001, 0b11000001, 0b11101110, 0b00110101, 0b11111111]
        )
    }

    #[test]
    fn test_transform_2u8_to_u16 () {
        assert_eq!(
            transform_2u8_to_u16(0b1101_1001, 0b1101_1111),
            0b11011001_1101_1111
        );
    }

    #[test]
    fn test_transform_2u16_to_u32 () {
        assert_eq!(
            transform_2u16_to_u32(0b11101100_10111101, 0b00110110_00011110),
            0b11101100_10111101_00110110_00011110
        );
    }

    #[test]
    fn test_transform_4u8_to_u32 () {
        assert_eq!(
            transform_4u8_to_u32(0b11101100, 0b10111101, 0b00110110, 0b00011110),
            0b11101100_10111101_00110110_00011110
        );
    }

    #[test]
    fn test_transform_2u32_to_u64 () {
        assert_eq!(
            transform_2u32_to_u64(
                0b11101100_10111101_00110110_00011110,
                0b00100101_11110110_01110010_00110000
            ),
            0b11101100_10111101_00110110_00011110_00100101_11110110_01110010_00110000
        );
    }

    #[test]
    fn test_transform_4u16_to_u64 () {
        assert_eq!(
            transform_4u16_to_u64(
                0b11101100_10111101, 0b00110110_00011110,
                0b00100101_11110110, 0b01110010_00110000
            ),
            0b11101100_10111101_00110110_00011110_00100101_11110110_01110010_00110000
        );
    }

    #[test]
    fn test_transform_8u8_to_u64 () {
        assert_eq!(
            transform_8u8_to_u64(
                0b11101100, 0b10111101, 0b00110110, 0b00011110,
                0b00100101, 0b11110110, 0b01110010, 0b00110000
            ),
            0b11101100_10111101_00110110_00011110_00100101_11110110_01110010_00110000
        );
    }
}