<div align="center">

# split-every

![crates.io version](https://img.shields.io/crates/v/split-every.svg?label=release)
![github.com forks](https://img.shields.io/github/forks/JumperBot/split-every)
![github.com stars](https://img.shields.io/github/stars/JumperBot/split-every)
![crates.io downloads](https://img.shields.io/crates/d/split-every.svg?label=downloads)

</div>

---

```rust
use split_every::prelude::*;

// This prints: [(0, 0), (0, 1)]
//              [(0, 0)]
//              [(0, 1), (0, 0)]
//              [(0, 1)]
let mut splitter: SplitEvery<&[(u8, u8)], &[(u8, u8)]> = [
    (0, 0), (0, 1), (0, 0),
    (0, 0), (0, 0), (0, 1),
    (0, 0), (0, 0), (0, 1),
].split_every_n_times(&[(0, 0)], 2);
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());

// This prints: "Oh hi there"
//              "I don't really"
//              "know what to"
//              "say".
let mut splitter: SplitEvery<&str, &str> =
    "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());

// This prints: ["This", "is", "you", "This"]
//              ["me", "This", "is", "someone", "This"]
//              ["them"]
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
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());

// This prints: ["This", "is", "you", "This"]
//              ["me", "This", "is", "someone", "This"]
//              ["them"]
let mut iter = [
    ["This", "is", "you"],
    ["This", "is", "me"],
    ["This", "is", "someone"],
    ["This", "is", "them"],
].iter().flatten().map(|val| *val);
let mut splitter: SplitEvery<Box<dyn FnMut() -> Option<&'static str>>, &str> =
    SplitEvery::n_times_from_fn(Box::new(move || iter.next()), "is", 2);
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
println!("{:?}", splitter.next().unwrap());
```

---

## ✨ Split For Every N Occurrences Of A Pattern Iteratively

This crate **helps you** split data for every `n` occurrences of a `pattern`.  
It contains an exclusive `iterator`.

---

## 📄 Licensing

`split-every` is licensed under the [`MIT LICENSE`](./LICENSE); This is the [`summarization`](https://choosealicense.com/licenses/mit/).
