use tikh_email_parser::EmailAddress;

#[test]
fn test_simple_email() {
    let email_str = "test@example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local_part, "test");
    assert_eq!(email.domain, "example.com");
}

#[test]
fn test_email_with_subdomain() {
    let email_str = "test@mail.example.com";
    let email = EmailAddress::parse(email_str).unwrap();
    assert_eq!(email.local_part, "test");
    assert_eq!(email.domain, "mail.example.com");
}

#[test]
fn test_invalid_email() {
    let email_str = "test@@example.com";
    let result = EmailAddress::parse(email_str);
    assert!(result.is_err());
}
