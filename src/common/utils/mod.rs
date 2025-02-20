pub struct Bit;

impl Bit {
    pub const RANK_FLAG_FILTER: u32 = 0x1FFF_0000; // 536805376 aka 0b00011111_11111111_00000000_00000000
    pub const RANK_FLAG_SHIFT: u32 = 16;
    pub const RANK_PRIME_FILTER: u32 = 0b0011_1111;
    pub const RANK_NUMBER_FILTER: u32 = 0b1111_00000000;

    /// Binary filter for `CardNumber` `Suit` flags.
    /// 00000000 00000000 11110000 00000000
    pub const SUIT_FLAG_FILTER: u32 = 0b1111_0000_0000_0000; // 61440 aka 0xF000
    pub const SUIT_SHORT_MASK: u32 = 0b1111;
    pub const SUIT_FLAG_SHIFT: u32 = 11;

    #[must_use]
    pub fn ckc_bits(ckc: u32) -> u32 {
        ckc & Bit::RANK_FLAG_FILTER
    }

    #[must_use]
    pub fn ckc_prime(ckc: u32) -> u32 {
        ckc & Bit::RANK_PRIME_FILTER
    }

    #[must_use]
    pub fn ckc_shift8(ckc: u32) -> u32 {
        ckc & Bit::RANK_NUMBER_FILTER
    }

    #[must_use]
    pub fn only_suit_flags(ckc: u32) -> u32 {
        ckc & Bit::SUIT_FLAG_FILTER
    }

    #[must_use]
    pub fn strip_suit_flags(ckc: u32) -> u32 {
        ckc & !Bit::SUIT_FLAG_FILTER
    }

    /// These utility methods come from `pkcore`, a library that is currently a work in progress.
    #[must_use]
    pub fn string(ckc: u32) -> String {
        let b = format!("{ckc:b}");
        // OK, let's take a moment to really stan on the rust std libraries. The fmt
        // [Fill/Alignment](https://doc.rust-lang.org/std/fmt/#fillalignment) is FIRE!
        let b = format!("{b:0>32}");
        let mut bit_string = String::with_capacity(34);

        for (i, c) in b.chars().enumerate() {
            bit_string.push(c);
            if i % 8 == 7 && i % 31 != 0 {
                bit_string.push(' ');
            }
        }
        bit_string
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod types__rank__tests {
    use super::*;

    #[test]
    fn string() {
        let ckc = 0b0000_0000_0000_0000_0000_0000_0000_0000;
        let expected = "00000000 00000000 00000000 00000000";
        assert_eq!(Bit::string(ckc), expected);

        let ckc = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let expected = "11111111 11111111 11111111 11111111";
        assert_eq!(Bit::string(ckc), expected);

        let ckc = 0b1010_1010_1010_1010_1010_1010_1010_1010;
        let expected = "10101010 10101010 10101010 10101010";
        assert_eq!(Bit::string(ckc), expected);
    }

    #[test]
    fn only_suit_flags() {
        let ckc = 0b1111_1111_1111_1111_1111_1111_1111_1111;
        let exp = 0b0000_0000_0000_0000_1111_0000_0000_0000;
        assert_eq!(Bit::only_suit_flags(ckc), exp);
    }
}
