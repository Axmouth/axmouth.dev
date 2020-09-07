use lettre_email::Email;

use crate::{errors::EmailError, util::simple_error_response};
use lettre::{
    smtp::authentication::{Credentials, Mechanism},
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, SendableEmail, SmtpClient,
    Transport,
};
use native_tls::{Protocol, TlsConnector};
use std::env;
use tokio::task::block_in_place;
use warp::{hyper::StatusCode, reject};

#[derive(Debug, Clone)]
pub struct EmailSender {
    pub contact_address: String,
    pub from_address: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub website_url: String,
}

impl EmailSender {
    pub fn new() -> Self {
        EmailSender {
            contact_address: env::var("CONTACT_ADDRESS")
                .expect("CONTACT_ADDRESS is not set")
                .trim(),
            from_address: env::var("MAIL_FROM_ADDRESS")
                .expect("MAIL_FROM_ADDRESS is not set")
                .trim(),
            host: env::var("MAIL_HOST").expect("MAIL_HOST is not set"),
            port: env::var("MAIL_PORT")
                .expect("MAIL_PORT is not set")
                .parse()
                .expect("MAIL_PORT is invalid"),
            password: env::var("MAIL_PASSWORD").ok(),
            username: env::var("MAIL_USERNAME").ok(),
            website_url: env::var("WEBSITE_URL").expect("WEBSITE_URL is not set"),
        }
    }
}

impl EmailSender {
    pub async fn send_email(
        &self,
        email: SendableEmail,
    ) -> Result<lettre::smtp::response::Response, EmailError> {
        println!("{:?} {:?}", self.username, self.password);
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
        let tls_parameters = ClientTlsParameters::new(self.host.clone(), tls_builder.build()?);
        let mut mailer = SmtpClient::new(
            (self.host.as_str(), self.port),
            ClientSecurity::Opportunistic(tls_parameters),
        )?;
        if let (Some(username), Some(password)) = (self.username.clone(), self.password.clone()) {
            let credentials: Credentials = Credentials::new(username, password);
            mailer = mailer
                .credentials(credentials)
                .authentication_mechanism(Mechanism::Plain);
        }
        let mut mailer = mailer.transport();
        // Send the email
        Ok(block_in_place(|| mailer.send(email))?)
    }

    pub async fn send_contact_email(
        &self,
        from_email: String,
        subject: String,
        body: String,
    ) -> Result<(), EmailError> {
        let email = Email::builder()
            // Addresses can be specified by the tuple (email, alias)
            .to(self.contact_address.clone())
            // ... or by an address only
            .from(from_email)
            .subject(subject)
            .alternative(body.clone(), body)
            .build()?;

        let _ = self.send_email(email.into()).await?;
        Ok(())
    }

    pub async fn send_email_verification_email(
        &self,
        user_email: String,
        username: String,
        token: String,
    ) -> Result<(), EmailError> {
        let email = Email::builder()
            // Addresses can be specified by the tuple (email, alias)
            .to((user_email.clone(), username.clone()))
            // ... or by an address only
            .from(self.from_address.clone())
            .subject(format!("Hi {}, we need you to verify your email", username))
            .alternative(
                self.get_verification_email_html(username.clone(), token.clone()),
                self.get_verification_email_text(username.clone(), token.clone()),
            )
            .build()?;

        let _ = self.send_email(email.into()).await?;
        Ok(())
    }

    fn get_verification_email_text(&self, username: String, token: String) -> String {
        format!(
            "Hello {},
        Please click the following link to cerify your email:
        <a href=\"{}/verify-email/{}\">Verify Email</a>",
            username, self.website_url, token
        )
    }

    fn get_verification_email_html(&self, username: String, token: String) -> String {
        format!(
            "Hello {},
        Please click the following link to cerify your email:
        {}/verify-email/{}",
            username, self.website_url, token
        )
    }
}
