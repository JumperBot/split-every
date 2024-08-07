//! Split for every n occurences of a pattern iteratively.
//! This crate **helps you** split a `string` for every `n` occurences of a `pattern`.  
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
//! let mut splitter: SplitEvery =
//!     "Oh hi there I don't really know what to say".split_every_n_of_str(" ", 3);
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! println!("{}", splitter.next().unwrap());
//! ```

/// A trait containing all `string` split-every functions.
pub trait SplitEveryImpl: AsRef<str> {
    /// This splits a `string` every `n` times a `string` is found.
    /// This splits exclusively.
    /// The `string` must be `utf8-encoded`.
    #[must_use]
    fn split_every_n_of_str<'a>(&'a self, pat: &'a str, n: usize) -> SplitEvery<'a> {
        assert!(n > 0, "n must be greater than 0");
        SplitEvery {
            inner: self.as_ref(),
            pat: Pattern::Str(pat),
            n,
            index: 0,
        }
    }

    /// This splits a `string` every `n` times a `char` is found.
    /// This splits exclusively.
    /// The `string` must be `utf8-encoded`.
    #[must_use]
    fn split_every_n_of_char(&self, pat: char, n: usize) -> SplitEvery<'_> {
        assert!(n > 0, "n must be greater than 0");
        SplitEvery {
            inner: self.as_ref(),
            pat: Pattern::Ch(pat),
            n,
            index: 0,
        }
    }
}

impl<T: AsRef<str>> SplitEveryImpl for T {}

/// A convinient substitution to `std::str::pattern::Pattern`.
enum Pattern<'a> {
    Str(&'a str),
    Ch(char),
}

impl<'a> Pattern<'a> {
    /// A convinient `len` method.
    fn len(&self) -> usize {
        match self {
            Self::Str(inner) => inner.len(),
            Self::Ch(inner) => inner.len_utf8(),
        }
    }
}

/// An `Iterator` struct for splitting a `string` every `n` occurences of a `pattern`.
pub struct SplitEvery<'a> {
    inner: &'a str,
    pat: Pattern<'a>,
    n: usize,
    index: usize,
}

impl<'a> Iterator for SplitEvery<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.inner.len() {
            return None;
        }
        let haystack: &str = unsafe { self.inner.get_unchecked(self.index..) };
        let mut len: usize = 0;
        for ind in 0..self.n {
            let haystack: &str = unsafe { haystack.get_unchecked(len..) };
            if let Some(byte_ind) = match self.pat {
                Pattern::Str(inner) => haystack.find(inner),
                Pattern::Ch(inner) => haystack.find(inner),
            } {
                len = unsafe { len.unchecked_add(byte_ind).unchecked_add(self.pat.len()) };
                continue;
            }
            if ind == 0 {
                self.index = self.inner.len();
                return Some(haystack);
            }
            break;
        }
        self.index = unsafe { self.index.unchecked_add(len) };
        Some(unsafe { haystack.get_unchecked(..len.unchecked_sub(self.pat.len())) })
    }
}

#[test]
fn test() {
    let mut splitter: SplitEvery = "oh oh oh oh oh".split_every_n_of_str(" ", 2);
    assert_eq!(splitter.next(), Some("oh oh"));
    assert_eq!(splitter.next(), Some("oh oh"));
    assert_eq!(splitter.next(), Some("oh"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery = "oboooobobobobob".split_every_n_of_char('o', 3);
    assert_eq!(splitter.next(), Some("obo"));
    assert_eq!(splitter.next(), Some("oob"));
    assert_eq!(splitter.next(), Some("bobob"));
    assert_eq!(splitter.next(), Some("b"));
    assert_eq!(splitter.next(), None);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery = "hhhahahahaha".split_every_n_of_char('h', 1);
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

    let mut splitter: SplitEvery =
        "Oh hi there I don't really know what to say".split_every_n_of_str(" ", 3);
    assert_eq!(splitter.next().unwrap(), "Oh hi there");
    assert_eq!(splitter.next().unwrap(), "I don't really");
    assert_eq!(splitter.next().unwrap(), "know what to");
    assert_eq!(splitter.next().unwrap(), "say");
}
