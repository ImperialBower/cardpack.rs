use crate::basic::types::basic_pile::BasicPile;
use crate::basic::types::traits::Ranged;
use crate::prelude::{BasicCard, Pip};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Combos(Vec<BasicPile>);

impl Combos {
    /// Returns every possible combination of `Cards` of a given length for a specific `BasicPile`.
    ///
    /// ```
    /// use cardpack::prelude::*;
    /// use cardpack::basic::decks::tiny::Tiny;
    ///
    /// let pile: BasicPile = (&Pile::<Tiny>::deck()).into();
    /// let combos = pile.combos(2).connectors();
    ///
    /// assert_eq!(combos.len(), 4);
    /// assert_eq!(
    ///     combos.to_string(),
    ///     "A♠ K♠, A♠ K♥, A♥ K♠, A♥ K♥"
    /// );
    /// ```
    ///
    /// OK, this refactoring is hot AF. This is what I am looking for when I refactor.
    ///
    /// 1. Find a problem.
    /// 2. Define it in a test.
    /// 3. Solve the problem no matter how ugly the code is.
    /// 4. Refactor into a something prettier.
    /// 5. ...
    /// 6. Profile
    ///
    /// Now this is what we call the `Red -> Green -> Refactor` TDD loop. When I am coding
    /// in a new language, one of the things I look out for is ways to find these idiomatic
    /// refactoring opportunities.
    ///
    /// When you code, over time you start to start to develop an accent based on the language you
    /// are most comfortable with. For me, at first, that was Perl. Later on, when I tried to level
    /// up from a `Web Master` to an `Enterprise Architect`, it became Java.
    ///
    /// ## Funny story:
    ///
    /// The second job I got for a startup was for this company called `Katmango`. OK,
    /// I know the name is trash, but I respected the guy who reached out to get me to join,
    /// and the tech seemed to be a good idea. (In fact, the idea of having a unified sign-on was
    /// so good, that Google and Apple introduced the tech themselves, making our efforts
    /// a complete waste of time.) I must say, I don't miss those days being razzed for not being
    /// enough of a team player for only putting in 60 hours a week, and overhearing the CTO and CEO
    /// sitting behind me in the South of Market repurposed sweat shop, talking about how if they
    /// didn't secure another round of funding we wouldn't miss payroll.
    ///
    /// So, anyway. One day, I was told that I was going to be the companies "Web Master". I was
    /// so excited. This was like the coolest sounding title there was in the great days
    /// of the dot.com boom. A little while later, one of my colleagues clued me in. Web Master
    /// was the job they gave to the dev on the team with the least amount of experience. It was
    /// the equivalent to Nate's original job in Ted Lasso. It was the job where you dealt with
    /// the most annoying aspect of any web business: customer complaints. That was a good level
    /// set.
    ///
    /// ## Back to the point
    ///
    /// I learned this lesson in the most wonderful way from my friend Jim Prior. TODO:
    ///
    /// ---------------------------------------------------------
    ///
    /// The main lesson I've gotten from all that is to strive to learn the basic idiomatic ways
    /// to work within a language.
    ///
    #[must_use]
    pub fn connectors(&self) -> Self {
        self.iter()
            .filter(|pile| pile.is_connector())
            .map(|pile| pile.clone().sorted_by_rank())
            .collect()
    }

    /// I love how the `CoPilot` version recommends functions that don't exist instead of
    /// the one that does.
    ///
    /// ```txt
    /// self.0
    ///             .iter()
    ///             .filter(|pile| pile.contains_rank(rank))
    ///             .cloned()
    ///             .collect()
    /// ```
    #[must_use]
    pub fn of_rank(&self, rank: Pip) -> Self {
        self.iter()
            .filter(|pile| pile.all_of_rank(rank))
            .cloned()
            .collect()
    }

    /// TODO: RF Should be able to create a common function that accepts a closure.
    #[must_use]
    pub fn of_same_rank(&self) -> Self {
        self.iter()
            .filter(|pile| pile.all_of_same_rank())
            .cloned()
            .collect()
    }

