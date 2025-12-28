//! # Fun Terminal UI Command
//!
//! Fun terminal effects like fake progress bars, hacker mode, countdown timers, and spinners.

use crate::cli::commands::fun::{FunArgs, FunCommand};
use crate::utils::progress::{
    self, osc_progress, osc_progress_clear, BouncingBar, ProgressState, SPINNER_FRAMES,
};
use anyhow::Result;
use colored::Colorize;
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
        FunCommand::Work {
            duration,
            tasks,
            style,
            list_styles,
        } => cmd_work(duration, tasks, &style, list_styles),
        FunCommand::Fortune { animal, say, list } => cmd_fortune(animal, say, list),
        FunCommand::Bounce { duration, message } => cmd_bounce(duration, message),
        FunCommand::Clock {
            duration,
            twelve_hour,
            seconds,
        } => cmd_clock(duration, twelve_hour, seconds),
        FunCommand::Qr { text, invert } => cmd_qr(&text, invert),
        FunCommand::Life {
            duration,
            pattern,
            width,
            height,
        } => cmd_life(duration, &pattern, width, height),
        FunCommand::Matrix { duration, density } => cmd_matrix(duration, density),
        FunCommand::Banner { text } => cmd_banner(&text),
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

/// Manual progress bar that bypasses terminal detection
fn draw_progress(progress: u64, width: usize, msg: &str) {
    let filled = (progress as usize * width) / 100;
    let empty = width - filled;
    let bar = format!("{}{}", "█".repeat(filled).cyan(), "░".repeat(empty).blue());
    eprint!("\r\x1b[K[{}] {}% {}", bar, progress, msg);
    io::stderr().flush().ok();
}

/// Manual spinner that bypasses terminal detection
fn draw_spinner(frame: usize, msg: &str) {
    progress::draw_spinner(frame, msg);
}

/// Fake progress bar with funny messages
fn cmd_progress(duration: u64, style: &str) -> Result<()> {
    let mut rng = rand::rng();
    let total_ms = duration * 1000;
    let start = Instant::now();

    // Report indeterminate progress at start
    osc_progress(0, ProgressState::Indeterminate);

    match style {
        "spinner" => {
            let mut frame = 0usize;
            let mut elapsed_percent;
            while start.elapsed().as_millis() < total_ms as u128 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];

                // Animate spinner for this message
                let msg_duration = rng.random_range(500..2000);
                let msg_start = Instant::now();
                while msg_start.elapsed().as_millis() < msg_duration {
                    elapsed_percent = (start.elapsed().as_millis() * 100 / total_ms as u128) as u64;
                    osc_progress(elapsed_percent, ProgressState::Normal);
                    draw_spinner(frame, msg);
                    frame += 1;
                    thread::sleep(Duration::from_millis(80));
                }
            }
            osc_progress_clear(); // Clear terminal progress
            eprintln!("\r\x1b[K{} {}", "✓".green(), "Done!".green());
        }
        "bar" => {
            let mut progress = 0u64;
            while progress < 100 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];

                // Random progress increment (with occasional stalls)
                let (increment, stall_msg) = if rng.random_bool(0.1) {
                    thread::sleep(Duration::from_millis(500));
                    (1, Some("Almost there...".yellow().to_string()))
                } else {
                    (rng.random_range(1..5), None)
                };

                progress = (progress + increment).min(100);
                osc_progress(progress, ProgressState::Normal);
                let display_msg = stall_msg.as_deref().unwrap_or(msg);
                draw_progress(progress, 30, display_msg);

                let delay = total_ms / 100 * increment;
                thread::sleep(Duration::from_millis(delay.max(50)));
            }
            osc_progress_clear(); // Clear terminal progress
            eprintln!(
                "\r\x1b[K[{}] 100% {}",
                "█".repeat(30).cyan(),
                "Complete!".green()
            );
        }
        _ => {
            // "both" style - spinner + progress bar
            let mut progress = 0u64;
            let mut frame = 0usize;

            while progress < 100 {
                let msg = LOADING_MESSAGES[rng.random_range(0..LOADING_MESSAGES.len())];

                // Random progress with stalls
                let (increment, stall) = if rng.random_bool(0.15) {
                    (1, true)
                } else {
                    (rng.random_range(1..4), false)
                };

                progress = (progress + increment).min(100);
                osc_progress(progress, ProgressState::Normal);

                // Animate for this step
                let step_ms = if stall {
                    800
                } else {
                    (total_ms / 100 * increment).max(50)
                };
                let step_start = Instant::now();
                let display_msg = if stall {
                    "Just a moment...".yellow().to_string()
                } else {
                    msg.to_string()
                };

                while step_start.elapsed().as_millis() < step_ms as u128 {
                    let spinner = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()].green();
                    let filled = (progress as usize * 30) / 100;
                    let empty = 30 - filled;
                    let bar = format!("{}{}", "█".repeat(filled).cyan(), "░".repeat(empty).blue());
                    eprint!(
                        "\r\x1b[K{} [{}] {}% {}",
                        spinner, bar, progress, display_msg
                    );
                    io::stderr().flush().ok();
                    frame += 1;
                    thread::sleep(Duration::from_millis(80));
                }
            }
            osc_progress_clear(); // Clear terminal progress
            eprintln!(
                "\r\x1b[K{} [{}] 100% {}",
                "✓".green(),
                "█".repeat(30).cyan(),
                "All done!".green().bold()
            );
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
    for remaining in (1..=seconds).rev() {
        let mins = remaining / 60;
        let secs = remaining % 60;
        let time_str = format!("{:02}:{:02}", mins, secs);

        // Report progress to terminal (inverted - countdown goes from 0 to 100)
        let percent = ((seconds - remaining) * 100) / seconds;
        osc_progress(percent, ProgressState::Normal);

        // Color based on time remaining
        let colored_time = if remaining <= 5 {
            time_str.red().bold()
        } else if remaining <= 10 {
            time_str.yellow()
        } else {
            time_str.green()
        };

        if simple {
            eprint!("\r\x1b[K{} remaining", colored_time);
        } else {
            let elapsed = seconds - remaining;
            let filled = (elapsed as usize * 30) / seconds as usize;
            let empty = 30 - filled;
            let bar = format!("{}{}", "█".repeat(filled).cyan(), "░".repeat(empty).blue());
            let frame = (seconds - remaining) as usize;
            let spinner = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()].green();
            eprint!("\r\x1b[K{} [{}] {}", spinner, bar, colored_time);
        }
        io::stderr().flush().ok();

        thread::sleep(Duration::from_secs(1));
    }

    // Clear terminal progress and line
    osc_progress_clear();
    eprint!("\r\x1b[K");

    // Bell and completion message
    print!("\x07"); // Terminal bell
    io::stdout().flush().ok();

    let final_message = message.unwrap_or_else(|| "Time's up!".to_string());
    println!("{} {}", ">>>".green().bold(), final_message.bold());

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
            run_spinner_manual(frames, duration * 1000)?;
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

            // Animate spinner manually
            let start = Instant::now();
            let duration_ms = duration * 1000;
            let mut frame_idx = 0usize;
            while start.elapsed().as_millis() < duration_ms as u128 {
                print!(
                    "\r{:12} {} {}",
                    name.yellow(),
                    frames[frame_idx % frames.len()],
                    desc
                );
                io::stdout().flush().ok();
                frame_idx += 1;
                thread::sleep(Duration::from_millis(80));
            }

            println!("\r{:12} {} {}", name.yellow(), frames[0], desc.dimmed());
        }

        println!("\n{}", "Use: dx fun spinners --name <name>".dimmed());
    }

    Ok(())
}

