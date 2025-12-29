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
use dx_datagen::{generators, password};
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
    let mut rng = rand::rng();
    for _ in 0..count {
        println!("{}", generators::alphanumeric(&mut rng, length));
    }
    Ok(())
}

fn cmd_hex(bytes: usize, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        println!("{}", generators::hex_bytes(&mut rng, bytes));
    }
    Ok(())
}

fn cmd_password(length: usize, no_symbols: bool, count: usize) -> Result<()> {
    let mut rng = rand::rng();
    for _ in 0..count {
        println!("{}", password::password(&mut rng, length, !no_symbols));
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
