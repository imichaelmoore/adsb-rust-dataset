//! This module connects to a specified TCP stream, reads lines from it, and sends
//! batches of parsed messages to a web service.
//!
//! Configuration options can be set through command line arguments or environment
//! variables. Mandatory configurations include DATASET_API_WRITE_TOKEN, DUMP1090_HOST, 
//! and DUMP1090_PORT. BATCH_SIZE is optional and defaults to 500.
//! If a required configuration is not set, the application will exit with a descriptive
//! error message.
//!
//! For example, setting configurations via environment variables can be done as:
//! ```bash
//! export DATASET_API_WRITE_TOKEN=your_token
//! export DUMP1090_HOST=your_host
//! export DUMP1090_PORT=your_port
//! export BATCH_SIZE=your_batch_size
//! export 1090_COLLECTOR=your_collector
//! ```
//!
//! Alternatively, they can be provided as command line arguments in the format:
//! `--arg_name arg_value`, e.g. `--DATASET_API_WRITE_TOKEN your_token`

use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use reqwest;
use serde_json::{json, Value};
use uuid::Uuid;
use std::collections::VecDeque;
use std::env;
use crate::parse::{parse, SBS1Message};

mod parse;

fn get_argument_or_env(var_name: &str, default_value: Option<&str>) -> String {
    let arg_prefix = format!("--{}", var_name.to_lowercase());

    let arg_value = env::args()
        .find_map(|arg| {
            if arg.to_lowercase().starts_with(&arg_prefix) {
                if let Some(index) = arg.find('=') {
                    // Extract the value after '='
                    Some(arg[index + 1..].to_string())
                } else {
                    // Use the next argument as the value
                    env::args().skip_while(|a| a.to_lowercase() != arg.to_lowercase()).nth(1)
                }
            } else {
                None
            }
        });
    
    arg_value.unwrap_or_else(|| env::var(var_name).unwrap_or_else(|_| {
        if let Some(default) = default_value {
            default.to_string()
        } else {
            eprintln!("Error: {} must be set via command-line argument or environment variable.", var_name);
            eprintln!("Example: `--{}=value` or `--{} value` or `export {}=value`", var_name.to_lowercase(), var_name.to_lowercase(), var_name);
            std::process::exit(1);
        }
    }))
}


const DEFAULT_BATCH_SIZE: usize = 500;

/// The main entry point of the application.
///
/// This function connects to the DUMP1090 TCP service, reads messages, parses them,
/// and sends them in batches to the DataSet web service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset_api_write_token = get_argument_or_env("DATASET_API_WRITE_TOKEN", None);
    let dump1090_host = get_argument_or_env("DUMP1090_HOST", None);
    let dump1090_port: u32 = get_argument_or_env("DUMP1090_PORT", None).parse().unwrap();
    let batch_size: usize = get_argument_or_env("BATCH_SIZE", Some(&DEFAULT_BATCH_SIZE.to_string())).parse().unwrap();
    let collector = get_argument_or_env("1090_COLLECTOR", Some("dump1090"));

    // Connecting to a TCP stream
    let stream = TcpStream::connect(format!("{}:{}", dump1090_host, dump1090_port.to_string()))?;
    let reader = BufReader::new(stream);

    // Initialize a double-ended queue with the specified capacity.
    let mut messages: VecDeque<SBS1Message> = VecDeque::with_capacity(batch_size);
    
    // Iterate over each line from the TCP stream.
    for line in reader.lines() {
        if let Ok(msg) = line {
            // Parse the line into an SBS1Message.
            if let Some(parsed) = parse(&msg) {
                messages.push_back(parsed);
                
                // Send the collected messages when the queue reaches the batch size.
                if messages.len() >= batch_size {
                    send_to_service(messages.drain(..).collect(), &dataset_api_write_token, &collector).await?;
                }
            }
        }
    }
    
    // Send any remaining messages if there are any left in the queue.
    if !messages.is_empty() {
        send_to_service(messages.drain(..).collect(), &dataset_api_write_token, &collector).await?;
    }

    Ok(())
}

/// Send a batch of parsed messages to the DataSet web service.
///
/// This function constructs the payload for the DataSet web service, sends it, 
/// and logs the response.
///
/// # Arguments
///
/// * `messages` - A vector of parsed SBS1 messages to send to the DataSet web service.
/// * `dataset_api_write_token` - The API write token for the DataSet web service.
/// * `collector` - The collector (or source) identifier.
///
/// # Returns
///
/// A Result indicating the success or failure of the operation.
async fn send_to_service(messages: Vec<SBS1Message>, dataset_api_write_token: &str, collector: &str) -> Result<(), reqwest::Error> {
    // Construct the event payload for each message.
    let events: Vec<Value> = messages.into_iter().map(|message| {
        json!({
            "parser": "adsb",
            "ts": message.timestamp,
            "source": collector,
            "collector": "imichaelmoore/adsb-rust-dataset",
            "sev": 3,
            "attrs": {"message": message}
        })
    }).collect();

    // Construct the final payload to be sent to the DataSet web service.
    let payload = json!({
        "session": Uuid::new_v4(),
        "sessionInfo": {
            "source": collector,
            "collector": "imichaelmoore/adsb-rust-dataset",
        },
        "events": events,
        "threads": []
    });

    // println!("{}", serde_json::to_string_pretty(&payload).unwrap());


    // Send the payload to the DataSet web service.
    let client = reqwest::Client::new();
    let res = client.post("https://app.scalyr.com/api/addEvents")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", dataset_api_write_token))
        .json(&payload)
        .send()
        .await?;

    // Log the response from the DataSet web service.
    println!("Response: {:?}", res.text().await?);

    Ok(())
}