fn run_spinner_manual(frames: &[&str], duration_ms: u64) -> Result<()> {
    let start = Instant::now();
    let mut frame_idx = 0usize;
    while start.elapsed().as_millis() < duration_ms as u128 {
        eprint!(
            "\r\x1b[K{} Running...",
            frames[frame_idx % frames.len()].green()
        );
        io::stderr().flush().ok();
        frame_idx += 1;
        thread::sleep(Duration::from_millis(80));
    }
    eprintln!("\r\x1b[K{} Done!", "✓".green());
    Ok(())
}

/// Fake developer tasks for the work command
const WORK_TASKS: &[&str] = &[
    "Compiling shaders",
    "Optimizing database indexes",
    "Syncing distributed nodes",
    "Downloading dependencies",
    "Running static analysis",
    "Generating bytecode",
    "Warming up caches",
    "Validating schemas",
    "Minifying assets",
    "Building Docker images",
    "Provisioning containers",
    "Encrypting secrets",
    "Resolving merge conflicts",
    "Updating lockfiles",
    "Calibrating neural networks",
    "Defragmenting memory pools",
    "Indexing search corpus",
    "Compressing artifacts",
    "Verifying checksums",
    "Transpiling modules",
    "Linking binaries",
    "Spawning worker threads",
    "Initializing state machine",
    "Loading configuration",
    "Establishing connections",
    "Parsing manifests",
    "Serializing objects",
    "Hydrating components",
    "Prefetching resources",
    "Analyzing dependencies",
];

/// Progress bar style definitions
const BAR_STYLES: &[(&str, &str, &[&str], &[&str])] = &[
    // (name, description, filled_chars, empty_chars)
    ("block", "Solid blocks", &["█"], &["░"]),
    ("gradient", "Gradient shading", &["█", "▓", "▒"], &["░"]),
    ("arrow", "Arrow style", &["=", "=", ">"], &["-"]),
    ("dots", "Braille dots", &["⣿"], &["⣀"]),
    ("emoji", "Fun emoji", &["🟩"], &["⬜"]),
    ("classic", "Hash marks", &["#"], &["-"]),
    ("circles", "Filled circles", &["●"], &["○"]),
    ("fade", "Fade effect", &["█", "▓", "▒", "░"], &[" "]),
];

