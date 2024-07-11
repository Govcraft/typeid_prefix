#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # `TypeID` Prefix
//!
//! This crate provides a type-safe implementation of the `TypePrefix` section of the
//! [TypeID Specification](https://github.com/jetpack-io/typeid).
//!
//! The main type provided by this crate is [`TypeIdPrefix`], which represents a valid
//! `TypeID` prefix. This type ensures that all instances conform to the `TypeID` specification:
//!
//! - Maximum length of 63 characters
//! - Contains only lowercase ASCII letters and underscores
//! - Does not start or end with an underscore
//! - Starts and ends with a lowercase letter
//!
//! ## Features
//!
//! - **Type-safe**: Ensures that `TypeID` prefixes conform to the specification.
//! - **Validation**: Provides robust validation for `TypeID` prefixes.
//! - **Sanitization**: Offers methods to clean and sanitize input strings into valid `TypeID` prefixes.
//! - **Zero-cost abstractions**: Designed to have minimal runtime overhead.
//! - **Optional tracing**: Integrates with the `tracing` crate for logging (optional feature).
//!
//! ## Usage
//!
//! ```rust
//! use typeid_prefix::prelude::*;
//! use std::convert::TryFrom;
//!
//! // Create a TypeIdPrefix from a valid string
//! let prefix = TypeIdPrefix::try_from("user").unwrap();
//! assert_eq!(prefix.as_str(), "user");
//!
//! // Attempt to create from an invalid string
//! let result = TypeIdPrefix::try_from("Invalid_Prefix");
//! assert!(result.is_err());
//!
//! // Sanitize an invalid string
//! let sanitized = "Invalid_Prefix123".create_prefix_sanitized();
//! assert_eq!(sanitized.as_str(), "invalid_prefix");
//! ```
//!
//! ## Optional Tracing
//!
//! When the `instrument` feature is enabled, the crate will log validation errors
//! using the `tracing` crate.

use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(feature = "instrument")]
use tracing;

pub use crate::error::ValidationError;

mod error;
mod traits;
mod type_id_prefix;

pub mod prelude {
    //! A prelude for the `TypeID` prefix crate.
    //!
    //! This module contains the most commonly used items from the crate.
    //!
    //! # Usage
    //!
    //! ```
    //! use typeid_prefix::prelude::*;
    //! ```
    pub use crate::traits::PrefixFactory;
    pub use crate::{TypeIdPrefix, Validate, ValidationError};
}

/// Represents a valid `TypeID` prefix as defined by the `TypeID` specification.
///
/// A `TypeIdPrefix` is guaranteed to:
/// - Have a maximum length of 63 characters
/// - Contain only lowercase ASCII letters and underscores
/// - Not start or end with an underscore
/// - Start and end with a lowercase letter
///
/// # Examples
///
/// ```
/// use typeid_prefix::TypeIdPrefix;
/// use std::convert::TryFrom;
///
/// let prefix = TypeIdPrefix::try_from("valid_prefix").unwrap();
/// assert_eq!(prefix.as_str(), "valid_prefix");
///
/// let invalid = TypeIdPrefix::try_from("Invalid_Prefix");
/// assert!(invalid.is_err());
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct TypeIdPrefix(String);





impl Deref for TypeIdPrefix {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl PartialEq<String> for TypeIdPrefix {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<TypeIdPrefix> for String {
    fn eq(&self, other: &TypeIdPrefix) -> bool {
        self == &other.0
    }
}

// You can also implement PartialEq<&str> if needed
impl PartialEq<&str> for TypeIdPrefix {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<TypeIdPrefix> for &str {
    fn eq(&self, other: &TypeIdPrefix) -> bool {
        self == &other.0
    }
}

/// Implements the `FromStr` trait for `TypeIdPrefix`.
///
/// This implementation allows creating a `TypeIdPrefix` from a string slice,
/// validating the input according to the TypeID specification.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use typeid_prefix::TypeIdPrefix;
///
/// let valid_prefix = TypeIdPrefix::from_str("user").expect("Valid prefix");
/// assert_eq!(valid_prefix.as_str(), "user");
///
/// let invalid_prefix = TypeIdPrefix::from_str("123");
/// assert!(invalid_prefix.is_err());
/// ```
///
/// # Errors
///
/// This method will return a `ValidationError` if the input string does not meet
/// the requirements of a valid TypeID prefix. Possible error conditions include:
///
/// - The input exceeds the maximum allowed length of 63 characters.
/// - The input contains characters other than lowercase ASCII letters and underscores.
/// - The input starts or ends with an underscore.
/// - The input does not start or end with a lowercase alphabetic character.
///
/// For more details on error conditions, see the `ValidationError` enum.
impl FromStr for TypeIdPrefix {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)
    }
}
impl TryFrom<String> for TypeIdPrefix
{
    type Error = ValidationError;