    #[allow(clippy::or_fun_call)]
    #[must_use]
    pub fn of_same_rank_or_above(&self, rank: Pip) -> Self {
        self.of_same_rank()
            .iter()
            .filter(|pile| pile.v().first().unwrap_or(&BasicCard::default()).rank >= rank)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn suited(&self) -> Self {
        self.0
            .iter()
            .filter(|pile| pile.all_of_same_suit())
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn unsuited(&self) -> Self {
        self.0
            .iter()
            .filter(|pile| !pile.all_of_same_suit())
            .cloned()
            .collect()
    }

    //\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\//\\
    // region vector functions

    /// Returns a reference to the first element of the vector, if there is one.
    ///
    /// ```
    /// use cardpack::basic::types::combos::Combos;
    /// use cardpack::prelude::*;
    ///
    /// let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
    /// let combos = pile.combos(2).connectors();
    ///
    /// assert_eq!(combos.first().unwrap().to_string(), "A♠ K♠");
    /// assert!(Combos::default().first().is_none());
    /// ```
    #[must_use]
    pub fn first(&self) -> Option<&BasicPile> {
        self.0.first()
    }

    /// Returns a reference to the second element of the vector, if there is one.
    ///
    /// ```
    /// use cardpack::basic::types::combos::Combos;
    /// use cardpack::prelude::*;
    ///
    /// let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
    /// let combos = pile.combos(2).connectors();
    ///
    /// assert_eq!(combos.second().unwrap().to_string(), "A♠ K♥");
    /// assert!(Combos::default().second().is_none());
    /// ```
    /// Returns a reference to the second element of the vector, if there is one.
    #[must_use]
    pub fn second(&self) -> Option<&BasicPile> {
        self.0.get(1)
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<&BasicPile> {
        self.0.get(position)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BasicPile> {
        self.0.iter()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, pile: BasicPile) {
        self.0.push(pile);
    }

    pub fn pop(&mut self) -> Option<BasicPile> {
        self.0.pop()
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    pub fn sort(&mut self) {
        self.0.sort();
    }

    pub fn sort_by(&mut self, f: impl FnMut(&BasicPile, &BasicPile) -> std::cmp::Ordering) {
        self.0.sort_by(f);
    }

    #[must_use]
    pub fn sort_internal(&self) -> Self {
        let mut s: Self = self.0.iter().map(|pile| pile.clone().sorted()).collect();
        s.sort();
        s.sort_by_length();
        s.reverse();
        s
    }

    pub fn sort_by_length(&mut self) {
        self.0.sort_by_key(BasicPile::len);
    }

    #[must_use]
    pub fn v(&self) -> &Vec<BasicPile> {
        &self.0
    }

    // endregion
}

impl Display for Combos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl From<Vec<BasicPile>> for Combos {
    fn from(v: Vec<BasicPile>) -> Self {
        Self(v)
    }
}

impl From<&Vec<BasicPile>> for Combos {
    fn from(v: &Vec<BasicPile>) -> Self {
        Self(v.clone())
    }
}

impl FromIterator<BasicPile> for Combos {
    fn from_iter<T: IntoIterator<Item = BasicPile>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Iterator for Combos {
    type Item = BasicPile;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a> IntoIterator for &'a Combos {
    type Item = &'a BasicPile;
    type IntoIter = std::slice::Iter<'a, BasicPile>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
#[allow(non_snake_case, unused_imports)]
mod basic__types__combos_tests {
    use super::*;
    use crate::prelude::{Decked, DeckedBase, French, FrenchRank, Pile, Standard52};

    #[test]
    fn connectors() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).connectors();

        assert_eq!(combos.len(), 192);
        assert_eq!(
            combos.to_string(),
            "A♠ K♠, A♠ K♥, A♠ K♦, A♠ K♣, K♠ Q♠, K♠ Q♥, K♠ Q♦, K♠ Q♣, Q♠ J♠, Q♠ J♥, Q♠ J♦, Q♠ J♣, J♠ T♠, J♠ T♥, J♠ T♦, J♠ T♣, T♠ 9♠, T♠ 9♥, T♠ 9♦, T♠ 9♣, 9♠ 8♠, 9♠ 8♥, 9♠ 8♦, 9♠ 8♣, 8♠ 7♠, 8♠ 7♥, 8♠ 7♦, 8♠ 7♣, 7♠ 6♠, 7♠ 6♥, 7♠ 6♦, 7♠ 6♣, 6♠ 5♠, 6♠ 5♥, 6♠ 5♦, 6♠ 5♣, 5♠ 4♠, 5♠ 4♥, 5♠ 4♦, 5♠ 4♣, 4♠ 3♠, 4♠ 3♥, 4♠ 3♦, 4♠ 3♣, 3♠ 2♠, 3♠ 2♥, 3♠ 2♦, 3♠ 2♣, A♥ K♠, A♥ K♥, A♥ K♦, A♥ K♣, K♥ Q♠, K♥ Q♥, K♥ Q♦, K♥ Q♣, Q♥ J♠, Q♥ J♥, Q♥ J♦, Q♥ J♣, J♥ T♠, J♥ T♥, J♥ T♦, J♥ T♣, T♥ 9♠, T♥ 9♥, T♥ 9♦, T♥ 9♣, 9♥ 8♠, 9♥ 8♥, 9♥ 8♦, 9♥ 8♣, 8♥ 7♠, 8♥ 7♥, 8♥ 7♦, 8♥ 7♣, 7♥ 6♠, 7♥ 6♥, 7♥ 6♦, 7♥ 6♣, 6♥ 5♠, 6♥ 5♥, 6♥ 5♦, 6♥ 5♣, 5♥ 4♠, 5♥ 4♥, 5♥ 4♦, 5♥ 4♣, 4♥ 3♠, 4♥ 3♥, 4♥ 3♦, 4♥ 3♣, 3♥ 2♠, 3♥ 2♥, 3♥ 2♦, 3♥ 2♣, A♦ K♠, A♦ K♥, A♦ K♦, A♦ K♣, K♦ Q♠, K♦ Q♥, K♦ Q♦, K♦ Q♣, Q♦ J♠, Q♦ J♥, Q♦ J♦, Q♦ J♣, J♦ T♠, J♦ T♥, J♦ T♦, J♦ T♣, T♦ 9♠, T♦ 9♥, T♦ 9♦, T♦ 9♣, 9♦ 8♠, 9♦ 8♥, 9♦ 8♦, 9♦ 8♣, 8♦ 7♠, 8♦ 7♥, 8♦ 7♦, 8♦ 7♣, 7♦ 6♠, 7♦ 6♥, 7♦ 6♦, 7♦ 6♣, 6♦ 5♠, 6♦ 5♥, 6♦ 5♦, 6♦ 5♣, 5♦ 4♠, 5♦ 4♥, 5♦ 4♦, 5♦ 4♣, 4♦ 3♠, 4♦ 3♥, 4♦ 3♦, 4♦ 3♣, 3♦ 2♠, 3♦ 2♥, 3♦ 2♦, 3♦ 2♣, A♣ K♠, A♣ K♥, A♣ K♦, A♣ K♣, K♣ Q♠, K♣ Q♥, K♣ Q♦, K♣ Q♣, Q♣ J♠, Q♣ J♥, Q♣ J♦, Q♣ J♣, J♣ T♠, J♣ T♥, J♣ T♦, J♣ T♣, T♣ 9♠, T♣ 9♥, T♣ 9♦, T♣ 9♣, 9♣ 8♠, 9♣ 8♥, 9♣ 8♦, 9♣ 8♣, 8♣ 7♠, 8♣ 7♥, 8♣ 7♦, 8♣ 7♣, 7♣ 6♠, 7♣ 6♥, 7♣ 6♦, 7♣ 6♣, 6♣ 5♠, 6♣ 5♥, 6♣ 5♦, 6♣ 5♣, 5♣ 4♠, 5♣ 4♥, 5♣ 4♦, 5♣ 4♣, 4♣ 3♠, 4♣ 3♥, 4♣ 3♦, 4♣ 3♣, 3♣ 2♠, 3♣ 2♥, 3♣ 2♦, 3♣ 2♣"
        );
    }

    #[test]
    fn of_rank() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_rank(FrenchRank::ACE);

        assert_eq!(combos.len(), 6);
        assert_eq!(
            combos.to_string(),
            "A♠ A♥, A♠ A♦, A♠ A♣, A♥ A♦, A♥ A♣, A♦ A♣"
        );
    }

    #[test]
    fn of_same_rank() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_same_rank();

        assert_eq!(combos.len(), 78);
    }

    #[test]
    fn of_same_rank_or_above() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).of_same_rank_or_above(FrenchRank::KING);

        assert_eq!(
            combos.to_string(),
            "A♠ A♥, A♠ A♦, A♠ A♣, K♠ K♥, K♠ K♦, K♠ K♣, A♥ A♦, A♥ A♣, K♥ K♦, K♥ K♣, A♦ A♣, K♦ K♣"
        );
    }

    #[test]
    fn suited() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        let suited = combos.suited();
        let connectors = suited.connectors();

        assert_eq!(suited.len(), 312);
        assert_eq!(connectors.len(), 48);
    }