/// Render a progress bar with the given style
fn render_bar(progress: u64, width: usize, style: &str, colored: bool) -> String {
    let (filled_chars, empty_chars) = BAR_STYLES
        .iter()
        .find(|(name, _, _, _)| *name == style)
        .map(|(_, _, f, e)| (*f, *e))
        .unwrap_or((&["█"], &["░"]));

    let filled_count = (progress as usize * width) / 100;
    let empty_count = width - filled_count;

    let filled_part: String = if filled_chars.len() == 1 {
        // Simple repeat
        filled_chars[0].repeat(filled_count)
    } else if style == "gradient" || style == "fade" {
        // Gradient: mostly solid, with trailing gradient
        if filled_count == 0 {
            String::new()
        } else if filled_count <= 2 {
            filled_chars[0].repeat(filled_count)
        } else {
            let gradient_len = filled_chars.len().min(filled_count);
            let solid_len = filled_count - gradient_len + 1;
            let mut s = filled_chars[0].repeat(solid_len);
            for char in filled_chars.iter().take(gradient_len).skip(1) {
                s.push_str(char);
            }
            s
        }
    } else if style == "arrow" {
        // Arrow: ====>
        if filled_count == 0 {
            String::new()
        } else if filled_count == 1 {
            ">".to_string()
        } else {
            format!("{}>", "=".repeat(filled_count - 1))
        }
    } else {
        filled_chars[0].repeat(filled_count)
    };

    let empty_part = empty_chars[0].repeat(empty_count);

    if colored {
        format!("{}{}", filled_part.green(), empty_part.dimmed())
    } else {
        format!("{}{}", filled_part, empty_part)
    }
}

/// Render a completed progress bar
fn render_bar_complete(width: usize, style: &str) -> String {
    let filled_char = BAR_STYLES
        .iter()
        .find(|(name, _, _, _)| *name == style)
        .map(|(_, _, f, _)| f[0])
        .unwrap_or("█");

    let bar = if style == "arrow" {
        format!("{}=", "=".repeat(width - 1))
    } else {
        filled_char.repeat(width)
    };

    bar.green().to_string()
}

/// Simulate doing fake work with progress bars
fn cmd_work(duration: u64, num_tasks: usize, style: &str, list_styles: bool) -> Result<()> {
    // Handle --list-styles flag
    if list_styles {
        println!("{}", "Available progress bar styles:".cyan().bold());
        println!();
        for (name, desc, _, _) in BAR_STYLES {
            let preview = render_bar(60, 20, name, false);
            println!("  {:10} [{}] {}", name.yellow(), preview, desc.dimmed());
        }
        return Ok(());
    }

    // Validate style
    if !BAR_STYLES.iter().any(|(name, _, _, _)| *name == style) {
        println!("Unknown style: {}", style.red());
        println!("Use --list-styles to see available styles.");
        return Ok(());
    }

    let mut rng = rand::rng();
    let total_ms = duration * 1000;
    let time_per_task = total_ms / num_tasks as u64;

    // Shuffle and pick tasks
    let mut task_indices: Vec<usize> = (0..WORK_TASKS.len()).collect();
    for i in (1..task_indices.len()).rev() {
        let j = rng.random_range(0..=i);
        task_indices.swap(i, j);
    }
    let selected_tasks: Vec<&str> = task_indices
        .into_iter()
        .take(num_tasks)
        .map(|i| WORK_TASKS[i])
        .collect();

    // Find max task name length for alignment
    let max_task_len = selected_tasks.iter().map(|t| t.len()).max().unwrap_or(0);

    println!("{}", "Starting build process...".cyan().bold());
    println!();

    for (task_idx, task) in selected_tasks.iter().enumerate() {
        let overall_progress = (task_idx * 100) / num_tasks;
        osc_progress(overall_progress as u64, ProgressState::Normal);

        // Pad task name for alignment
        let padded_task = format!("{:width$}", task, width = max_task_len);

        // Print task header
        print!(
            "{} {}{}",
            format!("[{}/{}]", task_idx + 1, num_tasks).dimmed(),
            padded_task.cyan(),
            "...".dimmed()
        );
        io::stdout().flush().ok();

        // Animate progress for this task
        let task_start = Instant::now();
        let mut progress = 0u64;
        let mut frame = 0usize;

        while progress < 100 {
            // Random increment with occasional stalls
            let increment = if rng.random_bool(0.1) {
                thread::sleep(Duration::from_millis(200));
                1
            } else {
                rng.random_range(2..8)
            };

            progress = (progress + increment).min(100);

            // Calculate remaining time
            let elapsed = task_start.elapsed().as_millis() as u64;
            let target_progress = ((elapsed * 100) / time_per_task).min(100);
            progress = progress.max(target_progress);

            // Draw inline progress - spinner on right side
            let spinner = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()];
            let bar = render_bar(progress, 20, style, true);

            print!(
                "\r{} {}... [{}] {} {:>3}%",
                format!("[{}/{}]", task_idx + 1, num_tasks).dimmed(),
                padded_task.cyan(),
                bar,
                spinner.green(),
                progress
            );
            io::stdout().flush().ok();

            frame += 1;
            thread::sleep(Duration::from_millis(60));

            // Exit if we've spent enough time
            if task_start.elapsed().as_millis() >= time_per_task as u128 {
                progress = 100;
            }
        }

        // Complete this task - show filled bar with checkmark on right
        println!(
            "\r{} {}... [{}] {} {}",
            format!("[{}/{}]", task_idx + 1, num_tasks).dimmed(),
            padded_task.cyan(),
            render_bar_complete(20, style),
            "✓".green(),
            "Done!".green()
        );
    }

    osc_progress_clear();
    println!();
    println!(
        "{} {} tasks completed successfully!",
        "✓".green().bold(),
        num_tasks
    );

    Ok(())
}