    /// Attempts to create a `TypeIdPrefix` from a `String`.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the input string is not a valid `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::TypeIdPrefix;
    /// use std::convert::TryFrom;
    ///
    /// let valid = TypeIdPrefix::try_from("valid_prefix".to_string()).unwrap();
    /// assert_eq!(valid.as_str(), "valid_prefix");
    ///
    /// let invalid = TypeIdPrefix::try_from("Invalid_Prefix".to_string());
    /// assert!(invalid.is_err());
    /// ```
    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::validate(input.as_ref())
    }
}

impl TryFrom<&str> for TypeIdPrefix
{
    type Error = ValidationError;

    /// Attempts to create a `TypeIdPrefix` from a string slice.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the input string is not a valid `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::TypeIdPrefix;
    /// use std::convert::TryFrom;
    ///
    /// let valid = TypeIdPrefix::try_from("valid_prefix").unwrap();
    /// assert_eq!(valid.as_str(), "valid_prefix");
    ///
    /// let invalid = TypeIdPrefix::try_from("Invalid_Prefix");
    /// assert!(invalid.is_err());
    /// ```
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::validate(input)
    }
}


impl TypeIdPrefix {
    fn validate(input: &str) -> Result<Self, ValidationError> {
        if input.len() > 63 {
            return Err(ValidationError::ExceedsMaxLength);
        }

        if input.is_empty() {
            return Ok(Self(input.to_string()));
        }

        if !input.is_ascii() {
            return Err(ValidationError::ContainsInvalidCharacters);
        }

        if input.starts_with('_') {
            return Err(ValidationError::StartsWithUnderscore);
        }

        if input.ends_with('_') {
            return Err(ValidationError::EndsWithUnderscore);
        }

        if !input.starts_with(|c: char| c.is_ascii_lowercase()) {
            return Err(ValidationError::InvalidStartCharacter);
        }

        if !input.ends_with(|c: char| c.is_ascii_lowercase()) {
            return Err(ValidationError::InvalidEndCharacter);
        }

        if !input.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Err(ValidationError::ContainsInvalidCharacters);
        }

        Ok(Self(input.to_string()))
    }

    fn clean_inner(input: &str) -> String {
        let mut result = input.to_string();
        result = result.to_lowercase();
        // Safely truncate to 63 characters if necessary
        if result.len() > 63 {
            result = result.chars().take(63).collect();
        }

        result = result.to_ascii_lowercase().chars()
            .filter(|&c| (c.is_ascii_lowercase() || c == '_') && c.is_ascii())
            .collect::<String>();

        // Remove leading and trailing underscores
        while result.starts_with('_') {
            result.remove(0);
        }

        while result.ends_with('_') {
            result.pop();
        }

        result
    }

    /// Returns a string slice of the `TypeID` prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use typeid_prefix::prelude::*;
    /// use std::convert::TryFrom;
    ///
    /// let prefix = TypeIdPrefix::try_from("valid_prefix").unwrap();
    /// assert_eq!(prefix.as_str(), "valid_prefix");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


