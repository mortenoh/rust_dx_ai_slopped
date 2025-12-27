//! # Random Generation Command
//!
//! Generate random numbers, strings, passwords, and make random choices.
//!
//! ## Examples
//! ```bash
//! dx rand int 1 100           # Random int between 1-100
//! dx rand string 32           # 32-char alphanumeric string
//! dx rand password 20         # 20-char password with symbols
//! dx rand choice a b c d      # Pick one randomly
//! dx rand dice 20             # Roll a d20
//! ```

use crate::cli::commands::rand::{RandArgs, RandCommand};
use anyhow::Result;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::Rng;

pub fn run(args: RandArgs) -> Result<()> {
    match args.command {
        RandCommand::Int { min, max, count } => cmd_int(min, max, count),
        RandCommand::Float { min, max, count } => cmd_float(min, max, count),
        RandCommand::String { length, count } => cmd_string(length, count),
        RandCommand::Hex { bytes, count } => cmd_hex(bytes, count),
        RandCommand::Password {
            length,
            no_symbols,
            count,
        } => cmd_password(length, no_symbols, count),
        RandCommand::Choice { items } => cmd_choice(&items),
        RandCommand::Shuffle { items } => cmd_shuffle(items),
        RandCommand::Coin { count } => cmd_coin(count),
        RandCommand::Dice { sides, count } => cmd_dice(sides, count),
    }
}

fn cmd_int(min: i64, max: i64, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        println!("{}", rng.random_range(min..=max));
    }
    Ok(())
}

fn cmd_float(min: f64, max: f64, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        let val: f64 = rng.random_range(min..max);
        println!("{:.6}", val);
    }
    Ok(())
}

fn cmd_string(length: usize, count: usize) -> Result<()> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();

    for _ in 0..count {
        let s: String = (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        println!("{}", s);
    }
    Ok(())
}

fn cmd_hex(bytes: usize, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        let data: Vec<u8> = (0..bytes).map(|_| rng.random::<u8>()).collect();
        println!("{}", hex::encode(data));
    }
    Ok(())
}

fn cmd_password(length: usize, no_symbols: bool, count: usize) -> Result<()> {
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const DIGITS: &[u8] = b"0123456789";
    const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

    let charset: Vec<u8> = if no_symbols {
        [ALPHA, DIGITS].concat()
    } else {
        [ALPHA, DIGITS, SYMBOLS].concat()
    };

    let mut rng = rand::rng();
    for _ in 0..count {
        let s: String = (0..length)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        println!("{}", s);
    }
    Ok(())
}

fn cmd_choice(items: &[String]) -> Result<()> {
    let mut rng = rand::rng();
    if let Some(choice) = items.choose(&mut rng) {
        println!("{}", choice);
    }
    Ok(())
}

fn cmd_shuffle(mut items: Vec<String>) -> Result<()> {
    let mut rng = rand::rng();
    items.shuffle(&mut rng);
    for item in items {
        println!("{}", item);
    }
    Ok(())
}

fn cmd_coin(count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        println!(
            "{}",
            if rng.random_bool(0.5) {
                "heads"
            } else {
                "tails"
            }
        );
    }
    Ok(())
}

fn cmd_dice(sides: u32, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        println!("{}", rng.random_range(1..=sides));
    }
    Ok(())
}