    // TODO: Why are these in the reverse order?
    #[test]
    fn unsuited() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2).unsuited();
        let mut connectors = combos.connectors();
        connectors.sort();

        assert_eq!(combos.len(), 1014);
        assert_eq!(connectors.len(), 144);
    }

    #[test]
    fn from_vec() {
        let from = vec![
            Pile::<French>::basic_pile(),
            Pile::<French>::basic_pile(),
            Pile::<French>::basic_pile(),
        ];

        let pile = Pile::<French>::basic_pile();

        let piles = Combos::from(from);

        assert_eq!(3, piles.len());
        for p in piles {
            assert_eq!(pile, p);
        }
    }

    #[test]
    fn get__returns_correct_element() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        // get(0) returns Some with a real pile, not None or an empty default
        let first = combos.get(0);
        assert!(first.is_some());
        assert!(!first.unwrap().is_empty());
        // out-of-bounds returns None
        assert!(combos.get(usize::MAX).is_none());
    }

    #[test]
    fn is_empty__false_when_populated() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        assert!(!combos.is_empty());
    }

    #[test]
    fn is_empty__true_when_empty() {
        let combos = Combos::default();
        assert!(combos.is_empty());
    }

    #[test]
    fn pop__returns_some() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let mut combos = pile.combos(2);
        let popped = combos.pop();
        assert!(popped.is_some());
        assert_ne!(popped, Some(BasicPile::default()));
    }

    #[test]
    fn sort__reorders() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let mut combos = pile.combos(2);
        let len_before = combos.len();
        combos.sort();
        assert_eq!(combos.len(), len_before);
        // verify it's actually sorted — first element should compare <= last
        let first = combos.get(0).unwrap().clone();
        let last = combos.get(combos.len() - 1).unwrap().clone();
        assert!(first <= last);
    }

    #[test]
    fn sort_by__works() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let mut combos = pile.combos(2);
        let len_before = combos.len();
        combos.sort_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(combos.len(), len_before);
    }

    #[test]
    fn v__not_empty() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        let v = combos.v();
        assert!(!v.is_empty());
        assert_ne!(v, &vec![BasicPile::default()]);
    }

    #[test]
    fn from_ref_vec_basic_pile() {
        let piles = vec![
            Pile::<Standard52>::basic_pile(),
            Pile::<Standard52>::basic_pile(),
        ];
        let combos = Combos::from(&piles);
        assert_eq!(combos.len(), 2);
        assert_ne!(combos, Combos::default());
    }

    #[test]
    fn iterator__yields_all() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let expected_len = pile.combos(2).len();
        let mut combos = pile.combos(2);
        let mut count = 0;
        while combos.next().is_some() {
            count += 1;
        }
        assert_eq!(count, expected_len);
    }

    #[test]
    fn into_iterator__for_ref() {
        let pile: BasicPile = (&Pile::<Standard52>::deck()).into();
        let combos = pile.combos(2);
        let expected_len = combos.len();
        let count = (&combos).into_iter().count();
        assert_eq!(count, expected_len);
    }
}