/// Programming fortunes/wisdom
const FORTUNES: &[&str] = &[
    "There are only two hard things in CS: cache invalidation and naming things.",
    "It works on my machine!",
    "// TODO: fix this later",
    "Have you tried turning it off and on again?",
    "The best code is no code at all.",
    "It's not a bug, it's a feature.",
    "99 little bugs in the code, 99 little bugs. Take one down, patch it around, 127 little bugs in the code.",
    "A good programmer is someone who always looks both ways before crossing a one-way street.",
    "Deleted code is debugged code.",
    "First, solve the problem. Then, write the code.",
    "Code never lies, comments sometimes do.",
    "Any fool can write code that a computer can understand. Good programmers write code that humans can understand.",
    "Programming is like writing a book... except if you miss a single comma on page 126 the whole thing makes no sense.",
    "The computer was born to solve problems that did not exist before.",
    "Weeks of coding can save you hours of planning.",
    "If debugging is the process of removing bugs, then programming must be the process of putting them in.",
    "The only way to learn a new programming language is by writing programs in it.",
    "Programming today is a race between software engineers striving to build bigger and better idiot-proof programs, and the universe trying to build bigger and better idiots. So far, the universe is winning.",
    "I don't always test my code, but when I do, I do it in production.",
    "There's no place like 127.0.0.1",
    "SELECT * FROM users WHERE clue > 0; -- 0 rows returned",
    "git commit -m 'Fixed a bug' (narrator: they did not fix the bug)",
    "There are 10 kinds of people: those who understand binary and those who don't.",
    "A SQL query walks into a bar, walks up to two tables and asks... 'Can I join you?'",
    "Why do programmers prefer dark mode? Because light attracts bugs.",
    "Algorithm: a word used by programmers when they don't want to explain what they did.",
    "Documentation is like sex: when it's good, it's very good; when it's bad, it's better than nothing.",
    "Copy and paste is a design pattern.",
    "Real programmers count from 0.",
    "The S in IoT stands for Security.",
    "In theory, there is no difference between theory and practice. In practice, there is.",
    "Measuring programming progress by lines of code is like measuring aircraft building progress by weight.",
    "The code you write makes you a programmer. The code you delete makes you a good one. The code you don't have to write makes you a great one.",
    "Talk is cheap. Show me the code.",
    "Software is like entropy: it is difficult to grasp, weighs nothing, and obeys the Second Law of Thermodynamics; i.e., it always increases.",
];

