//! # Fun Terminal UI Command
//!
//! Fun terminal effects like fake progress bars, hacker mode, countdown timers, and spinners.

use crate::cli::commands::fun::{FunArgs, FunCommand};
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

pub fn run(args: FunArgs) -> Result<()> {
    match args.command {
        FunCommand::Progress { duration, style } => cmd_progress(duration, &style),
        FunCommand::Hacker {
            duration,
            intensity,
        } => cmd_hacker(duration, intensity),
        FunCommand::Countdown {
            seconds,
            message,
            simple,
        } => cmd_countdown(seconds, message, simple),
        FunCommand::Spinners { duration, name } => cmd_spinners(duration, name),
    }
}

/// Funny loading messages
const LOADING_MESSAGES: &[&str] = &[
    "Reticulating splines...",
    "Calibrating flux capacitor...",
    "Downloading more RAM...",
    "Reversing the polarity...",
    "Compiling the compiler...",
    "Warming up the quantum tunneler...",
    "Untangling the spaghetti code...",
    "Feeding the hamsters...",
    "Charging the lasers...",
    "Deploying the carrier pigeons...",
    "Consulting the oracle...",
    "Aligning the bits...",
    "Summoning the daemon...",
    "Negotiating with the firewall...",
    "Bribing the CPU...",
    "Defragmenting the timeline...",
    "Generating witty loading messages...",
    "Converting coffee to code...",
    "Adjusting the reality matrix...",
    "Polishing the pixels...",
    "Herding the electrons...",
    "Optimizing the butterfly effect...",
    "Counting backwards from infinity...",
    "Dividing by zero...",
    "Proving P = NP...",
];

/// Fake progress bar with funny messages
fn cmd_progress(duration: u64, style: &str) -> Result<()> {
    let mut rng = rand::rng();
    let total_ms = duration * 1000;
    let start = Instant::now();

    match style {
        "spinner" => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            spinner.enable_steady_tick(Duration::from_millis(80));

            while start.elapsed().as_millis() < total_ms as u128 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];
                spinner.set_message(msg.to_string());
                let delay = rng.random_range(500..2000);
                thread::sleep(Duration::from_millis(delay));
            }

            spinner.finish_with_message("Done!".green().to_string());
        }
        "bar" => {
            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{bar:40.cyan/blue} {pos}% {msg}")
                    .unwrap()
                    .progress_chars("█▓░"),
            );

            let mut progress = 0u64;
            while progress < 100 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];
                pb.set_message(msg.to_string());

                // Random progress increment (with occasional stalls)
                let increment = if rng.random_bool(0.1) {
                    pb.set_message("Almost there...".yellow().to_string());
                    thread::sleep(Duration::from_millis(500));
                    1
                } else {
                    rng.random_range(1..5)
                };

                progress = (progress + increment).min(100);
                pb.set_position(progress);

                let delay = total_ms / 100 * increment;
                thread::sleep(Duration::from_millis(delay.max(50)));
            }

            pb.finish_with_message("Complete!".green().to_string());
        }
        _ => {
            // "both" style - spinner + progress bar
            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:30.cyan/blue}] {pos}% {msg}")
                    .unwrap()
                    .progress_chars("█▓░"),
            );
            pb.enable_steady_tick(Duration::from_millis(80));

            let mut progress = 0u64;
            while progress < 100 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];
                pb.set_message(msg.to_string());

                // Random progress with stalls
                let increment = if rng.random_bool(0.15) {
                    pb.set_message("Just a moment...".yellow().to_string());
                    thread::sleep(Duration::from_millis(800));
                    1
                } else {
                    rng.random_range(1..4)
                };

                progress = (progress + increment).min(100);
                pb.set_position(progress);

                let delay = total_ms / 100 * increment;
                thread::sleep(Duration::from_millis(delay.max(50)));
            }

            pb.finish_with_message("All done!".green().bold().to_string());
        }
    }

    Ok(())
}

/// Hacker mode output types
const HACKER_PREFIXES: &[(&str, &str)] = &[
    ("OK", "green"),
    ("INFO", "cyan"),
    ("WARN", "yellow"),
    ("ACCESS", "green"),
    ("TRACE", "blue"),
    ("SUCCESS", "green"),
    ("ERROR", "red"),
];

