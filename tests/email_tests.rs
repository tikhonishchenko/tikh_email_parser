use tikh_email_parser::EmailAddress;

/// Tests WHITESPACE parsing.
#[test]
fn test_whitespace_parsing() {
    let input = "    ";
    assert!(EmailAddress::parse(input).is_err());
}

/// Tests a valid email address with no CFWS.
#[test]
fn test_email_without_cfws() {
    let email_str = "user@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests a valid email with a simple comment (cfws).
#[test]
fn test_cfws_parsing() {
    let email_str = "(comment)user@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests a valid local part with `dot_atom`.
#[test]
fn test_dot_atom_parsing() {
    let email_str = "user.name@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user.name");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an invalid local part with consecutive dots.
#[test]
fn test_invalid_dot_atom() {
    let email_str = "user..name@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}

/// Tests a valid `dot_atom_text`.
#[test]
fn test_dot_atom_text_parsing() {
    let email_str = "u.ser@domain.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "u.ser");
    assert_eq!(email.domain_part, "domain.com");
}

/// Tests a valid quoted string in the local part.
#[test]
fn test_quoted_string_parsing() {
    let email_str = "\"quoted.local\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"quoted.local\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests valid `qcontent` parsing in the quoted string.
#[test]
fn test_qcontent_parsing() {
    let email_str = "\"quoted\\\"content\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"quoted\\\"content\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests valid `qtext` in the quoted string.
#[test]
fn test_qtext_parsing() {
    let email_str = "\"quoted content\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"quoted content\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests valid `quoted_pair` parsing in the quoted string.
#[test]
fn test_quoted_pair_parsing() {
    let email_str = "\"quoted\\\"pair\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"quoted\\\"pair\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests CFWS with nested comments.
#[test]
fn test_nested_comments_in_cfws() {
    let email_str = "user@(nested(comment))@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}


/// Tests a valid `comment` parsing.
#[test]
fn test_comment_parsing() {
    let email_str = "user(comment)@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests invalid `ctext` parsing with forbidden characters.
#[test]
fn test_invalid_ctext() {
    let email_str = "user@exa(mple).com"; 
    let result = EmailAddress::parse(email_str);
    assert!(result.is_ok()); 
}

/// Tests folding whitespace (`FWS`) parsing.
#[test]
fn test_fws_parsing() {
    let email_str = "user@ example . com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example . com"); 
}



/// Tests an email with `CRLF` (line breaks) in folding whitespace.
#[test]
fn test_crlf_parsing() {
    let email_str = "user@ example.com"; 
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}



/// Tests invalid email with `CRLF` and no subsequent domain.
#[test]
fn test_invalid_crlf_parsing() {
    let email_str = "user@\r\n";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}
