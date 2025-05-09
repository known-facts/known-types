# Known Types

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types)
[![Documentation](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/)

Well-known types for Rust.

## ✨ Features

- Zero mandatory dependencies, only optional [integrations](#integrations).
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

### Crates

Crate | Version | Docs | Summary
:--- | :--- | :--- | :---
[known-types] | [![known-types](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types) | [![known-types](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/) | Well-known types.
[known-types-anthropic] | [![known-types-anthropic](https://img.shields.io/crates/v/known-types-anthropic)](https://crates.io/crates/known-types-anthropic) | [![known-types-anthropic](https://docs.rs/known-types-anthropic/badge.svg)](https://docs.rs/known-types-anthropic/) | Well-known types for Anthropic APIs.
[known-types-google] | [![known-types-google](https://img.shields.io/crates/v/known-types-google)](https://crates.io/crates/known-types-google) | [![known-types-google](https://docs.rs/known-types-google/badge.svg)](https://docs.rs/known-types-google/) | Well-known types for Google APIs.
[known-types-graphql] | [![known-types-graphql](https://img.shields.io/crates/v/known-types-graphql)](https://crates.io/crates/known-types-graphql) | [![known-types-graphql](https://docs.rs/known-types-graphql/badge.svg)](https://docs.rs/known-types-graphql/) | Well-known types for GraphQL specifications.
[known-types-ietf] | [![known-types-ietf](https://img.shields.io/crates/v/known-types-ietf)](https://crates.io/crates/known-types-ietf) | [![known-types-ietf](https://docs.rs/known-types-ietf/badge.svg)](https://docs.rs/known-types-ietf/) | Well-known types for IETF specifications.
[known-types-nostr] | [![known-types-nostr](https://img.shields.io/crates/v/known-types-nostr)](https://crates.io/crates/known-types-nostr) | [![known-types-nostr](https://docs.rs/known-types-nostr/badge.svg)](https://docs.rs/known-types-nostr/) | Well-known types for the Nostr protocol.
[known-types-openai] | [![known-types-openai](https://img.shields.io/crates/v/known-types-openai)](https://crates.io/crates/known-types-openai) | [![known-types-openai](https://docs.rs/known-types-openai/badge.svg)](https://docs.rs/known-types-openai/) | Well-known types for OpenAI APIs.
[known-types-pypi] | [![known-types-pypi](https://img.shields.io/crates/v/known-types-pypi)](https://crates.io/crates/known-types-pypi) | [![known-types-pypi](https://docs.rs/known-types-pypi/badge.svg)](https://docs.rs/known-types-pypi/) | Well-known types for Python Package Index (PyPI) APIs.
[known-types-rubygems] | [![known-types-rubygems](https://img.shields.io/crates/v/known-types-rubygems)](https://crates.io/crates/known-types-rubygems) | [![known-types-rubygems](https://docs.rs/known-types-rubygems/badge.svg)](https://docs.rs/known-types-rubygems/) | Well-known types for RubyGems.org APIs.
[known-types-w3c] | [![known-types-w3c](https://img.shields.io/crates/v/known-types-w3c)](https://crates.io/crates/known-types-w3c) | [![known-types-w3c](https://docs.rs/known-types-w3c/badge.svg)](https://docs.rs/known-types-w3c/) | Well-known types for W3C specifications.
[known-types-x] | [![known-types-x](https://img.shields.io/crates/v/known-types-x)](https://crates.io/crates/known-types-x) | [![known-types-x](https://docs.rs/known-types-x/badge.svg)](https://docs.rs/known-types-x/) | Well-known types for X (formerly Twitter) APIs.
<img width=210 height=1/> | <img width=110 height=1/> | <img width=100 height=1/> | &nbsp;

### Integrations

Crate | Version | Usage | Derives
:--- | :--- | :--- | :---
[bincode] | `version = "2"` | `features = ["bincode"]` | `bincode::{Encode, Decode}`
[borsh] | `version = "1.5"` | `features = ["borsh"]` | `borsh::{BorshSerialize, BorshDeserialize}`
[musli] | `version = "0.0.131"` | `features = ["musli"]` | `musli::{Encode, Decode}`
[rasn] | `version = "0.26"` | `features = ["rasn"]` | `rasn::AsnType` with `rasn(automatic_tags)`
[serde] | `version = "1"` | `features = ["serde"]` | `serde::{Serialize, Deserialize}`
<img width=210 height=1/> | <img width=110 height=1/> | <img width=100 height=1/> | &nbsp;

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

[bincode]: https://crates.io/crates/bincode
[borsh]: https://crates.io/crates/borsh
[musli]: https://crates.io/crates/musli
[rasn]: https://crates.io/crates/rasn
[serde]: https://crates.io/crates/serde

[known-types]: https://github.com/known-facts/known-types/tree/master/lib/known-types
[known-types-anthropic]: https://github.com/known-facts/known-types/tree/master/lib/known-types-anthropic
[known-types-google]: https://github.com/known-facts/known-types/tree/master/lib/known-types-google
[known-types-graphql]: https://github.com/known-facts/known-types/tree/master/lib/known-types-graphql
[known-types-ietf]: https://github.com/known-facts/known-types/tree/master/lib/known-types-ietf
[known-types-nostr]: https://github.com/known-facts/known-types/tree/master/lib/known-types-nostr
[known-types-openai]: https://github.com/known-facts/known-types/tree/master/lib/known-types-openai
[known-types-pypi]: https://github.com/known-facts/known-types/tree/master/lib/known-types-pypi
[known-types-rubygems]: https://github.com/known-facts/known-types/tree/master/lib/known-types-rubygems
[known-types-w3c]: https://github.com/known-facts/known-types/tree/master/lib/known-types-w3c
[known-types-x]: https://github.com/known-facts/known-types/tree/master/lib/known-types-x
