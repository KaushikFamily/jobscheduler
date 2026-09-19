use lettre::{Message, SmtpTransport, Transport, message::header::ContentType, transport::smtp::authentication::Credentials};
use reqwest::StatusCode;

use crate::models::Task;

pub async fn test_email_v1(
    task: Task
) -> Result<String, Box<dyn std::error::Error>> 
{
    let email = Message::builder()
        .from("your_email@gmail.com".parse()?)
        .to("recipient@example.com".parse()?)
        .subject("Hello from Rust")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("This email was sent from Rust!"))?;

    let credentials = Credentials::new(
        "your_email@gmail.com".to_string(),
        "your_app_password".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(credentials)
        .build();

    mailer.send(&email)?;

    println!("Email sent!");

    Ok(String::from("returning this stub for now"))
}