impl fmt::Display for TypeIdPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use crate::traits::PrefixFactory;

    use super::*;

    #[test]
    fn test_type_id_spaces_sanitize() {
        assert_eq!(
            "Invalid String with Spaces!!__".create_prefix_sanitized().as_str(),
            "invalidstringwithspaces"
        );
    }

    #[test]
    fn test_type_id_truncation() {
        assert_eq!(
            "A_valid_string_that_is_way_too_long_and_should_be_truncated_to_63_chars".create_prefix_sanitized().as_str(),
            "a_valid_string_that_is_way_too_long_and_should_be_truncated_to"
        );
    }

    #[test]
    fn test_type_id_underscores_sanitize() {
        assert_eq!(
            "_underscores__everywhere__".create_prefix_sanitized().as_str(),
            "underscores__everywhere"
        );
    }

    #[test]
    fn test_typeid_prefix_non_ascii() {
        assert!(TypeIdPrefix::try_from("🌀").is_err());
        let sanitized_input = "🌀".create_prefix_sanitized();
        assert!(sanitized_input.as_str().is_empty(), "Prefix was not empty: {sanitized_input}");
    }

    #[test]
    fn test_typeid_prefix_empty() {
        assert!(TypeIdPrefix::try_from("").is_ok());
    }

    #[test]
    fn test_typeid_prefix_single_char() {
        assert!(TypeIdPrefix::try_from("a").is_ok());
    }

    #[test]
    fn test_typeid_prefix_valid_string() {
        assert!(TypeIdPrefix::try_from("valid_string").is_ok());
    }

    #[test]
    fn test_typeid_prefix_with_underscores() {
        assert!(TypeIdPrefix::try_from("valid_string_with_underscores").is_ok());
    }

    #[test]
    fn test_typeid_prefix_exceeds_max_length() {
        let input = "a_valid_string_with_underscores_and_length_of_63_characters_____";
        assert_eq!(
            TypeIdPrefix::try_from(input).unwrap_err(),
            ValidationError::ExceedsMaxLength
        );
        assert_eq!(
            input.create_prefix_sanitized().as_str(),
            "a_valid_string_with_underscores_and_length_of__characters"
        );
    }

    #[test]
    fn test_typeid_prefix_invalid_characters() {
        assert_eq!(
            TypeIdPrefix::try_from("InvalidString").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!("InvalidString".create_prefix_sanitized().as_str(), "invalidstring");
    }

    #[test]
    fn test_typeid_prefix_starts_with_underscore() {
        assert_eq!(
            TypeIdPrefix::try_from("_invalid").unwrap_err(),
            ValidationError::StartsWithUnderscore
        );
        assert_eq!("_invalid".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_ends_with_underscore() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid_").unwrap_err(),
            ValidationError::EndsWithUnderscore
        );
        assert_eq!("invalid_".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_invalid_characters_with_spaces() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid string with spaces").unwrap_err(),
            ValidationError::ContainsInvalidCharacters
        );
        assert_eq!("invalid string with spaces".create_prefix_sanitized().as_str(), "invalidstringwithspaces");
    }

    #[test]
    fn test_typeid_prefix_max_length() {
        let input = "a".repeat(63);
        assert!(TypeIdPrefix::try_from(input.as_str()).is_ok());
    }

    #[test]
    fn test_typeid_prefix_max_length_exceeded() {
        let input = "a".repeat(64);
        assert_eq!(
            TypeIdPrefix::try_from(input.as_str()).unwrap_err(),
            ValidationError::ExceedsMaxLength
        );
        assert_eq!(input.create_prefix_sanitized().as_str(), "a".repeat(63));
    }

    #[test]
    fn test_typeid_prefix_contains_uppercase() {
        assert_eq!(
            TypeIdPrefix::try_from("InvalidString").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!("InvalidString".create_prefix_sanitized().as_str(), "invalidstring");
    }

    #[test]
    fn test_typeid_prefix_non_alphanumeric() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid_string!").unwrap_err(),
            ValidationError::InvalidEndCharacter
        );
        assert_eq!("invalid_string!".create_prefix_sanitized().as_str(), "invalid_string");
    }

    #[test]
    fn test_typeid_prefix_numeric_start() {
        assert_eq!(
            TypeIdPrefix::try_from("1invalid").unwrap_err(),
            ValidationError::InvalidStartCharacter
        );
        assert_eq!("1invalid".create_prefix_sanitized().as_str(), "invalid");
    }

    #[test]
    fn test_typeid_prefix_numeric_end() {
        assert_eq!(
            TypeIdPrefix::try_from("invalid1").unwrap_err(),
            ValidationError::InvalidEndCharacter
        );
        assert_eq!("invalid1".create_prefix_sanitized().as_str(), "invalid");
    }
}