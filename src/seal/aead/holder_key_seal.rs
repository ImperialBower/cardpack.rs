//! `HolderKeySeal<D>`: dealer mode (holds the master) or verifier mode (no
//! secret; can only `unseal` with a published token).

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use core::hash::Hash;

use chacha20poly1305::{AeadInOut, KeyInit, Tag, XChaCha20Poly1305, XNonce};
use rand::Rng;

use crate::basic::types::card::Card;
use crate::basic::types::ordinal::{Codebook, Ordinal};
use crate::basic::types::pile::Pile;
use crate::basic::types::traits::DeckedBase;
use crate::seal::adapter::Seal;
use crate::seal::aead::error::AeadSealError;
use crate::seal::aead::keys::{CardKey, DealKey};
use crate::seal::aead::sealed_bytes::{Custody, SealedBytes};
use crate::seal::slot::SlotId;
use crate::seal::slot_pile::SlotPile;

/// Associated-data prefix. Part of the frozen `v1` format:
/// `AD = TAG_AD || u16 BE name_len || deck_name || u16 BE slot || context`.
pub const TAG_AD: &[u8] = b"cardpack/seal-aead/v1/ad";

/// The scheme, with or without the secret. Never stored inside any pile.
pub struct HolderKeySeal<D: DeckedBase> {
    /// `None` in verifier mode.
    master: Option<DealKey>,
    context: Vec<u8>,
    codebook: Codebook<D>,
    deck_name: String,
}

impl<D: DeckedBase> HolderKeySeal<D> {
    /// Dealer mode: can seal, can mint tokens, can unseal.
    pub fn dealer(master: DealKey, context: impl AsRef<[u8]>) -> Self {
        Self::build(Some(master), context)
    }

    /// Verifier mode: holds no secret. Can `unseal` with a published token;
    /// cannot seal, cannot mint.
    pub fn verifier(context: impl AsRef<[u8]>) -> Self {
        Self::build(None, context)
    }

    fn build(master: Option<DealKey>, context: impl AsRef<[u8]>) -> Self {
        Self {
            master,
            context: context.as_ref().to_vec(),
            codebook: Codebook::new(),
            deck_name: D::deck_name(),
        }
    }

    /// `true` in dealer mode.
    #[must_use]
    pub fn is_dealer(&self) -> bool {
        self.master.is_some()
    }

    /// The context this scheme binds into every sealed card.
    #[must_use]
    pub fn context(&self) -> &[u8] {
        &self.context
    }

    /// The token that opens exactly `slot`.
    ///
    /// # Errors
    ///
    /// [`AeadSealError::NoMasterKey`] in verifier mode.
    pub fn token_for(&self, slot: SlotId) -> Result<CardKey, AeadSealError> {
        let master = self.master.as_ref().ok_or(AeadSealError::NoMasterKey)?;
        Ok(master.slot_key(&self.deck_name, slot.get()))
    }

    /// [`token_for`](Self::token_for) over many slots, in the given order.
    ///
    /// # Errors
    ///
    /// [`AeadSealError::NoMasterKey`] in verifier mode.
    pub fn tokens_for(
        &self,
        slots: impl IntoIterator<Item = SlotId>,
    ) -> Result<Vec<(SlotId, CardKey)>, AeadSealError> {
        slots
            .into_iter()
            .map(|s| self.token_for(s).map(|k| (s, k)))
            .collect()
    }

    pub(crate) fn codebook(&self) -> &Codebook<D> {
        &self.codebook
    }