/// ASCII art animals
const ANIMALS: &[(&str, &[&str])] = &[
    (
        "cow",
        &[
            r"        \   ^__^",
            r"         \  (oo)\_______",
            r"            (__)\       )\/\",
            r"                ||----w |",
            r"                ||     ||",
        ],
    ),
    (
        "tux",
        &[
            r"   \",
            r"    \",
            r"        .-.",
            r"       |o_o |",
            r"       |:_/ |",
            r"      //   \ \",
            r"     (|     | )",
            r"    /'\_   _/`\",
            r"    \___)=(___/",
        ],
    ),
    (
        "ghost",
        &[
            r"   \",
            r"    \",
            r"     .-..",
            r"    ( o o )",
            r"    |  O  |",
            r"    |     |",
            r"    '~~~~~'",
        ],
    ),
    (
        "dragon",
        &[
            r"      \                    / \  //\",
            r"       \    |\___/|      /   \//  \\",
            r"            /0  0  \__  /    //  | \ \",
            r"           /     /  \/_/    //   |  \  \",
            r"           @_^_@'/   \/_   //    |   \   \",
            r"           //_^_/     \/_ //     |    \    \",
            r"        ( //) |        \///      |     \     \",
            r"      ( / /) _|_ /   )  //       |      \     _\",
            r"    ( // /) '/,_ _ _/  ( ; -.    |    _ _\.-~        .-~~~^-.",
            r"  (( / / )) ,-{        _      `-.|.-~-.           .googl    `,",
            r" (( // / ))  '/\      /                 ~-. _ .-~      .-~^-.  \",
            r" (( /// ))      `.   {            }                   /      \  \",
            r"  (( / ))     .googl `googl       googl  googl.googl /googl  }  \\",
            r"              googl   googl      googl    googl.goggoogglgoglgoggl|",
        ],
    ),
    (
        "cat",
        &[r"  \", r"   \", r"    /\_/\", r"   ( o.o )", r"    > ^ <"],
    ),
    (
        "dog",
        &[
            r"  \",
            r"   \",
            r"        / \__",
            r"       (    @\___",
            r"       /         O",
            r"      /   (_____/",
            r"     /_____/   U",
        ],
    ),
];

/// Show random programming wisdom with ASCII art
fn cmd_fortune(animal: Option<String>, say: Option<String>, list: bool) -> Result<()> {
    // Handle --list flag
    if list {
        println!("{}", "Available animals:".cyan().bold());
        for (name, _) in ANIMALS {
            println!("  {}", name);
        }
        return Ok(());
    }

    // Pick fortune or use custom message
    let message = if let Some(custom) = say {
        custom
    } else {
        let mut rng = rand::rng();
        FORTUNES[rng.random_range(0..FORTUNES.len())].to_string()
    };

    // Pick animal
    let selected_animal = if let Some(ref name) = animal {
        ANIMALS
            .iter()
            .find(|(n, _)| *n == name.as_str())
            .map(|(_, art)| *art)
    } else {
        let mut rng = rand::rng();
        Some(ANIMALS[rng.random_range(0..ANIMALS.len())].1)
    };

    let animal_art = match selected_animal {
        Some(art) => art,
        None => {
            println!("Unknown animal: {}", animal.unwrap());
            println!("Use --list to see available animals.");
            return Ok(());
        }
    };

    // Build speech bubble
    let max_width = 60;
    let words: Vec<&str> = message.split_whitespace().collect();
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Find max line width
    let bubble_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Print speech bubble
    println!(" {}", "_".repeat(bubble_width + 2));

    for (i, line) in lines.iter().enumerate() {
        let padding = " ".repeat(bubble_width - line.len());
        let (left, right) = if lines.len() == 1 {
            ('<', '>')
        } else if i == 0 {
            ('/', '\\')
        } else if i == lines.len() - 1 {
            ('\\', '/')
        } else {
            ('|', '|')
        };
        println!("{} {}{} {}", left, line.yellow(), padding, right);
    }

    println!(" {}", "-".repeat(bubble_width + 2));

    // Print animal
    for line in animal_art {
        println!("{}", line);
    }

    Ok(())
}

/// Bouncing indeterminate progress bar
fn cmd_bounce(duration: u64, message: Option<String>) -> Result<()> {
    let msg = message.as_deref().unwrap_or("Loading...");
    let total_ms = duration * 1000;
    let start = Instant::now();

    let mut bar = BouncingBar::new();

    while start.elapsed().as_millis() < total_ms as u128 {
        bar.tick();
        bar.draw(Some(msg));
        thread::sleep(Duration::from_millis(50));
    }

    bar.finish_with_message(&format!("{} Done!", "✓".green()));

    Ok(())
}

// ============================================================================
// ASCII CLOCK
// ============================================================================

/// Big ASCII digits for clock display (5 rows each)
const ASCII_DIGITS: [&[&str]; 10] = [
    // 0
    &[" ██████ ", "██    ██", "██    ██", "██    ██", " ██████ "],
    // 1
    &["   ██   ", " ████   ", "   ██   ", "   ██   ", " ██████ "],
    // 2
    &[" ██████ ", "      ██", " ██████ ", "██      ", " ██████ "],
    // 3
    &[" ██████ ", "      ██", "  █████ ", "      ██", " ██████ "],
    // 4
    &["██    ██", "██    ██", " ███████", "      ██", "      ██"],
    // 5
    &[" ██████ ", "██      ", " ██████ ", "      ██", " ██████ "],
    // 6
    &[" ██████ ", "██      ", " ██████ ", "██    ██", " ██████ "],
    // 7
    &[" ███████", "      ██", "     ██ ", "    ██  ", "   ██   "],
    // 8
    &[" ██████ ", "██    ██", " ██████ ", "██    ██", " ██████ "],
    // 9
    &[" ██████ ", "██    ██", " ███████", "      ██", " ██████ "],
];

const ASCII_COLON: &[&str] = &["   ", " ██", "   ", " ██", "   "];

/// Big ASCII clock display
fn cmd_clock(duration: u64, twelve_hour: bool, show_seconds: bool) -> Result<()> {
    let start = Instant::now();

    // Enable raw mode for keyboard input (q/Esc to quit)
    enable_raw_mode();

    // Hide cursor
    print!("\x1b[?25l");
    io::stdout().flush().ok();

    // Install Ctrl+C handler
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc_setup(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        // Check duration
        if duration > 0 && start.elapsed().as_secs() >= duration {
            break;
        }

        // Check for q or Escape
        if check_quit_key() {
            break;
        }

        let now = chrono::Local::now();
        let (hour, minute, second) = if twelve_hour {
            let h = now.format("%I").to_string().parse::<u32>().unwrap_or(12);
            (
                h,
                now.format("%M").to_string(),
                now.format("%S").to_string(),
            )
        } else {
            (
                now.format("%H").to_string().parse::<u32>().unwrap_or(0),
                now.format("%M").to_string(),
                now.format("%S").to_string(),
            )
        };

        // Get terminal size for centering
        let (term_width, term_height) = terminal_size::terminal_size()
            .map(|(w, h)| (w.0 as usize, h.0 as usize))
            .unwrap_or((80, 24));

        // Calculate clock width (each digit is ~9 chars wide, colon is 4)
        let clock_width = if show_seconds {
            // HH:MM:SS = 6 digits + 2 colons
            6 * 9 + 2 * 4
        } else {
            // HH:MM = 4 digits + 1 colon
            4 * 9 + 4
        };

        let h_padding = term_width.saturating_sub(clock_width) / 2;
        let v_padding = term_height.saturating_sub(7) / 2; // 5 rows + 2 for AM/PM

        // Clear screen and move to top
        print!("\x1b[2J\x1b[H");

        // Build time digits
        let hour_str = format!("{:02}", hour);
        let digits: Vec<usize> = if show_seconds {
            vec![
                hour_str.chars().next().unwrap().to_digit(10).unwrap_or(0) as usize,
                hour_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize,
                10, // colon
                minute.chars().next().unwrap().to_digit(10).unwrap_or(0) as usize,
                minute.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize,
                10, // colon
                second.chars().next().unwrap().to_digit(10).unwrap_or(0) as usize,
                second.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize,
            ]
        } else {
            vec![
                hour_str.chars().next().unwrap().to_digit(10).unwrap_or(0) as usize,
                hour_str.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize,
                10, // colon
                minute.chars().next().unwrap().to_digit(10).unwrap_or(0) as usize,
                minute.chars().nth(1).unwrap().to_digit(10).unwrap_or(0) as usize,
            ]
        };

        // Vertical padding
        for _ in 0..v_padding {
            println!();
        }

        // Print each row
        for row in 0..5 {
            print!("{}", " ".repeat(h_padding));
            for &d in &digits {
                let pattern = if d == 10 {
                    ASCII_COLON[row]
                } else {
                    ASCII_DIGITS[d][row]
                };
                print!("{} ", pattern.cyan());
            }
            println!();
        }

        // Show AM/PM for 12-hour
        if twelve_hour {
            let ampm = now.format("%p").to_string();
            println!("{}{}", " ".repeat(h_padding), ampm.dimmed());
        }

        io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(200));
    }

    // Disable raw mode and show cursor
    disable_raw_mode();
    print!("\x1b[?25h");
    io::stdout().flush().ok();

    Ok(())
}

