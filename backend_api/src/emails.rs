use lettre_email::Email;
use tera::Context;

use crate::app::TEMPLATES;
use crate::errors::EmailError;
use lettre::{
    smtp::authentication::{Credentials, Mechanism},
    ClientSecurity, ClientTlsParameters, EmailAddress, Envelope, SendableEmail, SmtpClient,
    Transport,
};
use native_tls::{Protocol, TlsConnector};
use std::{env, error::Error};
use tokio::task::block_in_place;

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
                .trim()
                .to_string(),
            from_address: env::var("MAIL_FROM_ADDRESS")
                .expect("MAIL_FROM_ADDRESS is not set")
                .trim()
                .to_string(),
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
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
        let tls_parameters = ClientTlsParameters::new(self.host.clone(), tls_builder.build()?);
        let mut mailer = SmtpClient::new(
            (self.host.as_str(), self.port),
            ClientSecurity::Opportunistic(tls_parameters),
        )?;
        if let (Some(username), Some(password)) = (self.username.clone(), self.password.clone()) {
            if username.trim().len() > 0 && password.trim().len() > 0 {
                let credentials: Credentials =
                    Credentials::new(username.trim().to_string(), password.trim().to_string());
                mailer = mailer
                    .credentials(credentials)
                    .authentication_mechanism(Mechanism::Plain);
            }
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
        let body_html = format!(
            "<p>Email to reply to: {}</p><br /><br /><br /><pre>{}</pre>",
            from_email, body
        );
        let body_text = format!("Email to reply to: {}\n\n\n{}", from_email, body);
        let email = Email::builder()
            // Addresses can be specified by the tuple (email, alias)
            .to(self.contact_address.clone())
            // ... or by an address only
            .from(self.from_address.clone())
            .subject(subject)
            .alternative(body_html, body_text)
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

    pub async fn send_reset_password_email(
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
            .subject(format!(
                "Hi {}, a password reset was request on your behalf",
                username
            ))
            .alternative(
                self.get_reset_password_email_text(username.clone(), token.clone()),
                self.get_reset_password_email_html(username.clone(), token.clone()),
            )
            .build()?;

        let _ = self.send_email(email.into()).await?;
        Ok(())
    }

    fn get_verification_email_text(&self, username: String, token: String) -> String {
        let mut context = Context::new();
        context.insert("display_name", &username);
        context.insert("token", &token);
        context.insert("website_url", &self.website_url);
        match TEMPLATES.render("emails/verify_email/verify_email_html.html", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::from("Error rendering email")
            }
        }
    }

    fn get_verification_email_html(&self, username: String, token: String) -> String {
        let mut context = Context::new();
        context.insert("display_name", &username);
        context.insert("token", &token);
        context.insert("website_url", &self.website_url);
        match TEMPLATES.render("emails/verify_email/verify_email_text.html", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::from("Error rendering email")
            }
        }
    }

    fn get_reset_password_email_text(&self, username: String, token: String) -> String {
        let mut context = Context::new();
        context.insert("display_name", &username);
        context.insert("token", &token);
        context.insert("website_url", &self.website_url);
        match TEMPLATES.render("emails/reset_password/reset_password_html.html", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::from("Error rendering email")
            }
        }
    }

    fn get_reset_password_email_html(&self, username: String, token: String) -> String {
        let mut context = Context::new();
        context.insert("display_name", &username);
        context.insert("token", &token);
        context.insert("website_url", &self.website_url);
        match TEMPLATES.render("emails/reset_password/reset_password_text.html", &context) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                let mut cause = e.source();
                while let Some(e) = cause {
                    println!("Reason: {}", e);
                    cause = e.source();
                }
                String::from("Error rendering email")
            }
        }
    }
}
