//! Split for every n occurrences of a pattern iteratively.
//! This crate **helps you** split data for every `n` occurrences of a `pattern`.  
//! It contains an exclusive `iterator`.
//!
//! # Examples
//!
//! ```rust
//! use split_every::prelude::*;
//!
//! // This prints: [(0, 0), (0, 1)]
//! //              [(0, 0)]
//! //              [(0, 1), (0, 0)]
//! //              [(0, 1)]
//! let mut splitter: SplitEvery<&[(u8, u8)], &[(u8, u8)]> = [
//!     (0, 0), (0, 1), (0, 0),
//!     (0, 0), (0, 0), (0, 1),
//!     (0, 0), (0, 0), (0, 1),
//! ].split_every_n_times(&[(0, 0)], 2);
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//!
//! // This prints: "Oh hi there"
//! //              "I don't really"
//! //              "know what to"
//! //              "say".
//! let mut splitter: SplitEvery<&str, &str> =
//!     "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//!
//! // This prints: ["This", "is", "you", "This"]
//! //              ["me", "This", "is", "someone", "This"]
//! //              ["them"]
//! let mut splitter: SplitEvery<Box<dyn FnMut() -> Option<&'static str>>, &str> = [
//!     ["This", "is", "you"],
//!     ["This", "is", "me"],
//!     ["This", "is", "someone"],
//!     ["This", "is", "them"],
//! ]
//! .iter()
//! .flatten()
//! .copied()
//! .split_every_n_times("is", 2);
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//!
//! // This prints: ["This", "is", "you", "This"]
//! //              ["me", "This", "is", "someone", "This"]
//! //              ["them"]
//!
//! let mut iter = [
//!     ["This", "is", "you"],
//!     ["This", "is", "me"],
//!     ["This", "is", "someone"],
//!     ["This", "is", "them"],
//! ].iter().flatten().map(|val| *val);
//! let mut splitter: SplitEvery<Box<dyn FnMut() -> Option<&'static str>>, &str> =
//!     SplitEvery::n_times_from_fn(Box::new(move || iter.next()), "is", 2);
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! println!("{:?}", splitter.next().unwrap());
//! ```

/// Import all necessary traits and structs.
pub mod prelude {
    pub use crate::{SplitEvery, SplitEveryImpl, SplitEveryIterImpl};
}

pub trait SplitEveryImpl: Sized {
    fn split_every_n_times(self, pat: Self, n: usize) -> SplitEvery<Self, Self> {
        SplitEvery {
            input: self,
            pat,
            n,
            ind: 0,
        }
    }
}

impl SplitEveryImpl for &str {}
impl SplitEveryImpl for String {}
impl SplitEveryImpl for std::string::Drain<'_> {}
impl<T: Clone + PartialEq> SplitEveryImpl for Vec<T> {}
impl<T: Clone + PartialEq> SplitEveryImpl for &[T] {}

pub trait SplitEveryIterImpl<'a, T: Clone + PartialEq>: Iterator<Item = T> + Sized + 'a {
    fn split_every_n_times(
        mut self,
        pat: T,
        n: usize,
    ) -> SplitEvery<Box<dyn FnMut() -> Option<T> + 'a>, T> {
        SplitEvery::n_times_from_fn(Box::new(move || self.next()), pat, n)
    }
}

impl<'a, T: Clone + PartialEq, U: Iterator<Item = T> + Sized + 'a> SplitEveryIterImpl<'a, T> for U {}

pub struct SplitEvery<Input, Pattern> {
    input: Input,
    pat: Pattern,
    n: usize,
    ind: usize,
}

impl<Input: FnMut() -> Option<Pattern>, Pattern: PartialEq> SplitEvery<Input, Pattern> {
    pub fn n_times_from_fn(input: Input, pat: Pattern, n: usize) -> SplitEvery<Input, Pattern> {
        SplitEvery {
            input,
            pat,
            n,
            ind: 0,
        }
    }
}

impl<Input: FnMut() -> Option<Pattern>, Pattern: PartialEq> Iterator
    for SplitEvery<Input, Pattern>
{
    type Item = Vec<Pattern>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            let out: Vec<Pattern> = std::iter::repeat_with(|| (self.input)())
                .take_while(Option::is_some)
                .flatten()
                .collect();
            if out.is_empty() {
                return None;
            }
            return Some(out);
        }
        let mut out: Vec<Pattern> = Vec::with_capacity(5);
        'main: for ind in 0..self.n {
            while let Some(val) = (self.input)() {
                if val == self.pat {
                    if ind == unsafe { self.n.unchecked_sub(1) } {
                        break 'main;
                    }
                    out.push(val);
                    continue 'main;
                }
                out.push(val);
            }
        }
        if out.is_empty() {
            return None;
        }
        Some(out)
    }
}

impl<Pattern: AsRef<str>> Iterator for SplitEvery<&str, Pattern> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        split_every_next_str_helper(self)
    }
}

impl<Pattern: AsRef<str>> Iterator for SplitEvery<String, Pattern> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        split_every_next_str_helper(self)
    }
}

impl<Pattern: AsRef<str>> Iterator for SplitEvery<std::string::Drain<'_>, Pattern> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        split_every_next_str_helper(self)
    }
}