    /// The dealer's one-call setup: shuffle a copy of `pile` with `rng`,
    /// assign slots `0..n` to the **shuffled** order, seal each card into its
    /// slot. Returns the shoe (a `SlotPile` of names) and the ledger
    /// (`Custody` of public bytes). Slot ≠ ordinal by construction — sealing
    /// a sorted deck slot-by-slot would make every slot name its own card.
    ///
    /// `rng` **must** be a CSPRNG: it draws the shuffle and every nonce.
    ///
    /// # Errors
    ///
    /// [`AeadSealError::NoMasterKey`] in verifier mode;
    /// [`AeadSealError::PileTooLong`] above `u16::MAX` cards;
    /// [`AeadSealError::CardNotInDeck`] for a card outside the vocabulary.
    pub fn deal(
        &self,
        pile: &Pile<D>,
        rng: &mut dyn Rng,
    ) -> Result<(SlotPile, Custody), AeadSealError>
    where
        D: Default + Ord + Copy + Hash,
    {
        if !self.is_dealer() {
            return Err(AeadSealError::NoMasterKey);
        }
        let n = u16::try_from(pile.len()).map_err(|_| AeadSealError::PileTooLong(pile.len()))?;
        let shuffled = pile.shuffled_with_rng(rng);
        let mut custody = Custody::new();
        for (i, card) in shuffled.cards().iter().enumerate() {
            // i < n <= u16::MAX by the check above.
            let slot = SlotId::new(u16::try_from(i).unwrap_or(u16::MAX));
            let sealed = self.seal(*card, slot, rng)?;
            // Slots 0..n are distinct by construction; a duplicate is a bug.
            custody
                .insert(slot, sealed)
                .map_err(|_| AeadSealError::PileTooLong(pile.len()))?;
        }
        Ok((SlotPile::new(n), custody))
    }

    /// `TAG_AD || u16 BE name_len || deck_name || u16 BE slot || context`.
    fn associated_data(&self, slot: SlotId) -> Vec<u8> {
        let name = self.deck_name.as_bytes();
        // Deck names are a few characters; a >64 KiB name is not a deck.
        let name_len = u16::try_from(name.len()).unwrap_or(u16::MAX);
        let mut ad = Vec::with_capacity(TAG_AD.len() + 2 + name.len() + 2 + self.context.len());
        ad.extend_from_slice(TAG_AD);
        ad.extend_from_slice(&name_len.to_be_bytes());
        ad.extend_from_slice(name);
        ad.extend_from_slice(&slot.get().to_be_bytes());
        ad.extend_from_slice(&self.context);
        ad
    }

    pub(crate) fn deck_name(&self) -> &str {
        &self.deck_name
    }
}

impl<D: DeckedBase> Seal<D> for HolderKeySeal<D> {
    type Sealed = SealedBytes;
    type Token = CardKey;
    type Error = AeadSealError;

    /// Ordinal → 2 bytes; fresh 24-byte nonce from `rng` (**must** be a
    /// CSPRNG); XChaCha20-Poly1305 under `K_slot` with the AD above.
    fn seal(
        &self,
        card: Card<D>,
        slot: SlotId,
        rng: &mut dyn Rng,
    ) -> Result<SealedBytes, AeadSealError> {
        let master = self.master.as_ref().ok_or(AeadSealError::NoMasterKey)?;
        let ordinal = self
            .codebook
            .ordinal(&card)
            .ok_or_else(|| AeadSealError::CardNotInDeck(card.base().to_string()))?;
        let key = master.slot_key(&self.deck_name, slot.get());
        let ad = self.associated_data(slot);

        let mut nonce = [0u8; 24];
        rng.fill_bytes(&mut nonce);
        let mut buf = ordinal.get().to_be_bytes();

        let cipher =
            XChaCha20Poly1305::new_from_slice(key.as_bytes()).map_err(|_| AeadSealError::Unseal)?;
        let tag = cipher
            .encrypt_inout_detached(&XNonce::from(nonce), &ad, (&mut buf[..]).into())
            .map_err(|_| AeadSealError::Unseal)?;
        Ok(SealedBytes::new(nonce, buf, tag.into()))
    }

    /// Decrypt under the *token*; the AD is recomputed from `(slot, context)`.
    /// Never touches the master key, so it works in verifier mode.
    fn unseal(
        &self,
        sealed: &SealedBytes,
        slot: SlotId,
        token: &CardKey,
    ) -> Result<Card<D>, AeadSealError> {
        let ad = self.associated_data(slot);
        let cipher = XChaCha20Poly1305::new_from_slice(token.as_bytes())
            .map_err(|_| AeadSealError::Unseal)?;
        let mut buf = *sealed.ct();
        cipher
            .decrypt_inout_detached(
                &XNonce::from(*sealed.nonce()),
                &ad,
                (&mut buf[..]).into(),
                &Tag::from(*sealed.tag()),
            )
            .map_err(|_| AeadSealError::Unseal)?;
        let ord = u16::from_be_bytes(buf);
        self.codebook
            .card(Ordinal::new(ord))
            .ok_or(AeadSealError::InvalidOrdinal(ord))
    }
}

