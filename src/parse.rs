//! This module provides functionality to parse and represent SBS1 messages.

extern crate chrono;
extern crate serde_derive;

use chrono::NaiveDateTime;
use std::str::FromStr;
use serde_derive::Serialize;

/// Represents a decoded SBS1 message with various aviation-related fields.
#[derive(Debug, Serialize)]
pub struct SBS1Message {
    pub timestamp: String, // Nanoseconds since the UNIX epoch
    message_type: Option<String>,
    transmission_type: Option<i32>,
    session_id: Option<String>,
    aircraft_id: Option<String>,
    icao24: Option<String>,
    flight_id: Option<String>,
    generated_date: Option<NaiveDateTime>,
    logged_date: Option<NaiveDateTime>,
    callsign: Option<String>,
    altitude: Option<i32>,
    ground_speed: Option<f32>,
    track: Option<f32>,
    lat: Option<f32>,
    lon: Option<f32>,
    vertical_rate: Option<i32>,
    squawk: Option<i32>,
    alert: Option<bool>,
    emergency: Option<bool>,
    spi: Option<bool>,
    on_ground: Option<bool>
}

impl SBS1Message {
    /// Creates a new `SBS1Message` with the current timestamp and all other fields set to `None`.
    fn new() -> Self {
        let now = std::time::SystemTime::now();
        let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let timestamp_in_nanos = since_the_epoch.as_secs() * 1_000_000_000 + since_the_epoch.subsec_nanos() as u64;
        let timestamp_in_nanos_as_string = timestamp_in_nanos.to_string();

        SBS1Message {
            timestamp: timestamp_in_nanos_as_string,
            // All other fields are initialized to None
            message_type: None,
            transmission_type: None,
            session_id: None,
            aircraft_id: None,
            icao24: None,
            flight_id: None,
            generated_date: None,
            logged_date: None,
            callsign: None,
            altitude: None,
            ground_speed: None,
            track: None,
            lat: None,
            lon: None,
            vertical_rate: None,
            squawk: None,
            alert: None,
            emergency: None,
            spi: None,
            on_ground: None
        }
    }
}

/// Parses an SBS1 message string and returns an `Option<SBS1Message>`.
///
/// # Arguments
///
/// * `msg` - A string slice containing an SBS1 message.
///
/// # Returns
///
/// An `Option` that contains a parsed `SBS1Message` if successful or `None` otherwise.
pub fn parse(msg: &str) -> Option<SBS1Message> {
    let mut sbs1 = SBS1Message::new();
    let parts: Vec<&str> = msg.trim().split(',').collect();

    match parts.get(0) {
        Some(&"MSG") => {
            sbs1.message_type = Some("MSG".to_string());
            sbs1.transmission_type = parse_int(parts.get(1));
            sbs1.session_id = parse_string(parts.get(2));
            sbs1.aircraft_id = parse_string(parts.get(3));
            sbs1.icao24 = parse_string(parts.get(4));
            sbs1.flight_id = parse_string(parts.get(5));
            sbs1.generated_date = parse_date_time(parts.get(6), parts.get(7));
            sbs1.logged_date = parse_date_time(parts.get(8), parts.get(9));
            sbs1.callsign = parse_string(parts.get(10));
            sbs1.altitude = parse_int(parts.get(11));
            sbs1.ground_speed = parse_float(parts.get(12));
            sbs1.track = parse_float(parts.get(13));
            sbs1.lat = parse_float(parts.get(14));
            sbs1.lon = parse_float(parts.get(15));
            sbs1.vertical_rate = parse_int(parts.get(16));
            sbs1.squawk = parse_int(parts.get(17));
            sbs1.alert = parse_bool(parts.get(18));
            sbs1.emergency = parse_bool(parts.get(19));
            sbs1.spi = parse_bool(parts.get(20));
            sbs1.on_ground = parse_bool(parts.get(21));
            Some(sbs1)
        },
        _ => None
    }
}

/// Converts an `Option<&&str>` into an `Option<String>`.
fn parse_string(opt: Option<&&str>) -> Option<String> {
    opt.map(|&s| s.to_string())
}

/// Parses a string representation of a boolean (by integer) into an `Option<bool>`.
fn parse_bool(opt: Option<&&str>) -> Option<bool> {
    match opt {
        Some(s) => i32::from_str(s).ok().map(|num| num != 0),
        None => None,
    }
}

/// Converts an `Option<&&str>` into an `Option<i32>`.
fn parse_int(opt: Option<&&str>) -> Option<i32> {
    opt.and_then(|&s| i32::from_str(s).ok())
}

/// Converts an `Option<&&str>` into an `Option<f32>`.
fn parse_float(opt: Option<&&str>) -> Option<f32> {
    opt.and_then(|&s| f32::from_str(s).ok())
}

/// Combines date and time string representations into a single `NaiveDateTime`.
///
/// # Arguments
///
/// * `opt_date` - An optional string slice representing the date.
/// * `opt_time` - An optional string slice representing the time.
///
/// # Returns
///
/// An `Option` that contains a `NaiveDateTime` if successful or `None` otherwise.
fn parse_date_time(opt_date: Option<&&str>, opt_time: Option<&&str>) -> Option<NaiveDateTime> {
    if let (Some(&date), Some(&time)) = (opt_date, opt_time) {
        let combined = format!("{} {}", date, time);
        NaiveDateTime::parse_from_str(&combined, "%Y/%m/%d %H:%M:%S").ok()
    } else {
        None
    }
}
