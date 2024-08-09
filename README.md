<div align="center">

# split-every

![crates.io version](https://img.shields.io/crates/v/split-every.svg?label=release)
![github.com forks](https://img.shields.io/github/forks/JumperBot/split-every)
![github.com stars](https://img.shields.io/github/stars/JumperBot/split-every)
![crates.io downloads](https://img.shields.io/crates/d/split-every.svg?label=downloads)

</div>

---

```rust
use split_every::{SplitEveryImpl, SplitEvery};
// This prints: "Oh hi there"
//              "I don't really"
//              "know what to"
//              "say".
let mut splitter: SplitEvery<&str> =
    "Oh hi there I don't really know what to say".split_every_n_times(" ", 3);
println!("{}", splitter.next().unwrap());
println!("{}", splitter.next().unwrap());
println!("{}", splitter.next().unwrap());
println!("{}", splitter.next().unwrap());
```

---

## ✨ Split For Every N Occurrences Of A Pattern Iteratively

This crate **helps you** split a `string` for every `n` occurrences of a `pattern`.  
It contains an exclusive `iterator`.

---

## 📄 Licensing

`split-every` is licensed under the [`MIT LICENSE`](./LICENSE); This is the [`summarization`](https://choosealicense.com/licenses/mit/).
