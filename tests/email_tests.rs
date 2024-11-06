use tikh_email_parser::EmailAddress;

/// Tests a simple email address.
#[test]
fn test_simple_email() {
    let email_str = "user@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "user");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an email with a comment in the local part.
#[test]
fn test_email_with_comments() {
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

/// Tests an email where the local part is in quotes.
#[test]
fn test_quoted_local() {
    let email_str = "\"user@name\"@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local, "\"user@name\"");
    assert_eq!(email.domain_part, "example.com");
}

/// Tests an invalid email address format.
#[test]
fn test_invalid_email() {
    let email_str = "user@@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}