fn split_every_next_str_helper<Input: AsRef<str>, Pattern: AsRef<str>>(
    split_every: &mut SplitEvery<Input, Pattern>,
) -> Option<String> {
    let input: &str = split_every.input.as_ref();
    if split_every.ind == input.len() {
        return None;
    }
    let pat: &str = split_every.pat.as_ref();
    let iter_haystack: &str = unsafe { input.get_unchecked(split_every.ind..) };
    let mut len: usize = 0;
    for ind in 0..split_every.n {
        let haystack: &str = unsafe { iter_haystack.get_unchecked(len..) };
        if let Some(byte_ind) = haystack.find(pat) {
            len = unsafe { len.unchecked_add(byte_ind).unchecked_add(pat.len()) };
            continue;
        }
        if ind == 0 {
            split_every.ind = input.len();
            return Some(haystack.to_string());
        }
        break;
    }
    split_every.ind = unsafe { split_every.ind.unchecked_add(len) };
    Some(unsafe { iter_haystack.get_unchecked(..len.unchecked_sub(pat.len())) }.to_string())
}

impl<T: Clone + PartialEq> Iterator for SplitEvery<Vec<T>, Vec<T>> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (ind, out): (usize, Option<Vec<T>>) =
            split_every_next_arr_helper(self, &self.input, &self.pat);
        self.ind = ind;
        out
    }
}

impl<T: Clone + PartialEq> Iterator for SplitEvery<&[T], &[T]> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (ind, out): (usize, Option<Vec<T>>) =
            split_every_next_arr_helper(self, self.input, self.pat);
        self.ind = ind;
        out
    }
}

fn split_every_next_arr_helper<T, U: Clone + PartialEq>(
    split_every: &SplitEvery<T, T>,
    input: &[U],
    pat: &[U],
) -> (usize, Option<Vec<U>>) {
    if split_every.ind == input.len() {
        return (split_every.ind, None);
    }
    let iter_haystack: &[U] = unsafe { input.get_unchecked(split_every.ind..) };
    let mut len: usize = 0;
    for ind in 0..split_every.n {
        if len == iter_haystack.len() {
            break;
        }
        if let Some((ind, _)) = unsafe { iter_haystack.get_unchecked(len..) }
            .windows(pat.len())
            .enumerate()
            .find(|(_, val)| val == &pat)
        {
            len = unsafe { len.unchecked_add(ind).unchecked_add(pat.len()) };
            continue;
        }
        if ind == 0 {
            return (input.len(), Some(iter_haystack.to_vec()));
        }
        break;
    }
    (
        unsafe { split_every.ind.unchecked_add(len) },
        Some(unsafe { iter_haystack.get_unchecked(..len.unchecked_sub(pat.len())) }.to_vec()),
    )
}

#[test]
fn test() {
    let mut splitter: SplitEvery<&str, &str> = "oh oh oh oh oh".split_every_n_times(" ", 2);
    assert_eq!(splitter.next().unwrap(), "oh oh");
    assert_eq!(splitter.next().unwrap(), "oh oh");
    assert_eq!(splitter.next().unwrap(), "oh");
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<&str, &str> = "a a a a".split_every_n_times("b", 2);
    assert_eq!(splitter.next().unwrap(), "a a a a");
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<Box<dyn FnMut() -> Option<&'static str>>, &str> = [
        ["This", "is", "you"],
        ["This", "is", "me"],
        ["This", "is", "someone"],
        ["This", "is", "them"],
    ]
    .iter()
    .flatten()
    .copied()
    .split_every_n_times("is", 2);
    assert_eq!(splitter.next().unwrap(), vec!["This", "is", "you", "This"]);
    assert_eq!(
        splitter.next().unwrap(),
        vec!["me", "This", "is", "someone", "This"]
    );
    assert_eq!(splitter.next().unwrap(), vec!["them"]);
    assert_eq!(splitter.next(), None);

    let mut iter = [
        ["This", "is", "you"],
        ["This", "is", "me"],
        ["This", "is", "someone"],
        ["This", "is", "them"],
    ]
    .iter()
    .flatten()
    .copied();
    let mut splitter: SplitEvery<Box<dyn FnMut() -> Option<&'static str>>, &str> =
        SplitEvery::<Box<dyn FnMut() -> Option<&'static str>>, &str>::n_times_from_fn(
            Box::new(move || iter.next()),
            "is",
            2,
        );
    assert_eq!(splitter.next().unwrap(), vec!["This", "is", "you", "This"]);
    assert_eq!(
        splitter.next().unwrap(),
        vec!["me", "This", "is", "someone", "This"]
    );
    assert_eq!(splitter.next().unwrap(), vec!["them"]);
    assert_eq!(splitter.next(), None);

    #[allow(clippy::type_complexity)]
    let mut splitter: SplitEvery<&[(u8, u8)], &[(u8, u8)]> = [
        (0, 0),
        (0, 1),
        (0, 0), // Split
        (0, 0),
        (0, 0), // Split
        (0, 1),
        (0, 0),
        (0, 0), // Split
        (0, 1),
    ]
    .split_every_n_times(&[(0, 0)], 2);
    assert_eq!(splitter.next().unwrap(), vec![(0, 0), (0, 1)]);
    assert_eq!(splitter.next().unwrap(), vec![(0, 0)]);
    assert_eq!(splitter.next().unwrap(), vec![(0, 1), (0, 0)]);
    assert_eq!(splitter.next().unwrap(), vec![(0, 1)]);
    assert_eq!(splitter.next(), None);

    let mut splitter: SplitEvery<&str, &str> =
        "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
    assert_eq!(splitter.next().unwrap(), "Oh hi there");
    assert_eq!(splitter.next().unwrap(), "I don't really");
    assert_eq!(splitter.next().unwrap(), "know what to");
    assert_eq!(splitter.next().unwrap(), "say");
    assert_eq!(splitter.next(), None);
}
