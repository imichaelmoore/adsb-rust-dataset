//! This module provides functionality to parse and represent SBS1 messages.

use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::Serialize;

/// Represents a decoded SBS1 message with various aviation-related fields.
#[derive(Debug, Serialize, PartialEq)]
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
    on_ground: Option<bool>,
}

impl SBS1Message {
    /// Creates a new `SBS1Message` with the current timestamp and all other fields set to `None`.
    fn new() -> Self {
        let now = std::time::SystemTime::now();
        let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let timestamp_in_nanos =
            since_the_epoch.as_secs() * 1_000_000_000 + u64::from(since_the_epoch.subsec_nanos());
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
            on_ground: None,
        }
    }
}

// Getter methods for testing and external access
#[allow(dead_code)]
impl SBS1Message {
    #[must_use]
    pub fn message_type(&self) -> Option<&str> {
        self.message_type.as_deref()
    }

    #[must_use]
    pub fn transmission_type(&self) -> Option<i32> {
        self.transmission_type
    }

    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        self.session_id.as_deref()
    }

    #[must_use]
    pub fn aircraft_id(&self) -> Option<&str> {
        self.aircraft_id.as_deref()
    }

    #[must_use]
    pub fn icao24(&self) -> Option<&str> {
        self.icao24.as_deref()
    }

    #[must_use]
    pub fn flight_id(&self) -> Option<&str> {
        self.flight_id.as_deref()
    }

    #[must_use]
    pub fn generated_date(&self) -> Option<NaiveDateTime> {
        self.generated_date
    }

    #[must_use]
    pub fn logged_date(&self) -> Option<NaiveDateTime> {
        self.logged_date
    }

    #[must_use]
    pub fn callsign(&self) -> Option<&str> {
        self.callsign.as_deref()
    }

    #[must_use]
    pub fn altitude(&self) -> Option<i32> {
        self.altitude
    }

    #[must_use]
    pub fn ground_speed(&self) -> Option<f32> {
        self.ground_speed
    }

    #[must_use]
    pub fn track(&self) -> Option<f32> {
        self.track
    }

    #[must_use]
    pub fn lat(&self) -> Option<f32> {
        self.lat
    }

    #[must_use]
    pub fn lon(&self) -> Option<f32> {
        self.lon
    }

    #[must_use]
    pub fn vertical_rate(&self) -> Option<i32> {
        self.vertical_rate
    }

    #[must_use]
    pub fn squawk(&self) -> Option<i32> {
        self.squawk
    }

    #[must_use]
    pub fn alert(&self) -> Option<bool> {
        self.alert
    }

    #[must_use]
    pub fn emergency(&self) -> Option<bool> {
        self.emergency
    }

    #[must_use]
    pub fn spi(&self) -> Option<bool> {
        self.spi
    }

    #[must_use]
    pub fn on_ground(&self) -> Option<bool> {
        self.on_ground
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

    match parts.first() {
        Some(&"MSG") => {
            sbs1.message_type = Some("MSG".to_string());
            sbs1.transmission_type = parse_int(parts.get(1));
            sbs1.session_id = parse_string(parts.get(2));
            sbs1.aircraft_id = parse_string(parts.get(3));
            sbs1.icao24 = parse_string(parts.get(4));
            sbs1.flight_id = parse_string(parts.get(5));
            sbs1.generated_date = parse_date_time(parts.get(6), parts.get(7));
            sbs1.logged_date = parse_date_time(parts.get(8), parts.get(9));
            sbs1.callsign = if parts.len() > 10 && !parts[10].is_empty() {
                Some(String::from(parts[10].trim()))
            } else {
                None
            };
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
        }
        _ => None,
    }
}

/// Converts an `Option<&&str>` into an `Option<String>`.
fn parse_string(opt: Option<&&str>) -> Option<String> {
    opt.map(|&s| s.to_string())
}

