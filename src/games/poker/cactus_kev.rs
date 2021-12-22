use bitvec::prelude::*;
use std::fmt::{Display, Formatter};
use wyz::fmt::FmtForward;

/// A `CactusKev` Card representation is made up of four bites.
pub type CactusKev = BitArray<Msb0, [u8; 4]>;

#[allow(clippy::module_name_repetitions)]
pub struct CactusKevCard<'a> {
    pub card: &'a CactusKev,
}

impl<'a> CactusKevCard<'a> {
    #[must_use]
    pub fn new(card: &'a CactusKev) -> Self {
        Self { card }
    }

    // pub fn blank() -> CactusKevCard {
    //     let card: CactusKev = BitArray::zeroed();
    //     CactusKevCard::new(&card);
    // }

    #[must_use]
    pub fn dump(&self) -> String {
        let mut word_string = String::with_capacity(35);
        for (idx, word) in self.card.as_bitslice().chunks(32).enumerate() {
            let start_bit = idx * 32;
            let bits = start_bit..start_bit + 32;
            for (bit, idx) in word.iter().by_val().zip(bits) {
                word_string.push_str(if bit { "1" } else { "0" });
                if idx % 8 == 7 && idx % 32 != 31 {
                    word_string.push(' ');
                }
            }
        }
        word_string
    }
}

/// [Module ``std::fmt``](https://doc.rust-lang.org/std/fmt/)
impl<'a> Display for CactusKevCard<'a> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = fmt.debug_list();

        let mut mark_string = String::with_capacity(35);
        mark_string.push_str("xxxAKQJT 98765432 CDHSrrrr xxpppppp");

        out.entry(&(self.dump()).fmt_display());
        out.entry(&(&mark_string).fmt_display());
        out.finish()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod cactus_kev_tests {
    use super::*;

    #[test]
    fn len() {
        let card: CactusKev = BitArray::zeroed();
        assert_eq!(card.len(), 32);
    }
}
