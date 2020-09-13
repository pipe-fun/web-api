use lettre::{SmtpClient, ClientSecurity, Transport, ClientTlsParameters};
use lettre::smtp::authentication::IntoCredentials;
use lettre::smtp::error::SmtpResult;
use native_tls::TlsConnector;
use lettre_email::{EmailBuilder, Email};
use lettre_email::error::Error;
use dotenv_codegen::dotenv;
use std::str::FromStr;

const SMTP_ADDRESS: &str = dotenv!("SMTP_ADDRESS");
const SMTP_PORT: &str = dotenv!("SMTP_PORT");
const EMAIL_USER: &str = dotenv!("EMAIL_USER");
const EMAIL_PASSWORD: &str = dotenv!("EMAIL_PASSWORD");

pub enum EmailType {
    Active,
    ChangePassword,
}

pub fn check_email(to: &str) -> Result<Email, Error> {
    EmailBuilder::new()
        .to(to)
        .from("example@example.com")
        .build()
}

pub fn send_email(to: &str, email_type: EmailType, message: &str) -> SmtpResult {
    let subject = match email_type {
        EmailType::Active => "active your account",
        EmailType::ChangePassword => "check code",
    };

    let email = EmailBuilder::new()
        .to(to)
        .from(EMAIL_USER)
        .subject(subject)
        .text(message)
        .build()
        .unwrap()
        .into();

    let credentials = (EMAIL_USER, EMAIL_PASSWORD).into_credentials();
    let tls = ClientTlsParameters::new(SMTP_ADDRESS.into(), TlsConnector::new().unwrap());
    let client_secure = ClientSecurity::Required(tls);

    let mut client = SmtpClient::new((SMTP_ADDRESS, u16::from_str(SMTP_PORT)
                                          .unwrap_or_else(|_| 587)), client_secure)
        .unwrap()
        .credentials(credentials)
        .smtp_utf8(true)
        .transport();

    client.send(email)
}