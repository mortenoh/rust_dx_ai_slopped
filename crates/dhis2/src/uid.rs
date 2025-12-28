//! DHIS2 UID generation and validation.
//!
//! DHIS2 UIDs are 11-character alphanumeric identifiers:
//! - First character: a-zA-Z (letter)
//! - Remaining 10: a-zA-Z0-9 (alphanumeric)

use anyhow::Result;
use colored::Colorize;
use rand::Rng;

/// Valid characters for the first position (must be a letter).
const FIRST_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Valid characters for positions 2-11 (alphanumeric).
const OTHER_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Generate a single DHIS2 UID.
pub fn generate_uid() -> String {
    let mut rng = rand::rng();
    let mut uid = String::with_capacity(11);

    // First character must be a letter
    uid.push(FIRST_CHARS[rng.random_range(0..FIRST_CHARS.len())] as char);

    // Remaining 10 characters can be alphanumeric
    for _ in 0..10 {
        uid.push(OTHER_CHARS[rng.random_range(0..OTHER_CHARS.len())] as char);
    }

    uid
}

/// Validate a UID string.
pub fn validate_uid(uid: &str) -> Result<(), String> {
    // Check length
    if uid.len() != 11 {
        return Err(format!("Invalid length: {} (expected 11)", uid.len()));
    }

    // Check first character is a letter
    let first = uid.chars().next().unwrap();
    if !first.is_ascii_alphabetic() {
        return Err(format!(
            "First character '{}' must be a letter (a-zA-Z)",
            first
        ));
    }

    // Check remaining characters are alphanumeric
    for (i, c) in uid.chars().enumerate().skip(1) {
        if !c.is_ascii_alphanumeric() {
            return Err(format!(
                "Character '{}' at position {} is not alphanumeric",
                c,
                i + 1
            ));
        }
    }

    Ok(())
}

/// Run the UID subcommand.
pub fn run(count: usize, validate: Option<String>, json: bool, plain: bool) -> Result<()> {
    // Validation mode
    if let Some(uid) = validate {
        match validate_uid(&uid) {
            Ok(()) => {
                if json {
                    println!(r#"{{"uid": "{}", "valid": true}}"#, uid);
                } else {
                    println!("{} '{}' is a valid DHIS2 UID", "✓".green(), uid);
                }
            }
            Err(e) => {
                if json {
                    println!(r#"{{"uid": "{}", "valid": false, "error": "{}"}}"#, uid, e);
                } else {
                    eprintln!("{} '{}' is NOT a valid DHIS2 UID: {}", "✗".red(), uid, e);
                }
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Generation mode
    let uids: Vec<String> = (0..count).map(|_| generate_uid()).collect();

    if json {
        if count == 1 {
            println!(r#"{{"uid": "{}"}}"#, uids[0]);
        } else {
            let json_array: Vec<String> = uids.iter().map(|u| format!(r#""{}""#, u)).collect();
            println!("[{}]", json_array.join(", "));
        }
    } else if plain {
        for uid in &uids {
            println!("{}", uid);
        }
    } else {
        println!("Generated {} DHIS2 UID(s):", count);
        println!();

        for (i, uid) in uids.iter().enumerate() {
            println!("  {:3}. {}", i + 1, uid.cyan());
        }

        if count > 1 {
            println!();
            println!("Copy-paste ready (comma-separated):");
            println!("  {}", uids.join(", "));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uid_length() {
        let uid = generate_uid();
        assert_eq!(uid.len(), 11);
    }

    #[test]
    fn test_uid_first_char_is_letter() {
        for _ in 0..100 {
            let uid = generate_uid();
            assert!(uid.chars().next().unwrap().is_ascii_alphabetic());
        }
    }

    #[test]
    fn test_uid_all_alphanumeric() {
        for _ in 0..100 {
            let uid = generate_uid();
            assert!(uid.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn test_validate_valid_uid() {
        assert!(validate_uid("abc12345678").is_ok());
        assert!(validate_uid("A1234567890").is_ok());
        assert!(validate_uid("xyzABC12345").is_ok());
    }

    #[test]
    fn test_validate_invalid_uid() {
        assert!(validate_uid("1bc12345678").is_err()); // Starts with number
        assert!(validate_uid("abc1234567").is_err()); // Too short
        assert!(validate_uid("abc123456789").is_err()); // Too long
        assert!(validate_uid("abc1234_678").is_err()); // Contains underscore
    }
}
