/// This code was taken from [Vladislav Supalov's](https://github.com/vsupalov)
/// [pokereval-rs](https://github.com/vsupalov/pokereval-rs) library, which in
/// turn was based on Cactus Kev's (aka [Kevin Suffecool](https://suffe.cool/))
/// [Poker Hand Evaluator](http://suffe.cool/poker/evaluator.html) code in C.
///
/// ```txt
/// Copyright (c) 2015 Vladislav Supalov
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in
/// all copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
/// THE SOFTWARE.
/// ```
///
///
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
