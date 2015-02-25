// Copyright 2014 Johannes Köster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Algorithm of Knuth Morris and Pratt.
//! Constructs an automaton recognizing the pattern, and scans linearly over
//! a text of length n. Complexity: O(n).
//! The transition function delta is simulated via the lps-function, that assigns to each position
//! q in the pattern the longest prefix of the pattern that is suffix of pattern[..q+1].
//! Then, in the NFA for the pattern, active states after reading position q are
//! {q, lps(q), lps(lps(q)), ... 0}.
//!
//! # Example
//!
//! ```
//! use bio::pattern_matching::kmp::KMP;
//! let text = b"aaaaabbabbbbbbbabbab";
//! let pattern = b"abbab";
//! let kmp = KMP::new(pattern);
//! let occ: Vec<usize> = kmp.find_all(text.iter()).collect();
//! assert_eq!(occ, [4, 15]);
//! ```


use std::iter::{repeat, Enumerate};


type LPS = Vec<usize>;


pub struct KMP<'a> {
    m: usize,
    lps: LPS,
    pattern: &'a [u8]
}


impl<'a> KMP<'a> {
    pub fn new(pattern: &'a [u8]) -> Self {
        let m = pattern.len();
        let lps = lps(pattern);

        KMP { lps: lps, m: m, pattern: pattern }
    }

    fn delta(&self, mut q: usize, a: u8) -> usize {
        while q == self.m || (self.pattern[q] != a && q > 0) {
            q = self.lps[q-1];
        }
        if self.pattern[q] == a {
            q += 1;
        }

        q
    }

    pub fn find_all<'b, I: Iterator<Item=&'b u8>>(&'b self, text: I) -> KMPMatches<I> {
        KMPMatches { kmp: self, q: 0, text: text.enumerate() }
    }
}


fn lps(pattern: &[u8]) -> LPS {
    let (m, mut q) = (pattern.len(), 0);
    let mut lps: LPS = repeat(0).take(m).collect();
    for i in 1..m {
        while q > 0 && pattern[q] != pattern[i] {
            q = lps[q - 1];
        }
        if pattern[q] == pattern[i] {
            q += 1;
        }
        lps[i] = q;
    }

    lps
}


pub struct KMPMatches<'a, I: Iterator<Item=&'a u8>> {
    kmp: &'a KMP<'a>,
    q: usize,
    text: Enumerate<I>,
}


impl<'a, I: Iterator<Item=&'a u8>> Iterator for KMPMatches<'a, I> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        for (i, &c) in self.text.by_ref() {
            self.q = self.kmp.delta(self.q, c);
            if self.q == self.kmp.m {
                return Some(i - self.kmp.m + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{lps, KMP};

    #[test]
    fn test_lps() {
        let pattern = b"ababaca";
        let lps = lps(pattern);
        assert_eq!(lps, [0, 0, 1, 2, 3, 0, 1]);
    }

    #[test]
    fn test_delta() {
        let pattern = b"abbab";
        let kmp = KMP::new(pattern);
        assert_eq!(kmp.delta(0, b'a'), 1);
        assert_eq!(kmp.delta(0, b'b'), 0);
        assert_eq!(kmp.delta(1, b'a'), 1);
        assert_eq!(kmp.delta(1, b'b'), 2);
        assert_eq!(kmp.delta(2, b'a'), 1);
        assert_eq!(kmp.delta(2, b'b'), 3);
        assert_eq!(kmp.delta(3, b'a'), 4);
        assert_eq!(kmp.delta(3, b'b'), 0);
        assert_eq!(kmp.delta(4, b'a'), 1);
        assert_eq!(kmp.delta(4, b'b'), 5);
        assert_eq!(kmp.delta(5, b'a'), 1);
        assert_eq!(kmp.delta(5, b'b'), 3);
    }
}
