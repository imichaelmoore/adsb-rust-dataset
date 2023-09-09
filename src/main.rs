//! This module connects to a specified TCP stream, reads lines from it, and sends
//! batches of parsed messages to a web service.

use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use reqwest;
use serde_json::{json, Value};
use uuid::Uuid;
use std::collections::VecDeque;
use crate::parse::{parse, SBS1Message};

mod parse;

/// Maximum number of SBS-1 messages to collect before sending to the DataSet web service.
const BATCH_SIZE: usize = 100;

/// Authentication token for the DataSet web service. Replace with your actual token.
const TOKEN: &str = "REDACTED";

/// Hostname or IP address of the DUMP1090 service. Replace with your actual host.
const DUMP1090_HOST: &str = "utilities.33901.cloud";

/// Port number of the DUMP1090 service.
const DUMP1090_PORT: u32 = 30003;

/// The main entry point of the application.
///
/// This function connects to the DUMP1090 TCP service, reads messages, parses them,
/// and sends them in batches to the DataSet web service.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connecting to a TCP stream
    let stream = TcpStream::connect(format!("{}:{}", DUMP1090_HOST, DUMP1090_PORT.to_string()))?;
    let reader = BufReader::new(stream);

    // Initialize a double-ended queue with a specified capacity.
    let mut messages: VecDeque<SBS1Message> = VecDeque::with_capacity(BATCH_SIZE);
    
    // Iterate over each line from the TCP stream.
    for line in reader.lines() {
        if let Ok(msg) = line {
            // Parse the line into an SBS1Message.
            if let Some(parsed) = parse(&msg) {
                messages.push_back(parsed);
                
                // Send the collected messages when the queue reaches the batch size.
                if messages.len() >= BATCH_SIZE {
                    send_to_service(messages.drain(..).collect()).await?;
                }
            }
        }
    }
    
    // Send any remaining messages if there are any left in the queue.
    if !messages.is_empty() {
        send_to_service(messages.drain(..).collect()).await?;
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
///
/// # Returns
///
/// A Result indicating the success or failure of the operation.
async fn send_to_service(messages: Vec<SBS1Message>) -> Result<(), reqwest::Error> {
    // Construct the event payload for each message.
    let events: Vec<Value> = messages.into_iter().map(|message| {
        json!({
            "parser": "adsb",
            "ts": message.timestamp,
            "sev": 3,
            "attrs": {"message": message, "parser": "adsb"}
        })
    }).collect();

    // Construct the final payload to be sent to the DataSet web service.
    let payload = json!({
        "session": Uuid::new_v4(),
        "sessionInfo": {},
        "events": events,
        "threads": []
    });

    // Print a pretty version of the JSON payload for debugging.
    let pretty_json = serde_json::to_string_pretty(&payload).unwrap();
    println!("{}", pretty_json);

    // Send the payload to the DataSet web service.
    let client = reqwest::Client::new();
    let res = client.post("https://app.scalyr.com/api/addEvents")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", TOKEN))
        .json(&payload)
        .send()
        .await?;

    // Log the response from the DataSet web service.
    println!("Response: {:?}", res.text().await?);

    Ok(())
}