const HACKER_ACTIONS: &[&str] = &[
    "Bypassing firewall on {}",
    "Decrypting AES-256 key: {}",
    "Injecting payload into {}",
    "Scanning port {} for vulnerabilities",
    "Establishing tunnel to {}",
    "Cracking hash: {}",
    "Exfiltrating data from {}",
    "Rerouting through proxy {}",
    "Spoofing MAC address to {}",
    "Brute forcing credentials on {}",
    "Dumping memory at {}",
    "Intercepting packets from {}",
    "Escalating privileges on {}",
    "Installing backdoor at {}",
    "Masking IP as {}",
];

const HACKER_COMMANDS: &[&str] = &[
    "cat /etc/passwd",
    "cat /etc/shadow",
    "rm -rf /var/log/*",
    "nc -lvp 4444",
    "nmap -sS -sV target",
    "ssh root@{}",
    "wget http://{}/shell.sh",
    "chmod +x exploit.sh",
    "./exploit --target {}",
    "python3 payload.py",
    "curl -X POST http://{}/exfil",
    "base64 -d secret.txt",
    "openssl enc -d -aes-256-cbc",
    "john --wordlist=rockyou.txt hash.txt",
    "hashcat -m 0 -a 0 hashes.txt",
];

/// Fake hacker terminal output
fn cmd_hacker(duration: u64, intensity: u8) -> Result<()> {
    let mut rng = rand::rng();
    let start = Instant::now();
    let total_ms = duration * 1000;

    // Delay between outputs based on intensity
    let (min_delay, max_delay) = match intensity {
        1 => (200, 800),
        3 => (30, 150),
        _ => (50, 300), // default intensity 2
    };

    println!("{}", "[INITIALIZING SECURE CONNECTION...]".green().bold());
    thread::sleep(Duration::from_millis(500));

    while start.elapsed().as_millis() < total_ms as u128 {
        let output_type = rng.random_range(0..4);

        match output_type {
            0 => {
                // Prefixed message
                let (prefix, color) = HACKER_PREFIXES[rng.random_range(0..HACKER_PREFIXES.len())];
                let action = HACKER_ACTIONS[rng.random_range(0..HACKER_ACTIONS.len())];
                let value = generate_hacker_value(&mut rng);
                let formatted = action.replace("{}", &value);

                let bracket_prefix = format!("[{}]", prefix);
                let prefix_colored = match color {
                    "green" => bracket_prefix.green(),
                    "cyan" => bracket_prefix.cyan(),
                    "yellow" => bracket_prefix.yellow(),
                    "red" => bracket_prefix.red(),
                    "blue" => bracket_prefix.blue(),
                    _ => bracket_prefix.white(),
                };
                println!("{} {}", prefix_colored, formatted);
            }
            1 => {
                // Command with prompt
                let cmd = HACKER_COMMANDS[rng.random_range(0..HACKER_COMMANDS.len())];
                let value = generate_hacker_value(&mut rng);
                let formatted = cmd.replace("{}", &value);

                let prompt = if rng.random_bool(0.5) { "$" } else { ">" };
                print!("{} ", prompt.green());
                io::stdout().flush().ok();

                // Typing effect
                for c in formatted.chars() {
                    print!("{}", c);
                    io::stdout().flush().ok();
                    thread::sleep(Duration::from_millis(rng.random_range(10..40)));
                }
                println!();
            }
            2 => {
                // Hex dump
                print!("{}: ", format!("0x{:016x}", rng.random::<u64>()).blue());
                for _ in 0..8 {
                    print!("{:02x} ", rng.random::<u8>());
                }
                println!();
            }
            _ => {
                // Progress line
                let percent = rng.random_range(0..100);
                let filled = percent / 5;
                let bar: String =
                    "=".repeat(filled as usize) + ">" + &" ".repeat(20 - filled as usize);
                println!(
                    "Downloading: {} [{}] {}%",
                    generate_filename(&mut rng).cyan(),
                    bar.green(),
                    percent
                );
            }
        }

        let delay = rng.random_range(min_delay..max_delay);
        thread::sleep(Duration::from_millis(delay));
    }

    println!();
    println!("{}", "[CONNECTION TERMINATED]".red().bold());

    Ok(())
}

fn generate_hacker_value(rng: &mut impl Rng) -> String {
    match rng.random_range(0..4) {
        0 => format!(
            "{}.{}.{}.{}",
            rng.random_range(1..255),
            rng.random_range(0..255),
            rng.random_range(0..255),
            rng.random_range(1..255)
        ),
        1 => format!("{:016x}", rng.random::<u64>()),
        2 => format!("node-{}", rng.random_range(1..100)),
        _ => format!(":{}", rng.random_range(1..65535)),
    }
}

