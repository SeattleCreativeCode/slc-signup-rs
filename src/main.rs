use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::Level;
use serde_derive::{Deserialize, Serialize};
use simple_logger;
use slack_hook::{self, PayloadBuilder, Slack};

const HOOK_URL: &'static str = include_str!("hook_url.txt");

#[derive(Debug, Deserialize)]
struct SignupEvent {
    email: String,
}

#[derive(Debug, Serialize)]
struct SignupOutput {}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(Level::Debug)?;
    lambda!(handler);

    Ok(())
}

fn handler(e: SignupEvent, c: Context) -> Result<SignupOutput, HandlerError> {
    let email = e.email;
    let trace_id = c.xray_trace_id;
    send_message(&email, &trace_id).map_err(|_| HandlerError::from("Send Error"))?;
    Ok(SignupOutput {})
}

fn send_message(email: &str, trace_id: &str) -> Result<(), slack_hook::Error> {
    let slack = Slack::new(HOOK_URL)?;
    let payload = PayloadBuilder::new()
        .text(format!(
            "New Invite Request: {} (xray: {})",
            email, trace_id
        ))
        .channel("#admin")
        .username("Inviter2000")
        .icon_emoji(":email:")
        .build()?;

    slack.send(&payload)
}
