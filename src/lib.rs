//! Split for every n occurrences of a pattern iteratively.
//! This crate **helps you** split a `string` for every `n` occurrences of a `pattern`.  
//! It contains an exclusive `iterator`.
//!
//! # Examples
//!
//! ```rust
//! use split_every::{SplitEveryImpl, SplitEvery};
//! // This prints: "Oh hi there"
//! //              "I don't really"
//! //              "know what to"
//! //              "say".
//! let mut splitter: SplitEvery<&str> =
//!     "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! ```

/// Import all necessary traits and structs.
pub mod prelude {
    pub use crate::{SplitEvery, SplitEveryImpl};
}

/// A trait containing all `string` split-every functions.
pub trait SplitEveryImpl: AsRef<str> {
    /// This splits a `string` every `n` times a `string` is found.
    /// This splits exclusively.
    #[must_use]
    #[allow(private_bounds)]
    fn split_every_n_times<'a, T: Pattern<'a>>(&'a self, pat: T, n: usize) -> SplitEvery<'a, T> {
        SplitEvery {
            inner: self.as_ref(),
            pat_byte_len: pat.byte_len(),
            pat,
            n,
            index: 0,
        }
    }
}

impl<T: AsRef<str>> SplitEveryImpl for T {}

/// A convinient substitution to `std::str::pattern::Pattern`.
trait Pattern<'a> {
    /// A convinient `find` method.
    fn find_in(&self, haystack: &str) -> Option<usize>;

    /// A convinient `len` method.
    fn byte_len(&self) -> Option<usize>;
}

impl<'a> Pattern<'a> for &'a str {
    fn find_in(&self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }

    fn byte_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'a> Pattern<'a> for char {
    fn find_in(&self, haystack: &str) -> Option<usize> {
        haystack.find(*self)
    }

    fn byte_len(&self) -> Option<usize> {
        Some(self.len_utf8())
    }
}

impl<'a> Pattern<'a> for &'a [char] {
    fn find_in(&self, haystack: &str) -> Option<usize> {
        haystack.find(*self)
    }

    fn byte_len(&self) -> Option<usize> {
        Some(self.iter().map(|ch| ch.len_utf8()).sum())
    }
}

impl<'a, F: FnMut(char) -> bool + Copy> Pattern<'a> for F {
    fn find_in(&self, haystack: &str) -> Option<usize> {
        haystack.find(*self)
    }

    fn byte_len(&self) -> Option<usize> {
        None
    }
}

impl<'a> Pattern<'a> for &'a String {
    fn find_in(&self, haystack: &str) -> Option<usize> {
        haystack.find(*self)
    }

    fn byte_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An `Iterator` struct for splitting a `string` every `n` occurrences of a `pattern`.
#[allow(private_bounds)]
pub struct SplitEvery<'a, T: Pattern<'a>> {
    inner: &'a str,
    pat: T,
    pat_byte_len: Option<usize>,
    n: usize,
    index: usize,
}

impl<'a, T: Pattern<'a>> Iterator for SplitEvery<'a, T> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.inner.len() {
            return None;
        }
        let iter_haystack: &str = unsafe { self.inner.get_unchecked(self.index..) };
        let mut len: usize = 0;
        if let Some(pat_byte_len) = self.pat_byte_len {
            for ind in 0..self.n {
                let haystack: &str = unsafe { iter_haystack.get_unchecked(len..) };
                if let Some(byte_ind) = self.pat.find_in(haystack) {
                    len = unsafe { len.unchecked_add(byte_ind).unchecked_add(pat_byte_len) };
                    continue;
                }
                if ind == 0 {
                    self.index = self.inner.len();
                    return Some(haystack);
                }
                break;
            }
            self.index = unsafe { self.index.unchecked_add(len) };
            Some(unsafe { iter_haystack.get_unchecked(..len.unchecked_sub(pat_byte_len)) })
        } else {
            let mut last_pat_len: usize = 0;
            for ind in 0..self.n {
                let haystack: &str = unsafe { iter_haystack.get_unchecked(len..) };
                if let Some(byte_ind) = self.pat.find_in(haystack) {
                    last_pat_len = match unsafe { haystack.as_bytes().get_unchecked(byte_ind) } {
                        0b0000_0000..=0b0111_1111 => 1,
                        0b1000_0000..=0b1101_1111 => 2,
                        0b1110_0000..=0b1110_1111 => 3,
                        0b1111_0000..=0b1111_1111 => 4,
                    };
                    len = unsafe { len.unchecked_add(byte_ind).unchecked_add(last_pat_len) };
                    continue;
                }
                if ind == 0 {
                    self.index = self.inner.len();
                    return Some(haystack);
                }
                break;
            }
            self.index = unsafe { self.index.unchecked_add(len) };
            Some(unsafe { iter_haystack.get_unchecked(..len.unchecked_sub(last_pat_len)) })
        }
    }
}

#[test]
fn test() {
    let mut splitter: SplitEvery<&str> = "oh oh oh oh oh".split_every_n_times(" ", 2);
    assert_eq!(splitter.next(), Some("oh oh"));
    assert_eq!(splitter.next(), Some("oh oh"));
    assert_eq!(splitter.next(), Some("oh"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter = "oh—oh—oh—oh—oh".split_every_n_times(|ch| ch == '—', 2);
    assert_eq!(splitter.next(), Some("oh—oh"));
    assert_eq!(splitter.next(), Some("oh—oh"));
    assert_eq!(splitter.next(), Some("oh"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<char> = "oboooobobobobob".split_every_n_times('o', 3);
    assert_eq!(splitter.next(), Some("obo"));
    assert_eq!(splitter.next(), Some("oob"));
    assert_eq!(splitter.next(), Some("bobob"));
    assert_eq!(splitter.next(), Some("b"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<char> = "hhhahahahaha".split_every_n_times('h', 1);
    assert_eq!(splitter.next(), Some(""));
    assert_eq!(splitter.next(), Some(""));
    assert_eq!(splitter.next(), Some(""));
    assert_eq!(splitter.next(), Some("a"));
    assert_eq!(splitter.next(), Some("a"));
    assert_eq!(splitter.next(), Some("a"));
    assert_eq!(splitter.next(), Some("a"));
    assert_eq!(splitter.next(), Some("a"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<&str> =
        "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
    assert_eq!(splitter.next().unwrap(), "Oh hi there");
    assert_eq!(splitter.next().unwrap(), "I don't really");
    assert_eq!(splitter.next().unwrap(), "know what to");
    assert_eq!(splitter.next().unwrap(), "say");
}
