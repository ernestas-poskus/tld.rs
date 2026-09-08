//! # `tld`
//!
//! A compile-time static perfect hash map of all official top-level domains (TLDs),
//! automatically synchronized from the [IANA Root Zone Database](https://data.iana.org/TLD/tlds-alpha-by-domain.txt).
//!
//! ## Features
//!
//! - **Zero Runtime Allocation / O(1) Lookups**: Backed by [`phf::Set`].
//! - **`no_std` Compatible**: Can be used in embedded or `no_std` crates.
//! - **Case-insensitive helper**: [`exist_case_insensitive`] for convenience.
//!
//! ## Example
//!
//! ```rust
//! use tld::{exist, exist_case_insensitive, TLD};
//!
//! // Fast exact lookup (requires lowercase ASCII string)
//! assert!(exist("com"));
//! assert!(exist("org"));
//! assert!(exist("io"));
//! assert!(exist("uk"));
//! assert!(!exist("invalidtld"));
//!
//! // Case-insensitive lookup helper
//! assert!(exist_case_insensitive("COM"));
//! assert!(exist_case_insensitive("Xn--FIQS8S"));
//!
//! // Direct access to the compile-time phf::Set
//! assert!(TLD.contains("net"));
//! assert!(TLD.len() > 1400);
//! ```

#![no_std]
#![deny(missing_docs)]
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Checks if the given ASCII lowercase string is a valid Top-Level Domain (TLD)
/// in the official IANA database.
///
/// Note: This function performs an exact match and expects lowercased input.
/// For case-insensitive lookup, see [`exist_case_insensitive`].
///
/// # Examples
///
/// ```
/// assert!(tld::exist("com"));
/// assert!(tld::exist("io"));
/// assert!(tld::exist("lt"));
/// assert!(!exist_invalid("example"));
///
/// fn exist_invalid(s: &str) -> bool {
///     tld::exist(s)
/// }
/// ```
#[inline]
#[must_use]
pub fn exist(s: &str) -> bool {
    TLD.contains(s)
}

/// Checks if the given string is a valid Top-Level Domain (TLD), ignoring ASCII case.
///
/// This performs an O(1) lookup without allocating memory on the heap.
///
/// # Examples
///
/// ```
/// assert!(tld::exist_case_insensitive("COM"));
/// assert!(tld::exist_case_insensitive("cOm"));
/// assert!(tld::exist_case_insensitive("io"));
/// assert!(!tld::exist_case_insensitive("nonexistent"));
/// ```
#[inline]
#[must_use]
pub fn exist_case_insensitive(s: &str) -> bool {
    // If all characters are already lowercase ASCII, avoid buffer entirely
    if s.bytes().all(|b| !b.is_ascii_uppercase()) {
        return TLD.contains(s);
    }

    // Maximum TLD length is 63 octets per RFC 1035 / RFC 1123
    let bytes = s.as_bytes();
    if bytes.len() > 63 || bytes.is_empty() {
        return false;
    }

    let mut buf = [0u8; 63];
    for (i, &b) in bytes.iter().enumerate() {
        buf[i] = b.to_ascii_lowercase();
    }

    if let Ok(lower_str) = core::str::from_utf8(&buf[..bytes.len()]) {
        TLD.contains(lower_str)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tld_set() {
        assert!(TLD.get_key("aaa").is_some());
        assert!(TLD.get_key("#33dawaaa").is_none());
        assert!(TLD.get_key("aco").is_some());
        assert!(TLD.get_key("uk").is_some());
        assert!(TLD.get_key("ye").is_some());
        assert!(TLD.get_key("com").is_some());
        assert!(TLD.get_key("de").is_some());
        assert!(TLD.get_key("fr").is_some());
        assert!(TLD.get_key("ag").is_some());
        assert!(TLD.get_key("ru").is_some());
        assert!(TLD.get_key("nl").is_some());
        assert!(TLD.get_key("lt").is_some());
        assert!(TLD.get_key("amex").is_some());
        assert!(TLD.get_key("zw").is_some());
    }

    #[test]
    fn test_exist() {
        assert!(exist("fr"));
        assert!(exist("de"));
        assert!(exist("zw"));
        assert!(!exist("a9292zw"));
        assert!(!exist("mcd"));
        assert!(!exist(""));
    }

    #[test]
    fn test_exist_case_insensitive() {
        assert!(exist_case_insensitive("FR"));
        assert!(exist_case_insensitive("De"));
        assert!(exist_case_insensitive("zW"));
        assert!(exist_case_insensitive("COM"));
        assert!(!exist_case_insensitive("A9292ZW"));
        assert!(!exist_case_insensitive("MCD"));
        assert!(!exist_case_insensitive(""));
        assert!(exist_case_insensitive("xn--fiqs8s"));
        assert!(exist_case_insensitive("XN--FIQS8S"));
    }
}
