# Known Types

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types)
[![Documentation](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/)

Well-known types for Rust.

## ✨ Features

- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add known-types
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
known-types = "0.1"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
known-types = { version = "0.1", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the library

```rust
use known_types;
```

## 📚 Reference

https://docs.rs/known-types/

## 👨‍💻 Development

```bash
git clone https://github.com/known-facts/known-types.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/known-facts/known-types&text=Known%20Types)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/known-facts/known-types&title=Known%20Types)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/known-facts/known-types&t=Known%20Types)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/known-facts/known-types)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/known-facts/known-types)

[feature flags]: https://github.com/known-facts/known-types/blob/master/lib/known-types/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