// ============================================================================
// QR CODE
// ============================================================================

/// Generate QR code in terminal
fn cmd_qr(text: &str, invert: bool) -> Result<()> {
    use qrcode::QrCode;

    let code = QrCode::new(text.as_bytes()).map_err(|e| anyhow::anyhow!("QR error: {}", e))?;

    let (dark, light) = if invert {
        ("  ", "██")
    } else {
        ("██", "  ")
    };

    // Add quiet zone (border)
    let width = code.width();

    // Top border
    for _ in 0..width + 4 {
        print!("{}", light);
    }
    println!();
    for _ in 0..width + 4 {
        print!("{}", light);
    }
    println!();

    // QR code with side borders
    for y in 0..width {
        print!("{}{}", light, light); // Left border
        for x in 0..width {
            let module = code[(x, y)];
            let is_dark = module == qrcode::Color::Dark;
            print!("{}", if is_dark { dark } else { light });
        }
        println!("{}{}", light, light); // Right border
    }

    // Bottom border
    for _ in 0..width + 4 {
        print!("{}", light);
    }
    println!();
    for _ in 0..width + 4 {
        print!("{}", light);
    }
    println!();

    Ok(())
}

// ============================================================================
// GAME OF LIFE
// ============================================================================

/// Conway's Game of Life
fn cmd_life(duration: u64, pattern: &str, width: usize, height: usize) -> Result<()> {
    let mut rng = rand::rng();
    let start = Instant::now();

    // Enable raw mode for keyboard input (q/Esc to quit)
    enable_raw_mode();

    // Initialize grid
    let mut grid = vec![vec![false; width]; height];

    match pattern {
        "glider" => {
            // Classic glider at top-left
            if height > 3 && width > 3 {
                grid[1][2] = true;
                grid[2][3] = true;
                grid[3][1] = true;
                grid[3][2] = true;
                grid[3][3] = true;
            }
        }
        "blinker" => {
            // Simple blinker in center
            let cy = height / 2;
            let cx = width / 2;
            if cy > 0 && cx > 1 {
                grid[cy][cx - 1] = true;
                grid[cy][cx] = true;
                grid[cy][cx + 1] = true;
            }
        }
        "pulsar" => {
            // Pulsar pattern (period 3)
            let cy = height / 2;
            let cx = width / 2;
            if height > 15 && width > 15 {
                // Pulsar is complex, add simplified version
                for &dy in &[-6i32, -1, 1, 6] {
                    for &dx in &[-4i32, -3, -2, 2, 3, 4] {
                        let y = (cy as i32 + dy) as usize;
                        let x = (cx as i32 + dx) as usize;
                        if y < height && x < width {
                            grid[y][x] = true;
                        }
                    }
                }
            }
        }
        _ => {
            // Random
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = rng.random_bool(0.3);
                }
            }
        }
    }

    // Hide cursor
    print!("\x1b[?25l");
    io::stdout().flush().ok();

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc_setup(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    let mut generation = 0u64;

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        if duration > 0 && start.elapsed().as_secs() >= duration {
            break;
        }

        // Check for q or Escape
        if check_quit_key() {
            break;
        }

        // Clear and draw
        print!("\x1b[2J\x1b[H");
        println!(
            "{} Generation: {}\n",
            "Game of Life".cyan().bold(),
            generation
        );

        for row in &grid {
            for &cell in row {
                if cell {
                    print!("{}", "██".green());
                } else {
                    print!("  ");
                }
            }
            println!();
        }

        println!("\n{}", "Press q, Esc, or Ctrl+C to exit".dimmed());
        io::stdout().flush().ok();

        // Compute next generation
        let mut next = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                let neighbors = count_neighbors(&grid, x, y, width, height);
                next[y][x] = matches!((grid[y][x], neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        grid = next;
        generation += 1;

        thread::sleep(Duration::from_millis(100));
    }

    // Disable raw mode and show cursor
    disable_raw_mode();
    print!("\x1b[?25h");
    io::stdout().flush().ok();

    Ok(())
}

fn count_neighbors(grid: &[Vec<bool>], x: usize, y: usize, width: usize, height: usize) -> usize {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as i32 + dx).rem_euclid(width as i32) as usize;
            let ny = (y as i32 + dy).rem_euclid(height as i32) as usize;
            if grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}

