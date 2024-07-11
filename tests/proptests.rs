use proptest::prelude::*;
use std::convert::TryFrom;
use typeid_prefix::{Sanitize, TypeIdPrefix};

mod proofs;

proptest! {
    #[test]
    fn test_typeidprefix_try_from_and_sanitize(input in "\\PC*") {
        let try_from_result = TypeIdPrefix::try_from(input.clone());
        let sanitized = input.create_sanitized_prefix();

        // Test TypeIdPrefix::try_from
        if input.len() > 63 {
            prop_assert!(try_from_result.is_err());
        } else if input.is_empty() {
            prop_assert!(try_from_result.is_ok());
        } else {
            let is_ascii = input.is_ascii();
            let starts_with_valid_char = input.chars().next().map_or(false, |c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.chars().last().map_or(false, |c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.chars().all(|c| c.is_ascii_lowercase() || c == '_');
            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                prop_assert!(try_from_result.is_err());
            } else {
                prop_assert!(try_from_result.is_ok());
            }
        }

        // Test from_sanitized
        prop_assert!(sanitized.len() <= 63);
        prop_assert!(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!sanitized.starts_with('_'));
        prop_assert!(!sanitized.ends_with('_'));

        // Ensure sanitized version is always valid
        prop_assert!(TypeIdPrefix::try_from(sanitized.as_str()).is_ok());
    }

    #[test]
    fn test_typeidprefix_try_from_str_and_sanitize(input in "\\PC*") {
        let try_from_result = TypeIdPrefix::try_from(input.as_str());
        let sanitized = input.create_sanitized_prefix();

        // Test TypeIdPrefix::try_from for &str
        if input.len() > 63 {
            prop_assert!(try_from_result.is_err());
        } else if input.is_empty() {
            prop_assert!(try_from_result.is_ok());
        } else {
            let is_ascii = input.is_ascii();
            let starts_with_valid_char = input.chars().next().map_or(false, |c| c.is_ascii_lowercase());
            let ends_with_valid_char = input.chars().last().map_or(false, |c| c.is_ascii_lowercase());
            let contains_only_valid_chars = input.chars().all(|c| c.is_ascii_lowercase() || c == '_');
            if !is_ascii || !starts_with_valid_char || !ends_with_valid_char || !contains_only_valid_chars {
                prop_assert!(try_from_result.is_err());
            } else {
                prop_assert!(try_from_result.is_ok());
            }
        }

        // Test from_sanitized (same as in previous test)
        prop_assert!(sanitized.len() <= 63);
        prop_assert!(sanitized.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!sanitized.starts_with('_'));
        prop_assert!(!sanitized.ends_with('_'));

        // Ensure sanitized version is always valid
        prop_assert!(TypeIdPrefix::try_from(sanitized.as_str()).is_ok());
    }

    #[test]
    fn test_typeidprefix_clean(input in ".{0,100}") {
        let cleaned = input.create_sanitized_prefix();
        prop_assert!(cleaned.len() <= 63);
        prop_assert!(cleaned.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
        prop_assert!(!cleaned.starts_with('_'));
        prop_assert!(!cleaned.ends_with('_'));
        prop_assert!(TypeIdPrefix::try_from(cleaned.as_str()).is_ok());
    }
}