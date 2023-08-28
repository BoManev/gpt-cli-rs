use anyhow::Context;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use log::info;
use models::CompletionBody;

use crate::models::CompletionResponse;

mod models;
fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let key = std::env::var("OPENAI_KEY").context("OpenAI API key not found")?;

    let args = Args::parse();
    let body = CompletionBody {
        model: args.model.clone(),
        max_tokens: Some(args.tokens.unwrap_or(200)),
        prompt: format!(
            "Please provide me with a linux command to {}. Answer with the full command",
            args.prompt.as_str()
        ),
        temperature: Some(0.0),
        stream: Some(false),
        top_p: None,
    };

    info!("input args: {:#?}", args);

    let response = reqwest::blocking::Client::new()
        .post("https://api.openai.com/v1/completions")
        .json(&body)
        .bearer_auth(key)
        .send()
        .context("Failed to send request for completion to OpenAI")?
        .error_for_status()?;

    info!("response status: {}", response.status());

    let body: CompletionResponse = response.json().context("Failed to parse response")?;

    info!("response body: {:#?}", args);

    let command = body.choices[0].text.clone().unwrap();
    println!("\nYour command is: {}", command.trim());

    let selections = &["Copy to clipboard", "Cancel"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Continue...")
        .default(9)
        .items(&selections[..])
        .interact()?;

    if selection == 0 {
        cli_clipboard::set_contents(command.to_owned()).unwrap();
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub prompt: String,

    #[arg(short, long)]
    pub tokens: Option<i32>,

    #[arg(short, long, default_value = "text-davinci-003")]
    pub model: String,
}
