//! # Tikh Email Parser
//!
//! This library provides a parser for validating and extracting components of email addresses.

use pest::Parser; // Import the Parser trait
use pest_derive::Parser;
use thiserror::Error;

/// Parser for email addresses.
#[derive(Parser)]
#[grammar = "grammar.pest"] 
pub struct EmailParser;

/// Represents an email address with a local part and domain.
#[derive(Debug)]
pub struct EmailAddress {
    /// Local part of the email address.
    pub local: String,
    /// Domain part of the email address.
    pub domain_part: String,
}

/// Custom error for parsing failures.
#[derive(Error, Debug)]
pub enum EmailParseError {
    /// Error returned when parsing fails.
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

impl EmailAddress {
    /// Parses an email address string and returns an `EmailAddress`.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the email address to parse.
    ///
    /// # Errors
    ///
    /// Returns an `EmailParseError` if the input does not conform to the email grammar.
    pub fn parse(input: &str) -> Result<Self, EmailParseError> {
        let mut parsed = EmailParser::parse(Rule::email, input)?;
        let email_pair = parsed.next().unwrap();

        let mut local = String::new();
        let mut domain_part = String::new();

        for pair in email_pair.into_inner() {
            match pair.as_rule() {
                Rule::local_part => {
                    local = pair.as_str().to_string();
                }
                Rule::domain => {
                    domain_part = pair.as_str().to_string();
                }
                _ => {}
            }
        }

        Ok(EmailAddress { local, domain_part })
    }
}
