# `tld`

[![Crates.io](https://img.shields.io/crates/v/tld.svg)](https://crates.io/crates/tld)
[![Documentation](https://docs.rs/tld/badge.svg)](https://docs.rs/tld)
[![Rust](https://github.com/ernestas-poskus/tld.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/ernestas-poskus/tld.rs/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

Fast compile-time static perfect hash set of all Top-Level Domains (TLDs), automatically kept in sync with the official [IANA Root Zone Database](https://data.iana.org/TLD/tlds-alpha-by-domain.txt).

## Features

- ⚡ **O(1) lookups** with zero runtime allocation powered by [`phf`](https://crates.io/crates/phf).
- 📦 **`no_std` compatible** out of the box.
- 🔄 **Automated weekly updates** from IANA.
- 🔤 **Case-insensitive & exact lookup helpers**.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tld = "2"
```

## Usage

```rust
use tld::{exist, exist_case_insensitive, TLD};

// Exact ASCII lowercase lookup
assert!(exist("com"));
assert!(exist("org"));
assert!(exist("io"));
assert!(exist("ai"));
assert!(!exist("invalidtld"));

// Case-insensitive lookup (no heap allocation)
assert!(exist_case_insensitive("COM"));
assert!(exist_case_insensitive("Io"));
assert!(exist_case_insensitive("XN--FIQS8S"));

// Direct access to compile-time phf::Set
assert!(TLD.contains("net"));
println!("Total TLDs: {}", TLD.len());
```

## License

Licensed under BSD 3-Clause.
