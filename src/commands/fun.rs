//! # Fun Terminal UI Command
//!
//! Fun terminal effects like fake progress bars, hacker mode, countdown timers, and spinners.

use crate::cli::commands::fun::{FunArgs, FunCommand};
use crate::utils::progress::{
    self, osc_progress, osc_progress_clear, ProgressState, SPINNER_FRAMES,
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
        FunCommand::Work { duration, tasks } => cmd_work(duration, tasks),
        FunCommand::Fortune { animal, say, list } => cmd_fortune(animal, say, list),
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

/// Simulate doing fake work with progress bars
fn cmd_work(duration: u64, num_tasks: usize) -> Result<()> {
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
            let filled = (progress as usize * 20) / 100;
            let empty = 20 - filled;
            let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

            print!(
                "\r{} {}... [{}] {} {:>3}%",
                format!("[{}/{}]", task_idx + 1, num_tasks).dimmed(),
                padded_task.cyan(),
                bar.blue(),
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
            "█".repeat(20).green(),
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