impl<D: DeckedBase> fmt::Debug for HolderKeySeal<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HolderKeySeal")
            .field(
                "mode",
                &if self.is_dealer() {
                    "dealer"
                } else {
                    "verifier"
                },
            )
            .field("deck", &self.deck_name)
            .field("context_len", &self.context.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod seal__aead__holder_key_seal_tests {
    use super::*;
    use crate::basic::decks::french::French;
    use crate::basic::decks::skat::Skat;
    use crate::basic::decks::standard52::Standard52;
    use alloc::vec::Vec;

    fn master() -> DealKey {
        DealKey::from_bytes([0x01; 32])
    }

    #[test]
    fn dealer__is_dealer_and_verifier_is_not() {
        assert!(HolderKeySeal::<Standard52>::dealer(master(), b"ctx").is_dealer());
        assert!(!HolderKeySeal::<Standard52>::verifier(b"ctx").is_dealer());
    }

    #[test]
    fn token_for__deterministic() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"ctx");
        let b = HolderKeySeal::<Standard52>::dealer(master(), b"ctx");
        assert_eq!(
            a.token_for(SlotId::new(7)).unwrap().to_bytes(),
            b.token_for(SlotId::new(7)).unwrap().to_bytes()
        );
    }

    #[test]
    fn token_for__differs_per_slot() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"ctx");
        assert_ne!(
            a.token_for(SlotId::new(7)).unwrap().to_bytes(),
            a.token_for(SlotId::new(8)).unwrap().to_bytes()
        );
    }

    #[test]
    fn token_for__differs_per_deck() {
        let f = HolderKeySeal::<French>::dealer(master(), b"ctx");
        let s = HolderKeySeal::<Skat>::dealer(master(), b"ctx");
        assert_ne!(
            f.token_for(SlotId::new(0)).unwrap().to_bytes(),
            s.token_for(SlotId::new(0)).unwrap().to_bytes()
        );
    }

    /// The context binds the *ciphertext* (through the AD), not the key.
    /// Frozen: a token minted for one context opens the same slot under
    /// another context only if the sealed bytes were made under that one.
    #[test]
    fn token_for__independent_of_context() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"ctx-1");
        let b = HolderKeySeal::<Standard52>::dealer(master(), b"ctx-2");
        assert_eq!(
            a.token_for(SlotId::new(7)).unwrap().to_bytes(),
            b.token_for(SlotId::new(7)).unwrap().to_bytes()
        );
    }

    #[test]
    fn token_for__matches_golden_slot_key() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"test");
        assert_eq!(
            a.token_for(SlotId::new(7)).unwrap().to_bytes(),
            master().slot_key("Standard 52", 7).to_bytes()
        );
    }

    #[test]
    fn verifier__cannot_mint_tokens() {
        let v = HolderKeySeal::<Standard52>::verifier(b"ctx");
        assert_eq!(
            v.token_for(SlotId::new(7)).unwrap_err(),
            AeadSealError::NoMasterKey
        );
        assert_eq!(
            v.tokens_for([SlotId::new(1)]).unwrap_err(),
            AeadSealError::NoMasterKey
        );
    }

    #[test]
    fn tokens_for__keeps_order() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"ctx");
        let t: Vec<(SlotId, CardKey)> = a.tokens_for([SlotId::new(3), SlotId::new(1)]).unwrap();
        assert_eq!(t[0].0, SlotId::new(3));
        assert_eq!(t[1].0, SlotId::new(1));
        assert_eq!(
            t[0].1.to_bytes(),
            a.token_for(SlotId::new(3)).unwrap().to_bytes()
        );
    }

    // ── Story 3: the Seal impl ──────────────────────────────────────────

    use crate::basic::types::card::Card;
    use crate::basic::types::traits::Decked;
    use crate::seal::adapter::Seal;
    use crate::seal::plaintext::seal_roundtrip;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    /// A test-only RNG that yields one constant byte forever. Used for the
    /// golden vector. NEVER a model for production: a constant nonce breaks
    /// the scheme (EPIC-04b gotcha 1).
    struct ConstRng(u8);
    impl rand::TryRng for ConstRng {
        type Error = core::convert::Infallible;
        fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
            Ok(u32::from_ne_bytes([self.0; 4]))
        }
        fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
            Ok(u64::from_ne_bytes([self.0; 8]))
        }
        fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
            dst.fill(self.0);
            Ok(())
        }
    }

    fn dealer52() -> HolderKeySeal<Standard52> {
        HolderKeySeal::<Standard52>::dealer(master(), b"test")
    }

    fn ace_of_spades() -> Card<Standard52> {
        Standard52::deck().cards()[0]
    }

    macro_rules! roundtrip_all_decks {
        ($($deck:ident),* $(,)?) => {{
            let mut seed = 0u64;
            $(
                seed += 1;
                let scheme = HolderKeySeal::<crate::prelude::$deck>::dealer(master(), b"law");
                seal_roundtrip::<crate::prelude::$deck, _>(
                    &scheme,
                    |slot| scheme.token_for(slot).unwrap(),
                    &mut StdRng::seed_from_u64(seed),
                );
            )*
        }};
    }

    #[test]
    fn hks__roundtrip_law_every_shipped_deck() {
        roundtrip_all_decks!(
            Canasta,
            Dashavatara,
            Euchre24,
            Euchre32,
            French,
            Mughal,
            Pinochle,
            Short,
            Skat,
            Spades,
            Standard52,
            Tarot,
            Tiny
        );
        #[cfg(feature = "yaml")]
        roundtrip_all_decks!(Razz);
    }

    #[test]
    fn hks__golden_vector() {
        // Python (pycryptodome): seal(master=b"\x01"*32, "Standard 52", slot 7,
        // context b"test", ordinal 0, nonce b"\x02"*24).hex()
        let scheme = dealer52();
        assert_eq!(
            scheme.codebook().ordinal(&ace_of_spades()).unwrap().get(),
            0
        );
        let sealed = scheme
            .seal(ace_of_spades(), SlotId::new(7), &mut ConstRng(0x02))
            .unwrap();
        let hex = sealed
            .to_bytes()
            .iter()
            .fold(alloc::string::String::new(), |mut s, b| {
                use core::fmt::Write;
                let _ = write!(s, "{b:02x}");
                s
            });
        assert_eq!(
            hex,
            "020202020202020202020202020202020202020202020202b06baa35ec141d5425c389a624cb2263d2a3"
        );
    }

    #[test]
    fn hks__wrong_token_errors() {
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let bad = CardKey::from_bytes([0xee; 32]);
        assert_eq!(
            scheme.unseal(&sealed, SlotId::new(7), &bad).unwrap_err(),
            AeadSealError::Unseal
        );
    }

    #[test]
    fn hks__token_for_other_slot_errors() {
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let other = scheme.token_for(SlotId::new(8)).unwrap();
        assert_eq!(
            scheme.unseal(&sealed, SlotId::new(7), &other).unwrap_err(),
            AeadSealError::Unseal
        );
        // The right token at the wrong slot also fails: the slot is in the AD.
        let right = scheme.token_for(SlotId::new(7)).unwrap();
        assert_eq!(
            scheme.unseal(&sealed, SlotId::new(8), &right).unwrap_err(),
            AeadSealError::Unseal
        );
    }

    #[test]
    fn hks__wrong_context_errors() {
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let token = scheme.token_for(SlotId::new(7)).unwrap();
        let other = HolderKeySeal::<Standard52>::verifier(b"other");
        assert_eq!(
            other.unseal(&sealed, SlotId::new(7), &token).unwrap_err(),
            AeadSealError::Unseal
        );
    }

    #[test]
    fn hks__wrong_deck_errors() {
        let french = HolderKeySeal::<French>::dealer(master(), b"test");
        let skat = HolderKeySeal::<Skat>::dealer(master(), b"test");
        let card = French::deck().cards()[2]; // a real card, also in Skat
        let sealed = french
            .seal(card, SlotId::new(7), &mut StdRng::seed_from_u64(1))
            .unwrap();
        let skat_token = skat.token_for(SlotId::new(7)).unwrap();
        assert_eq!(
            skat.unseal(&sealed, SlotId::new(7), &skat_token)
                .unwrap_err(),
            AeadSealError::Unseal
        );
        let french_token = french.token_for(SlotId::new(7)).unwrap();
        assert_eq!(
            skat.unseal(&sealed, SlotId::new(7), &french_token)
                .unwrap_err(),
            AeadSealError::Unseal
        );
    }

    #[test]
    fn hks__tampered_ciphertext_errors_for_every_bit() {
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let token = scheme.token_for(SlotId::new(7)).unwrap();
        let bytes = sealed.to_bytes();
        for bit in 0..(SealedBytes::LEN * 8) {
            let mut t = bytes;
            t[bit / 8] ^= 1 << (bit % 8);
            assert_eq!(
                scheme.unseal(&SealedBytes::from_bytes(t), SlotId::new(7), &token),
                Err(AeadSealError::Unseal),
                "bit {bit} flipped and still opened"
            );
        }
    }

    #[test]
    fn hks__nonce_is_fresh() {
        let scheme = dealer52();
        let mut rng = StdRng::seed_from_u64(1);
        let a = scheme
            .seal(ace_of_spades(), SlotId::new(7), &mut rng)
            .unwrap();
        let b = scheme
            .seal(ace_of_spades(), SlotId::new(7), &mut rng)
            .unwrap();
        assert_ne!(a.nonce(), b.nonce());
        assert_ne!(a.tag(), b.tag());
        assert_ne!(a, b);
    }

    #[test]
    fn hks__verifier_can_unseal() {
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let token = scheme.token_for(SlotId::new(7)).unwrap();
        let verifier = HolderKeySeal::<Standard52>::verifier(b"test");
        assert_eq!(
            verifier.unseal(&sealed, SlotId::new(7), &token).unwrap(),
            ace_of_spades()
        );
    }

    #[test]
    fn hks__verifier_cannot_seal() {
        let verifier = HolderKeySeal::<Standard52>::verifier(b"test");
        assert_eq!(
            verifier
                .seal(
                    ace_of_spades(),
                    SlotId::new(7),
                    &mut StdRng::seed_from_u64(1)
                )
                .unwrap_err(),
            AeadSealError::NoMasterKey
        );
    }

    #[test]
    fn hks__blank_card_errors() {
        let scheme = dealer52();
        assert!(matches!(
            scheme.seal(
                Card::<Standard52>::default(),
                SlotId::new(0),
                &mut StdRng::seed_from_u64(1)
            ),
            Err(AeadSealError::CardNotInDeck(_))
        ));
    }

    #[test]
    fn hks__reveal_with_through_kernel() {
        use crate::seal::revealed::Revealed;
        let scheme = dealer52();
        let sealed = scheme
            .seal(
                ace_of_spades(),
                SlotId::new(7),
                &mut StdRng::seed_from_u64(1),
            )
            .unwrap();
        let token = scheme.token_for(SlotId::new(7)).unwrap();
        let verifier = HolderKeySeal::<Standard52>::verifier(b"test");
        let mut revealed = Revealed::<Standard52>::new();
        let card = revealed
            .reveal_with(SlotId::new(7), &sealed, &verifier, &token)
            .unwrap();
        assert_eq!(card, ace_of_spades());
        assert_eq!(revealed.get(SlotId::new(7)), Some(ace_of_spades()));
    }

    // ── Story 4: deal and the holder flow ───────────────────────────────

    use crate::seal::revealed::Revealed;

    fn reveal_all(
        scheme: &HolderKeySeal<Standard52>,
        shoe: &SlotPile,
        custody: &Custody,
    ) -> crate::basic::types::pile::Pile<Standard52> {
        let verifier = HolderKeySeal::<Standard52>::verifier(scheme.context());
        let mut revealed = Revealed::<Standard52>::new();
        for &slot in shoe.slots() {
            let token = scheme.token_for(slot).unwrap();
            revealed
                .reveal_with(slot, custody.get(slot).unwrap(), &verifier, &token)
                .unwrap();
        }
        revealed.pile_for(shoe.slots()).unwrap()
    }

    #[test]
    fn hks__deal_then_reveal_all_is_permutation_of_deck() {
        let scheme = dealer52();
        let deck = Standard52::deck();
        let (shoe, custody) = scheme.deal(&deck, &mut StdRng::seed_from_u64(3)).unwrap();
        assert_eq!(shoe.len(), 52);
        assert_eq!(custody.len(), 52);
        let pile = reveal_all(&scheme, &shoe, &custody);
        assert!(deck.same(&pile));
        assert_ne!(deck, pile, "deal must shuffle before sealing");
    }

    #[test]
    fn hks__deal_shoe_and_custody_name_the_same_slots() {
        let scheme = dealer52();
        let (shoe, custody) = scheme
            .deal(&Standard52::deck(), &mut StdRng::seed_from_u64(3))
            .unwrap();
        for &slot in shoe.slots() {
            assert!(custody.get(slot).is_some(), "custody missing {slot}");
        }
        assert_eq!(shoe.slots(), SlotPile::new(52).slots());
    }

    #[test]
    fn hks__deal_slot_is_not_ordinal() {
        let scheme = dealer52();
        let deck = Standard52::deck();
        let mut fixed_points = 0usize;
        for seed in 0..20u64 {
            let (shoe, custody) = scheme
                .deal(&deck, &mut StdRng::seed_from_u64(seed))
                .unwrap();
            let pile = reveal_all(&scheme, &shoe, &custody);
            fixed_points += pile
                .cards()
                .iter()
                .enumerate()
                .filter(|(i, c)| scheme.codebook().ordinal(c).unwrap().index() == *i)
                .count();
        }
        // Expected ≈ 1 fixed point per deal (20 total); 52·20 would be a leak.
        assert!(
            fixed_points < 100,
            "{fixed_points} fixed points in 20 deals"
        );
    }

    /// The hazard `deal` exists to avoid: seal a sorted deck slot-by-slot and
    /// every slot names its own ordinal. Pinned so nobody "simplifies" `deal`.
    #[test]
    fn hks__unshuffled_seal_leaks_slot_eq_ordinal() {
        let scheme = dealer52();
        let deck = Standard52::deck();
        let mut rng = StdRng::seed_from_u64(0);
        for (i, card) in deck.cards().iter().enumerate() {
            let slot = SlotId::new(u16::try_from(i).unwrap());
            let sealed = scheme.seal(*card, slot, &mut rng).unwrap();
            let back = scheme
                .unseal(&sealed, slot, &scheme.token_for(slot).unwrap())
                .unwrap();
            assert_eq!(
                scheme.codebook().ordinal(&back).unwrap().index(),
                slot.index()
            );
        }
    }

    #[test]
    fn hks__deal_in_verifier_mode_errors() {
        let verifier = HolderKeySeal::<Standard52>::verifier(b"test");
        assert_eq!(
            verifier
                .deal(&Standard52::deck(), &mut StdRng::seed_from_u64(3))
                .unwrap_err(),
            AeadSealError::NoMasterKey
        );
    }

    #[test]
    fn hks__holder_flow() {
        // Dealer.
        let scheme = dealer52();
        let mut rng = StdRng::seed_from_u64(9);
        let (mut shoe, custody) = scheme.deal(&Standard52::deck(), &mut rng).unwrap();
        let hole = shoe.draw(2).unwrap();
        assert_eq!(shoe.len(), 50);
        let tokens = scheme.tokens_for(hole.slots().iter().copied()).unwrap();

        // Holder turns up the first card: publishes (slot, token).
        let (slot, token) = (tokens[0].0, tokens[0].1.clone());

        // Anyone verifies with no secret.
        let verifier = HolderKeySeal::<Standard52>::verifier(b"test");
        let mut revealed = Revealed::<Standard52>::new();
        let card = revealed
            .reveal_with(slot, custody.get(slot).unwrap(), &verifier, &token)
            .unwrap();
        assert!(Standard52::deck().cards().contains(&card));
        assert_eq!(revealed.len(), 1);
        assert!(!revealed.is_revealed(hole.slots()[1]));

        // That token opens nothing else.
        let other = hole.slots()[1];
        assert_eq!(
            verifier.unseal(custody.get(other).unwrap(), other, &token),
            Err(AeadSealError::Unseal)
        );
    }

    #[test]
    fn scheme__debug_never_prints_the_master() {
        let a = HolderKeySeal::<Standard52>::dealer(master(), b"ctx");
        let dbg = alloc::format!("{a:?}");
        assert!(!dbg.contains("01, 01"), "leaked: {dbg}");
        assert!(dbg.contains("dealer"), "{dbg}");
    }
}