/// Parses a string representation of a boolean (by integer) into an `Option<bool>`.
fn parse_bool(opt: Option<&&str>) -> Option<bool> {
    opt.and_then(|s| i32::from_str(s).ok().map(|num| num != 0))
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
        let combined = format!("{date} {time}");
        NaiveDateTime::parse_from_str(&combined, "%Y/%m/%d %H:%M:%S").ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, NaiveDate, Timelike};

    // Real-world SBS-1 message format:
    // MSG,<trans_type>,<session>,<aircraft>,<icao24>,<flight>,<gen_date>,<gen_time>,<log_date>,<log_time>,
    // <callsign>,<altitude>,<ground_speed>,<track>,<lat>,<lon>,<vert_rate>,<squawk>,<alert>,<emergency>,<spi>,<on_ground>

    /// Helper to create a full MSG,3 (airborne position) message
    fn sample_msg3_position() -> &'static str {
        "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,UAL123,35000,450.5,180.0,40.7128,-74.0060,0,1200,0,0,0,0"
    }

    /// Helper to create a MSG,1 (identification) message
    fn sample_msg1_identification() -> &'static str {
        "MSG,1,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,DAL456,,,,,,,,,,,"
    }

    /// Helper to create a MSG,4 (airborne velocity) message
    fn sample_msg4_velocity() -> &'static str {
        "MSG,4,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,35000,520.0,270.5,,,1500,,,,,0"
    }

    // ==================== parse() function tests ====================

    #[test]
    fn test_parse_valid_msg3_with_full_position() {
        let msg = sample_msg3_position();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        assert_eq!(parsed.message_type(), Some("MSG"));
        assert_eq!(parsed.transmission_type(), Some(3));
        assert_eq!(parsed.session_id(), Some("1"));
        assert_eq!(parsed.aircraft_id(), Some("1"));
        assert_eq!(parsed.icao24(), Some("A1B2C3"));
        assert_eq!(parsed.flight_id(), Some("1"));
        assert_eq!(parsed.callsign(), Some("UAL123"));
        assert_eq!(parsed.altitude(), Some(35000));
        assert_eq!(parsed.ground_speed(), Some(450.5));
        assert_eq!(parsed.track(), Some(180.0));
        assert_eq!(parsed.lat(), Some(40.7128));
        assert_eq!(parsed.lon(), Some(-74.006)); // Note: f32 precision
        assert_eq!(parsed.vertical_rate(), Some(0));
        assert_eq!(parsed.squawk(), Some(1200));
        assert_eq!(parsed.alert(), Some(false));
        assert_eq!(parsed.emergency(), Some(false));
        assert_eq!(parsed.spi(), Some(false));
        assert_eq!(parsed.on_ground(), Some(false));
    }

    #[test]
    fn test_parse_msg1_identification_message() {
        let msg = sample_msg1_identification();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        assert_eq!(parsed.message_type(), Some("MSG"));
        assert_eq!(parsed.transmission_type(), Some(1));
        assert_eq!(parsed.icao24(), Some("A1B2C3"));
        assert_eq!(parsed.callsign(), Some("DAL456"));
        // MSG,1 typically only has identification, other fields empty
        assert_eq!(parsed.altitude(), None);
        assert_eq!(parsed.ground_speed(), None);
        assert_eq!(parsed.lat(), None);
        assert_eq!(parsed.lon(), None);
    }

    #[test]
    fn test_parse_msg4_velocity_message() {
        let msg = sample_msg4_velocity();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        assert_eq!(parsed.message_type(), Some("MSG"));
        assert_eq!(parsed.transmission_type(), Some(4));
        assert_eq!(parsed.altitude(), Some(35000));
        assert_eq!(parsed.ground_speed(), Some(520.0));
        assert_eq!(parsed.track(), Some(270.5));
        assert_eq!(parsed.vertical_rate(), Some(1500));
        assert_eq!(parsed.callsign(), None); // MSG,4 typically no callsign
        assert_eq!(parsed.on_ground(), Some(false));
    }

    #[test]
    fn test_parse_rejects_non_msg_types() {
        // SBS-1 has other message types like SEL, ID, AIR, STA, CLK
        assert!(parse("SEL,1,1,1,A1B2C3,1,,,,,,,,,,,,,,,").is_none());
        assert!(parse("AIR,1,1,1,A1B2C3,1,,,,,,,,,,,,,,,").is_none());
        assert!(parse("STA,1,1,1,A1B2C3,1,,,,,,,,,,,,,,,").is_none());
        assert!(parse("CLK,1,1,1,A1B2C3,1,,,,,,,,,,,,,,,").is_none());
    }

    #[test]
    fn test_parse_empty_string() {
        assert!(parse("").is_none());
    }

    #[test]
    fn test_parse_whitespace_only() {
        assert!(parse("   ").is_none());
        assert!(parse("\t\n").is_none());
    }

    #[test]
    fn test_parse_handles_leading_trailing_whitespace() {
        let msg =
            "  MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,TEST,1000,,,,,,,,,, \n";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.message_type(), Some("MSG"));
        assert_eq!(parsed.callsign(), Some("TEST"));
        assert_eq!(parsed.altitude(), Some(1000));
    }

    #[test]
    fn test_parse_empty_callsign() {
        // Callsign field is empty (common for MSG,3 without identification)
        let msg =
            "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,35000,,,40.0,-74.0,,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.callsign(), None);
        assert_eq!(parsed.altitude(), Some(35000));
    }

    #[test]
    fn test_parse_callsign_with_spaces() {
        // Callsign may have trailing spaces
        let msg = "MSG,1,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,UAL123  ,,,,,,,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.callsign(), Some("UAL123")); // Should be trimmed
    }

    #[test]
    fn test_parse_negative_values() {
        // Negative latitude/longitude (southern/western hemispheres)
        // Negative vertical rate (descending)
        // Fields: callsign(10), alt(11), gspd(12), track(13), lat(14), lon(15), vrate(16), squawk(17)...
        let msg = "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,-500,,,-33.8688,151.2093,-2000,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.altitude(), Some(-500)); // Below sea level / ground pressure
        assert_eq!(parsed.lat(), Some(-33.8688)); // Sydney, Australia
        assert_eq!(parsed.lon(), Some(151.2093));
        assert_eq!(parsed.vertical_rate(), Some(-2000)); // Descending
    }

    #[test]
    fn test_parse_boolean_fields() {
        // Test with alert, emergency, spi, on_ground all set to 1 (true)
        let msg = "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,0,,,,,,7700,1,1,1,1";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.squawk(), Some(7700)); // Emergency squawk
        assert_eq!(parsed.alert(), Some(true));
        assert_eq!(parsed.emergency(), Some(true));
        assert_eq!(parsed.spi(), Some(true));
        assert_eq!(parsed.on_ground(), Some(true));
    }

    #[test]
    fn test_parse_date_time_fields() {
        let msg = sample_msg3_position();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        let expected_dt = NaiveDate::from_ymd_opt(2024, 1, 15)
            .unwrap()
            .and_hms_opt(12, 30, 45)
            .unwrap();

        assert_eq!(parsed.generated_date(), Some(expected_dt));
        assert_eq!(parsed.logged_date(), Some(expected_dt));
    }

    #[test]
    fn test_parse_invalid_date_format() {
        // Date in wrong format
        let msg = "MSG,3,1,1,A1B2C3,1,15-01-2024,12:30:45,15-01-2024,12:30:45,,35000,,,,,,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.generated_date(), None);
        assert_eq!(parsed.logged_date(), None);
    }

    #[test]
    fn test_parse_minimal_msg() {
        // MSG with minimum required fields, rest empty
        let msg = "MSG,3,,,A1B2C3,,,,,,,,,,,,,,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.message_type(), Some("MSG"));
        assert_eq!(parsed.transmission_type(), Some(3));
        assert_eq!(parsed.icao24(), Some("A1B2C3"));
    }

    #[test]
    fn test_parse_all_transmission_types() {
        // SBS-1 defines transmission types 1-8
        for trans_type in 1..=8 {
            let msg = format!(
                "MSG,{trans_type},1,1,ABCDEF,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,,,,,,,,,,,"
            );
            let result = parse(&msg);

            assert!(
                result.is_some(),
                "Failed for transmission type {trans_type}"
            );
            assert_eq!(result.unwrap().transmission_type(), Some(trans_type));
        }
    }

    #[test]
    fn test_parse_timestamp_is_set() {
        let msg = sample_msg3_position();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        // Timestamp should be a valid nanosecond value (non-empty numeric string)
        assert!(!parsed.timestamp.is_empty());
        assert!(parsed.timestamp.parse::<u64>().is_ok());
    }

    #[test]
    fn test_parse_icao24_hex_codes() {
        // ICAO24 addresses are 24-bit hex codes
        let test_cases = ["A1B2C3", "FFFFFF", "000000", "ABC123", "a1b2c3"];

        for icao in test_cases {
            let msg =
                format!("MSG,3,1,1,{icao},1,2024/01/15,12:30:45,2024/01/15,12:30:45,,,,,,,,,,,,");
            let result = parse(&msg);

            assert!(result.is_some());
            assert_eq!(result.unwrap().icao24(), Some(icao));
        }
    }

    #[test]
    fn test_parse_float_precision() {
        // Test that float values are parsed correctly
        let msg = "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,12345,123.456,359.9,51.5074,-0.1278,,,,,";
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        // Check float values with appropriate tolerance
        assert!((parsed.ground_speed().unwrap() - 123.456).abs() < 0.001);
        assert!((parsed.track().unwrap() - 359.9).abs() < 0.1);
        assert!((parsed.lat().unwrap() - 51.5074).abs() < 0.0001);
        assert!((parsed.lon().unwrap() - (-0.1278)).abs() < 0.0001);
    }

    #[test]
    fn test_parse_common_squawk_codes() {
        // Test common squawk codes
        // Squawk is at position 17: callsign(10), alt(11), gspd(12), track(13), lat(14), lon(15), vrate(16), squawk(17)
        let squawk_codes = [
            (1200, "VFR"),
            (7500, "Hijack"),
            (7600, "Radio failure"),
            (7700, "Emergency"),
        ];

        for (code, _meaning) in squawk_codes {
            let msg = format!(
                "MSG,3,1,1,A1B2C3,1,2024/01/15,12:30:45,2024/01/15,12:30:45,,,,,,,,{code},,,,"
            );
            let result = parse(&msg);

            assert!(result.is_some());
            assert_eq!(result.unwrap().squawk(), Some(code));
        }
    }

    // ==================== Helper function tests ====================

    #[test]
    fn test_parse_string_with_value() {
        let s = "test";
        assert_eq!(parse_string(Some(&s)), Some("test".to_string()));
    }

    #[test]
    fn test_parse_string_with_none() {
        assert_eq!(parse_string(None), None);
    }

    #[test]
    fn test_parse_string_with_empty() {
        let s = "";
        assert_eq!(parse_string(Some(&s)), Some(String::new()));
    }

    #[test]
    fn test_parse_int_valid() {
        let s = "42";
        assert_eq!(parse_int(Some(&s)), Some(42));
    }

    #[test]
    fn test_parse_int_negative() {
        let s = "-100";
        assert_eq!(parse_int(Some(&s)), Some(-100));
    }

    #[test]
    fn test_parse_int_invalid() {
        let s = "not_a_number";
        assert_eq!(parse_int(Some(&s)), None);
    }

    #[test]
    fn test_parse_int_empty() {
        let s = "";
        assert_eq!(parse_int(Some(&s)), None);
    }

    #[test]
    fn test_parse_int_none() {
        assert_eq!(parse_int(None), None);
    }

    #[test]
    fn test_parse_float_valid() {
        let s = "3.14159";
        let result = parse_float(Some(&s));
        assert!(result.is_some());
        assert!((result.unwrap() - 3.14159).abs() < 0.00001);
    }

    #[test]
    fn test_parse_float_negative() {
        let s = "-273.15";
        let result = parse_float(Some(&s));
        assert!(result.is_some());
        assert!((result.unwrap() - (-273.15)).abs() < 0.01);
    }

    #[test]
    fn test_parse_float_integer() {
        let s = "100";
        assert_eq!(parse_float(Some(&s)), Some(100.0));
    }

    #[test]
    fn test_parse_float_invalid() {
        let s = "not_a_float";
        assert_eq!(parse_float(Some(&s)), None);
    }

    #[test]
    fn test_parse_float_none() {
        assert_eq!(parse_float(None), None);
    }

    #[test]
    fn test_parse_bool_true() {
        let s = "1";
        assert_eq!(parse_bool(Some(&s)), Some(true));
    }

    #[test]
    fn test_parse_bool_false() {
        let s = "0";
        assert_eq!(parse_bool(Some(&s)), Some(false));
    }

    #[test]
    fn test_parse_bool_nonzero_is_true() {
        // Any non-zero value should be true
        let s = "5";
        assert_eq!(parse_bool(Some(&s)), Some(true));

        let s = "-1";
        assert_eq!(parse_bool(Some(&s)), Some(true));
    }

    #[test]
    fn test_parse_bool_invalid() {
        let s = "true"; // Not an integer
        assert_eq!(parse_bool(Some(&s)), None);
    }

    #[test]
    fn test_parse_bool_none() {
        assert_eq!(parse_bool(None), None);
    }

    #[test]
    fn test_parse_date_time_valid() {
        let date = "2024/01/15";
        let time = "12:30:45";
        let result = parse_date_time(Some(&date), Some(&time));

        assert!(result.is_some());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
        assert_eq!(dt.second(), 45);
    }

    #[test]
    fn test_parse_date_time_missing_date() {
        let time = "12:30:45";
        assert_eq!(parse_date_time(None, Some(&time)), None);
    }

    #[test]
    fn test_parse_date_time_missing_time() {
        let date = "2024/01/15";
        assert_eq!(parse_date_time(Some(&date), None), None);
    }

    #[test]
    fn test_parse_date_time_both_missing() {
        assert_eq!(parse_date_time(None, None), None);
    }

    #[test]
    fn test_parse_date_time_invalid_format() {
        let date = "2024-01-15"; // Wrong separator
        let time = "12:30:45";
        assert_eq!(parse_date_time(Some(&date), Some(&time)), None);
    }

    #[test]
    fn test_parse_date_time_invalid_time() {
        let date = "2024/01/15";
        let time = "25:00:00"; // Invalid hour
        assert_eq!(parse_date_time(Some(&date), Some(&time)), None);
    }

    // ==================== Serialization tests ====================

    #[test]
    fn test_sbs1_message_serializes_to_json() {
        let msg = sample_msg3_position();
        let result = parse(msg);

        assert!(result.is_some());
        let parsed = result.unwrap();

        // Should serialize without panicking
        let json = serde_json::to_string(&parsed);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("\"message_type\":\"MSG\""));
        assert!(json_str.contains("\"icao24\":\"A1B2C3\""));
        assert!(json_str.contains("\"callsign\":\"UAL123\""));
    }
}