// ============================================================================
// MATRIX RAIN
// ============================================================================

/// Matrix-style falling code rain
fn cmd_matrix(duration: u64, density: u8) -> Result<()> {
    let mut rng = rand::rng();
    let start = Instant::now();

    // Enable raw mode for keyboard input (q/Esc to quit)
    enable_raw_mode();

    // Get terminal size
    let (width, height) = terminal_size::terminal_size()
        .map(|(w, h)| (w.0 as usize, h.0 as usize))
        .unwrap_or((80, 24));

    // Katakana-like characters + digits
    let chars: Vec<char> = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ0123456789"
        .chars()
        .collect();

    // Column states: (position, speed, length, active)
    let num_columns = width / 2;
    let mut columns: Vec<(f32, f32, usize, bool)> = (0..num_columns)
        .map(|_| {
            (
                rng.random_range(-(height as f32)..0.0),
                rng.random_range(0.3..1.0),
                rng.random_range(5..15),
                rng.random_bool(density as f64 / 10.0),
            )
        })
        .collect();

    // Screen buffer
    let mut screen: Vec<Vec<char>> = vec![vec![' '; num_columns]; height];

    // Hide cursor
    print!("\x1b[?25l");
    io::stdout().flush().ok();

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc_setup(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        if duration > 0 && start.elapsed().as_secs() >= duration {
            break;
        }

        // Check for q or Escape
        if check_quit_key() {
            break;
        }

        // Update columns
        for (i, (pos, speed, len, active)) in columns.iter_mut().enumerate() {
            if !*active {
                if rng.random_bool(0.02 * density as f64) {
                    *active = true;
                    *pos = 0.0;
                    *len = rng.random_range(5..15);
                    *speed = rng.random_range(0.3..1.0);
                }
                continue;
            }

            *pos += *speed;

            // Add character at head position
            let head_y = *pos as i32;
            if head_y >= 0 && (head_y as usize) < height {
                screen[head_y as usize][i] = chars[rng.random_range(0..chars.len())];
            }

            // Clear tail
            let tail_y = head_y - (*len as i32);
            if tail_y >= 0 && (tail_y as usize) < height {
                screen[tail_y as usize][i] = ' ';
            }

            // Reset if off screen
            if tail_y > height as i32 {
                *active = false;
            }
        }

        // Render
        print!("\x1b[H"); // Move to top
        for (y, row) in screen.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                let head_y = columns[x].0 as i32;
                if ch != ' ' {
                    if y as i32 == head_y {
                        // Bright head
                        print!("\x1b[97m{}\x1b[0m", ch); // White
                    } else if y as i32 > head_y - 3 {
                        // Bright green near head
                        print!("\x1b[92m{}\x1b[0m", ch);
                    } else {
                        // Dark green
                        print!("\x1b[32m{}\x1b[0m", ch);
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        io::stdout().flush().ok();
        thread::sleep(Duration::from_millis(50));
    }

    // Disable raw mode, restore cursor and clear
    disable_raw_mode();
    print!("\x1b[?25h\x1b[2J\x1b[H");
    io::stdout().flush().ok();

    Ok(())
}

// ============================================================================
// ASCII BANNER
// ============================================================================

/// Simple block letter font (each letter is 5 rows x variable width)
const BANNER_FONT: &[(char, &[&str])] = &[
    ('A', &["  █  ", " █ █ ", "█████", "█   █", "█   █"]),
    ('B', &["████ ", "█   █", "████ ", "█   █", "████ "]),
    ('C', &[" ████", "█    ", "█    ", "█    ", " ████"]),
    ('D', &["████ ", "█   █", "█   █", "█   █", "████ "]),
    ('E', &["█████", "█    ", "████ ", "█    ", "█████"]),
    ('F', &["█████", "█    ", "████ ", "█    ", "█    "]),
    ('G', &[" ████", "█    ", "█  ██", "█   █", " ████"]),
    ('H', &["█   █", "█   █", "█████", "█   █", "█   █"]),
    ('I', &["█████", "  █  ", "  █  ", "  █  ", "█████"]),
    ('J', &["█████", "   █ ", "   █ ", "█  █ ", " ██  "]),
    ('K', &["█   █", "█  █ ", "███  ", "█  █ ", "█   █"]),
    ('L', &["█    ", "█    ", "█    ", "█    ", "█████"]),
    ('M', &["█   █", "██ ██", "█ █ █", "█   █", "█   █"]),
    ('N', &["█   █", "██  █", "█ █ █", "█  ██", "█   █"]),
    ('O', &[" ███ ", "█   █", "█   █", "█   █", " ███ "]),
    ('P', &["████ ", "█   █", "████ ", "█    ", "█    "]),
    ('Q', &[" ███ ", "█   █", "█   █", "█  █ ", " ██ █"]),
    ('R', &["████ ", "█   █", "████ ", "█  █ ", "█   █"]),
    ('S', &[" ████", "█    ", " ███ ", "    █", "████ "]),
    ('T', &["█████", "  █  ", "  █  ", "  █  ", "  █  "]),
    ('U', &["█   █", "█   █", "█   █", "█   █", " ███ "]),
    ('V', &["█   █", "█   █", "█   █", " █ █ ", "  █  "]),
    ('W', &["█   █", "█   █", "█ █ █", "██ ██", "█   █"]),
    ('X', &["█   █", " █ █ ", "  █  ", " █ █ ", "█   █"]),
    ('Y', &["█   █", " █ █ ", "  █  ", "  █  ", "  █  "]),
    ('Z', &["█████", "   █ ", "  █  ", " █   ", "█████"]),
    ('0', &[" ███ ", "█  ██", "█ █ █", "██  █", " ███ "]),
    ('1', &[" █   ", "██   ", " █   ", " █   ", "███  "]),
    ('2', &[" ███ ", "█   █", "  ██ ", " █   ", "█████"]),
    ('3', &["████ ", "    █", " ███ ", "    █", "████ "]),
    ('4', &["█  █ ", "█  █ ", "█████", "   █ ", "   █ "]),
    ('5', &["█████", "█    ", "████ ", "    █", "████ "]),
    ('6', &[" ███ ", "█    ", "████ ", "█   █", " ███ "]),
    ('7', &["█████", "    █", "   █ ", "  █  ", " █   "]),
    ('8', &[" ███ ", "█   █", " ███ ", "█   █", " ███ "]),
    ('9', &[" ███ ", "█   █", " ████", "    █", " ███ "]),
    (' ', &["     ", "     ", "     ", "     ", "     "]),
    ('!', &["  █  ", "  █  ", "  █  ", "     ", "  █  "]),
    ('.', &["     ", "     ", "     ", "     ", "  █  "]),
    ('-', &["     ", "     ", "█████", "     ", "     "]),
];

/// Big ASCII text banner
fn cmd_banner(text: &str) -> Result<()> {
    let text = text.to_uppercase();

    // Build 5 rows
    let mut rows: Vec<String> = vec![String::new(); 5];

    for ch in text.chars() {
        if let Some((_, pattern)) = BANNER_FONT.iter().find(|(c, _)| *c == ch) {
            for (i, line) in pattern.iter().enumerate() {
                rows[i].push_str(line);
                rows[i].push(' '); // spacing between letters
            }
        } else {
            // Unknown char - add space
            for row in &mut rows {
                row.push_str("     ");
                row.push(' ');
            }
        }
    }

    // Print with color
    println!();
    for row in &rows {
        println!("  {}", row.cyan());
    }
    println!();

    Ok(())
}

// ============================================================================
// HELPERS
// ============================================================================

/// Setup Ctrl+C handler (best effort)
fn ctrlc_setup<F>(handler: F)
where
    F: FnOnce() + Send + 'static,
{
    // Simple best-effort handler using std
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let handler = std::sync::Mutex::new(Some(handler));
        let _ = ctrlc::set_handler(move || {
            if let Ok(mut guard) = handler.lock() {
                if let Some(f) = guard.take() {
                    f();
                }
            }
        });
    }));
}

