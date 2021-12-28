/// this is a table lookup for all "flush" hands (e.g.  both
/// flushes and straight-flushes.  entries containing a zero
/// mean that combination is not possible with a five-card
/// flush hand.
pub const FLUSHES: [u16; 7937] = include!("snip/flushes.snip");

/// this is a table lookup for all non-flush hands consisting
/// of five unique ranks (i.e.  either Straights or High Card
/// hands).  it's similar to the above "flushes" array.
pub const UNIQUE_5: [u16; 7937] = include!("snip/unique5.snip");

/// those two arrays are needed for original evaluator version
pub const PRODUCTS: [u32; 4888] = include!("snip/products.snip");
pub const VALUES: [u16; 4888] = include!("snip/values.snip");

/// primes associated with card values
pub const PRIMES: [u8; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

/// permutations of 5 cards from 7, to evaluate a hand + table cards with a 5-card algorithm
pub const PERM_7: [[u8; 5]; 21] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 3, 6],
    [0, 1, 2, 4, 5],
    [0, 1, 2, 4, 6],
    [0, 1, 2, 5, 6],
    [0, 1, 3, 4, 5],
    [0, 1, 3, 4, 6],
    [0, 1, 3, 5, 6],
    [0, 1, 4, 5, 6],
    [0, 2, 3, 4, 5],
    [0, 2, 3, 4, 6],
    [0, 2, 3, 5, 6],
    [0, 2, 4, 5, 6],
    [0, 3, 4, 5, 6],
    [1, 2, 3, 4, 5],
    [1, 2, 3, 4, 6],
    [1, 2, 3, 5, 6],
    [1, 2, 4, 5, 6],
    [1, 3, 4, 5, 6],
    [2, 3, 4, 5, 6],
];

/// permutations to evaluate all 6 card combinations.
pub const PERM_6: [[u8; 5]; 6] = [
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 5],
    [0, 1, 2, 4, 5],
    [0, 1, 3, 4, 5],
    [0, 2, 3, 4, 5],
    [1, 2, 3, 4, 5],
];

// perfect hash specific lookups
pub const HASH_VALUES: [u16; 8192] = include!("snip/hash_values.snip");
pub const HASH_ADJUST: [u16; 512] = include!("snip/hash_adjust.snip");
