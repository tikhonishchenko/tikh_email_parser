
use tikh_email_parser::EmailAddress;

/// Tests a simple valid email address.
#[test]
fn test_simple_email() {
    let email_str = "user@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with CFWS (comments and folding white space) in various places.
#[test]
fn test_email_with_cfws() {
    let email_str = "(comment) user(comment)@example.com (comment)";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with a comment in the local part.
#[test]
fn test_email_with_comments_in_local_part() {
    let email_str = "user(comment)@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with special characters in the local part.
#[test]
fn test_email_with_special_chars() {
    let email_str = "user.name+tag@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user.name+tag");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with a quoted local part.
#[test]
fn test_quoted_local_part() {
    let email_str = "\"user@name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user@name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with an invalid format with multiple "@" symbols.
#[test]
fn test_invalid_email_multiple_at_symbols() {
    let email_str = "user@@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}

/// Tests an email address with consecutive dots in the local part.
#[test]
fn test_invalid_local_part_consecutive_dots() {
    let email_str = "user..name@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}

/// Tests an email with escaped characters in the quoted local part.
#[test]
fn test_email_with_escaped_characters_in_local_part() {
    let email_str = "\"user@name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user@name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests a fully valid email with quoted strings and comments in both local and domain parts.
#[test]
fn test_complex_valid_email_with_quoted_strings_and_comments() {
    let email_str = "(comment) \"user\"@example.com (comment)";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with only whitespace around it (invalid scenario).
#[test]
fn test_invalid_email_with_whitespace_only() {
    let email_str = "     ";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}

/// Tests an email with dot_atom structure in the local part.
#[test]
fn test_email_with_dot_atom_local_part() {
    let email_str = "user.name+tag@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user.name+tag");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with a quoted string in the local part.
#[test]
fn test_email_with_quoted_string_local_part() {
    let email_str = "\"user name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with valid qcontent in the quoted string.
#[test]
fn test_email_with_qcontent_in_quoted_string() {
    let email_str = "\"user\\@name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user\\@name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with various qtext characters in the quoted string.
#[test]
fn test_email_with_qtext_characters_in_quoted_string() {
    let email_str = "\"user name with spaces\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user name with spaces\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with quoted pair in the quoted string.
#[test]
fn test_email_with_quoted_pair_in_quoted_string() {
    let email_str = "\"user\\@name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user\\@name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with CFWS (comments and folding whitespace).
#[test]
fn test_email_with_cfws_comments_and_whitespace() {
    let email_str = "(comment) user@ example.com (comment)";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with multiple nested comments in CFWS.
#[test]
fn test_email_with_multiple_nested_comments_in_cfws() {
    let email_str = "user@(nested(comment))@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}

/// Tests an email with a simple comment.
#[test]
fn test_email_with_simple_comment() {
    let email_str = "user(comment)@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with CRLF (line break) in folding whitespace.
#[test]
fn test_email_with_crlf_in_fws() {
    let email_str = "user@example.com\r\n";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}