/// Check if q or Escape was pressed (non-blocking)
#[cfg(unix)]
fn check_quit_key() -> bool {
    use std::os::unix::io::AsRawFd;

    let stdin_fd = std::io::stdin().as_raw_fd();

    // Try to read a byte without blocking (VMIN=0, VTIME=0 should already be set)
    let mut buf = [0u8; 1];
    match nix::unistd::read(stdin_fd, &mut buf) {
        Ok(1) => buf[0] == b'q' || buf[0] == b'Q' || buf[0] == 27, // 27 = Escape
        _ => false,
    }
}

#[cfg(not(unix))]
fn check_quit_key() -> bool {
    false // q/Esc not supported on Windows, use Ctrl+C
}

/// Enable raw mode for keyboard input (Unix only)
#[cfg(unix)]
fn enable_raw_mode() {
    let stdin = std::io::stdin();

    if let Ok(mut termios) = nix::sys::termios::tcgetattr(&stdin) {
        // Disable canonical mode and echo
        termios.local_flags &=
            !(nix::sys::termios::LocalFlags::ICANON | nix::sys::termios::LocalFlags::ECHO);
        // Non-blocking reads
        termios.control_chars[nix::sys::termios::SpecialCharacterIndices::VMIN as usize] = 0;
        termios.control_chars[nix::sys::termios::SpecialCharacterIndices::VTIME as usize] = 0;

        let _ = nix::sys::termios::tcsetattr(&stdin, nix::sys::termios::SetArg::TCSANOW, &termios);
    }
}

#[cfg(not(unix))]
fn enable_raw_mode() {}

/// Disable raw mode (Unix only)
#[cfg(unix)]
fn disable_raw_mode() {
    let stdin = std::io::stdin();

    if let Ok(mut termios) = nix::sys::termios::tcgetattr(&stdin) {
        // Re-enable canonical mode and echo
        termios.local_flags |=
            nix::sys::termios::LocalFlags::ICANON | nix::sys::termios::LocalFlags::ECHO;

        let _ = nix::sys::termios::tcsetattr(&stdin, nix::sys::termios::SetArg::TCSANOW, &termios);
    }
}

#[cfg(not(unix))]
fn disable_raw_mode() {}
