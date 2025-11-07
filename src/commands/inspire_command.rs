/**
 * =========================================
 * Note: This is an internal command intended
 * for development use only. You may remove it
 * depending on your specific use case or how
 * you plan to use this tool.
 * =========================================
 */
use crate::modules::termenu::Termenu;
use colored::Colorize;
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub fn register() -> Termenu {
    let mut inspire_cmd: Termenu = Termenu::new_async_command(
        "inspire",
        "Fetches and displays a random inspirational quote.",
        |_options: HashMap<String, Option<String>>| {
            Box::pin(async move {
                println!("Fetching a random inspirational quote...\n");

                // Create a client that ignores SSL certificate validation
                let client = reqwest::Client::builder()
                    .danger_accept_invalid_certs(true)
                    .build()
                    .map_err(|e| format!("Client build failed: {}", e))?;

                // API endpoint for random quotes
                let resp: reqwest::Response = client
                    .get("https://api.quotable.io/random")
                    .send()
                    .await
                    .map_err(|e| format!("Request failed: {}", e))?;

                let text = resp
                    .text()
                    .await
                    .map_err(|e| format!("Read failed: {}", e))?;

                let resp_json: Value =
                    serde_json::from_str(&text).map_err(|e| format!("Invalid JSON: {}", e))?;

                let quote = resp_json["content"].as_str().unwrap_or("No quote found.");
                let author = resp_json["author"].as_str().unwrap_or("Unknown");

                println!("💡 \"{}\"", quote.red());
                println!("   — {}", author);

                Ok(())
            }) as Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        },
    );

    inspire_cmd.add_option(
        "--category",
        "Specify a quote category (optional, not used in current API).",
    );

    inspire_cmd
}
