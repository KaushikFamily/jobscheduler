use std::{env};

use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType, transport::smtp::authentication::Credentials};


// Duplicate code for sending emails, need to refactor and see how to use traits to advantage
pub async fn feed_fishes_v1(
    task: &str
) -> Result<String, Box<dyn std::error::Error>> 
{
    dotenvy::dotenv()?;

    let user_email = env::var("SMTP_USERNAME")?;
    let user_password = env::var("SMTP_PASSWORD")?;
    let recipients: Vec<String> = env::var("SMTP_EMAIL_RECIPIENTS")?
        .split(',')
        .map(|email| email.trim().to_string())
        .collect()
    ;

    let mut builder = Message::builder()
        .from(user_email.parse()?)
        .subject("Feed Fishes 9:45 PM")
        .header(ContentType::TEXT_PLAIN)
    ;

    for recipient in recipients {
        builder = builder.to(recipient.parse()?);
    }

    let email = builder
        .body(String::from("Feed the Fishes"))?
    ;

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

    Ok(String::from("SUCCESSFUL EXECUTION"))
}