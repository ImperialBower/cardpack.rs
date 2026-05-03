use alloc::string::String;

pub struct Bit;

impl Bit {
    pub const RANK_FLAG_FILTER: usize = 0x1FFF_0000; // 536805376 aka 0b00011111_11111111_00000000_00000000
    pub const RANK_FLAG_SHIFT: usize = 16;
    pub const RANK_PRIME_FILTER: usize = 0b0011_1111;
    pub const RANK_NUMBER_FILTER: usize = 0b1111_00000000;

    /// Binary filter for `CardNumber` `Suit` flags.
    /// 00000000 00000000 11110000 00000000
    pub const SUIT_FLAG_FILTER: usize = 0b1111_0000_0000_0000; // 61440 aka 0xF000
    pub const SUIT_SHORT_MASK: usize = 0b1111;
    pub const SUIT_FLAG_SHIFT: usize = 11;

    #[must_use]
    pub const fn ckc_bits(ckc: usize) -> usize {
        ckc & Self::RANK_FLAG_FILTER
    }

    #[must_use]
    pub fn ckc_prime(ckc: usize) -> usize {
        ckc & Self::RANK_PRIME_FILTER
    }

    #[must_use]
    pub fn ckc_shift8(ckc: usize) -> usize {
        ckc & Self::RANK_NUMBER_FILTER
    }

    #[must_use]
    pub fn only_suit_flags(ckc: usize) -> usize {
        ckc & Self::SUIT_FLAG_FILTER
    }

    #[must_use]
    pub fn strip_suit_flags(ckc: usize) -> usize {
        ckc & !Self::SUIT_FLAG_FILTER
    }

    /// These utility methods come from `pkcore`, a library that is currently a work in progress.
    #[must_use]
    pub fn string(ckc: usize) -> String {
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

    #[test]
    fn ckc_bits() {
        // RANK_FLAG_FILTER = 0x1FFF_0000 (bits 16-28)
        let ckc = 0x0001_0000_usize; // one bit in filter range
        assert_eq!(Bit::ckc_bits(ckc), 0x0001_0000);
        assert_ne!(Bit::ckc_bits(ckc), 0);
        assert_ne!(Bit::ckc_bits(ckc), 1);
        // bits outside filter range are masked out
        assert_eq!(Bit::ckc_bits(0x0000_FFFF), 0);
        // all-ones: only filter bits survive
        assert_eq!(Bit::ckc_bits(usize::MAX), Bit::RANK_FLAG_FILTER);
    }

    #[test]
    fn ckc_prime() {
        // RANK_PRIME_FILTER = 0b0011_1111 = 0x3F (bits 0-5)
        let ckc = 0xFF_usize;
        assert_eq!(Bit::ckc_prime(ckc), 0x3F);
        assert_ne!(Bit::ckc_prime(ckc), 0);
        assert_ne!(Bit::ckc_prime(ckc), 1);
        // bits outside the prime filter are masked out
        assert_eq!(Bit::ckc_prime(0xFF00), 0);
        // all-ones: only filter bits survive
        assert_eq!(Bit::ckc_prime(usize::MAX), Bit::RANK_PRIME_FILTER);
    }

    #[test]
    fn ckc_shift8() {
        // RANK_NUMBER_FILTER = 0b1111_0000_0000 = 0x0F00 (bits 8-11)
        let ckc = 0xFFFF_usize;
        assert_eq!(Bit::ckc_shift8(ckc), 0x0F00);
        assert_ne!(Bit::ckc_shift8(ckc), 0);
        assert_ne!(Bit::ckc_shift8(ckc), 1);
        // bits outside the filter are masked out
        assert_eq!(Bit::ckc_shift8(0x00FF), 0);
        // all-ones: only filter bits survive
        assert_eq!(Bit::ckc_shift8(usize::MAX), Bit::RANK_NUMBER_FILTER);
    }

    #[test]
    fn strip_suit_flags() {
        // SUIT_FLAG_FILTER = 0xF000 (bits 12-15)
        // strip_suit_flags removes those bits using &!filter
        let ckc = 0xFFFF_usize;
        let expected = 0x0FFF_usize;
        assert_eq!(Bit::strip_suit_flags(ckc), expected);
        assert_ne!(Bit::strip_suit_flags(ckc), 0);
        assert_ne!(Bit::strip_suit_flags(ckc), 1);
        // pure suit bits: stripped to zero
        assert_eq!(Bit::strip_suit_flags(Bit::SUIT_FLAG_FILTER), 0);
        // complement to only_suit_flags: together they reconstruct the original
        assert_eq!(Bit::only_suit_flags(ckc) | Bit::strip_suit_flags(ckc), ckc);
    }
}
