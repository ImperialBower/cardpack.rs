use crate::funky::types::card::BuffoonCard;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct BPile(Vec<BuffoonCard>);
