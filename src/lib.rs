use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct EmailParser;

#[derive(Debug)]
pub struct EmailAddress {
    pub local_part: String,
    pub domain: String,
}

#[derive(Error, Debug)]
pub enum EmailParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

impl EmailAddress {
    pub fn parse(input: &str) -> Result<Self, EmailParseError> {
        let mut parsed = EmailParser::parse(Rule::email, input)?;
        let email_pair = parsed.next().unwrap();

        let mut local_part = String::new();
        let mut domain = String::new();

        for pair in email_pair.into_inner() {
            match pair.as_rule() {
                Rule::local_part => {
                    local_part = pair.as_str().to_string();
                }
                Rule::domain => {
                    domain = pair.as_str().to_string();
                }
                _ => {}
            }
        }

        Ok(EmailAddress { local_part, domain })
    }
}
