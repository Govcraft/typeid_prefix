use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ValidationError {
    ExceedsMaxLength,
    ContainsInvalidCharacters, // More descriptive name
    StartsWithUnderscore,
    EndsWithUnderscore,
    InvalidStartCharacter,
    InvalidEndCharacter,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            Self::ExceedsMaxLength => {
                "Input exceeds 63 characters"
            }
            Self::ContainsInvalidCharacters => {
                "Input contains invalid characters: only lowercase ASCII letters and underscores are allowed"
            }
            Self::StartsWithUnderscore => {
                "Input cannot start with an underscore"
            }
            Self::EndsWithUnderscore => {
                "Input cannot end with an underscore"
            }
            Self::InvalidStartCharacter => {
                "Input must start with a lowercase alphabetic character"
            }
            Self::InvalidEndCharacter => {
                "Input must end with a lowercase alphabetic character"
            }
        };

        #[cfg(feature = "logging")]
        tracing::error!("ValidationError: {}", error_message);

        write!(f, "{error_message}")
    }
}

impl std::error::Error for ValidationError {}