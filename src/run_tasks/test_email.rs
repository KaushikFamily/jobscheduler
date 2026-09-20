use std::env;

use lettre::{AsyncSmtpTransport, AsyncTransport, Message, SmtpTransport, Tokio1Executor, Transport, message::header::ContentType, transport::smtp::authentication::Credentials};

pub async fn test_email_v1(
) -> Result<String, Box<dyn std::error::Error>> 
{
    dotenvy::dotenv()?;

    let user_email = env::var("SMTP_USERNAME")?;
    let user_password = env::var("SMTP_PASSWORD")?;

    let email = Message::builder()
        .from(user_email.parse()?)
        .to(user_email.parse()?)
        .subject("Hello from Rust")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("This email was sent from Rust!"))?;

    let credentials = Credentials::new(
        user_email,
        user_password
    );

    let mailer =
        AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
            .credentials(credentials)
            .build();

    mailer.send(email).await?;

    println!("Email sent!");

    Ok(String::from("returning this stub for now"))
}