fn generate_filename(rng: &mut impl Rng) -> String {
    let names = [
        "users", "data", "backup", "secrets", "config", "passwd", "shadow",
    ];
    let exts = [".db", ".sql", ".tar.gz", ".txt", ".json", ".bin"];
    format!(
        "{}{}",
        names[rng.random_range(0..names.len())],
        exts[rng.random_range(0..exts.len())]
    )
}

/// Countdown timer with visual effects
fn cmd_countdown(seconds: u64, message: Option<String>, simple: bool) -> Result<()> {
    let pb = ProgressBar::new(seconds);

    if simple {
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {pos}s remaining")
                .unwrap(),
        );
    } else {
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:30}] {msg}")
                .unwrap()
                .progress_chars("█▓░"),
        );
    }

    pb.enable_steady_tick(Duration::from_millis(100));

    for remaining in (0..seconds).rev() {
        let mins = remaining / 60;
        let secs = remaining % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        // Color based on time remaining
        let colored_time = if remaining <= 5 {
            time_str.red().bold().to_string()
        } else if remaining <= 10 {
            time_str.yellow().to_string()
        } else {
            time_str.green().to_string()
        };

        pb.set_message(colored_time);
        pb.set_position(seconds - remaining);

        thread::sleep(Duration::from_secs(1));
    }

    pb.finish_and_clear();

    // Bell and completion message
    print!("\x07"); // Terminal bell
    io::stdout().flush().ok();

    let final_message = message.unwrap_or_else(|| "Time's up!".to_string());
    println!("\n{} {}", ">>>".green().bold(), final_message.bold());

    Ok(())
}

/// Spinner styles to showcase
const SPINNER_STYLES: &[(&str, &str, &[&str])] = &[
    (
        "dots",
        "Classic braille dots",
        &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
    ),
    (
        "dots2",
        "Braille dots variant",
        &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
    ),
    (
        "line",
        "Growing line",
        &["▁", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃"],
    ),
    ("circle", "Rotating quadrants", &["◐", "◓", "◑", "◒"]),
    (
        "arrow",
        "Rotating arrow",
        &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
    ),
    (
        "bounce",
        "Bouncing bar",
        &[
            "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
        ],
    ),
    ("arc", "Arc spinner", &["◜", "◠", "◝", "◞", "◡", "◟"]),
    (
        "moon",
        "Moon phases",
        &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
    ),
    (
        "clock",
        "Clock faces",
        &[
            "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
        ],
    ),
    ("earth", "Rotating earth", &["🌍", "🌎", "🌏"]),
    ("runner", "Running figure", &["🚶", "🏃"]),
    (
        "pong",
        "Pong ball",
        &[
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
        ],
    ),
];

/// Showcase spinner styles
fn cmd_spinners(duration: u64, name: Option<String>) -> Result<()> {
    if let Some(ref spinner_name) = name {
        // Show single spinner
        if let Some((name, desc, frames)) =
            SPINNER_STYLES.iter().find(|(n, _, _)| *n == spinner_name)
        {
            println!("{}: {}\n", name.cyan().bold(), desc);
            run_spinner(frames, duration * 1000)?;
        } else {
            println!("Unknown spinner: {}", spinner_name);
            println!("\nAvailable spinners:");
            for (name, _, _) in SPINNER_STYLES {
                println!("  {}", name);
            }
        }
    } else {
        // Showcase all spinners
        println!("{}\n", "Spinner Showcase".cyan().bold());

        for (name, desc, frames) in SPINNER_STYLES {
            print!("{:12} ", name.yellow());
            io::stdout().flush().ok();

            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner} {msg}")
                    .unwrap()
                    .tick_strings(frames),
            );
            spinner.enable_steady_tick(Duration::from_millis(80));
            spinner.set_message(desc.to_string());

            thread::sleep(Duration::from_secs(duration));

            spinner.finish_and_clear();
            println!("{} {}", frames[0], desc.dimmed());
        }

        println!("\n{}", "Use: dx fun spinners --name <name>".dimmed());
    }

    Ok(())
}

fn run_spinner(frames: &[&str], duration_ms: u64) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} Running...")
            .unwrap()
            .tick_strings(frames),
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    thread::sleep(Duration::from_millis(duration_ms));

    spinner.finish_with_message("Done!");
    Ok(())
}
