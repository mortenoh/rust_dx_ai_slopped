//! System command implementations.

use crate::cli::commands::system::{SystemArgs, SystemCommand};
use anyhow::Result;
use colored::Colorize;
use serde::Serialize;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

pub fn run(args: SystemArgs) -> Result<()> {
    match args.command {
        SystemCommand::Info { json } => cmd_info(json),
    }
}

#[derive(Serialize)]
struct SystemInfo {
    os_name: String,
    os_version: String,
    kernel_version: String,
    hostname: String,
    uptime_seconds: u64,
    cpu_name: String,
    cpu_cores: usize,
    memory_total_bytes: u64,
    memory_used_bytes: u64,
    swap_total_bytes: u64,
    swap_used_bytes: u64,
    disks: Vec<DiskInfo>,
}

#[derive(Serialize)]
struct DiskInfo {
    mount_point: String,
    total_bytes: u64,
    used_bytes: u64,
    percent_used: u8,
}

fn cmd_info(json_output: bool) -> Result<()> {
    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let disks = Disks::new_with_refreshed_list();

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime = System::uptime();

    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = sys.cpus().len();

    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    let disk_infos: Vec<DiskInfo> = disks
        .iter()
        .filter(|d| d.total_space() > 0)
        .map(|d| {
            let total = d.total_space();
            let used = total - d.available_space();
            let percent = if total > 0 {
                ((used as f64 / total as f64) * 100.0) as u8
            } else {
                0
            };
            DiskInfo {
                mount_point: d.mount_point().to_string_lossy().to_string(),
                total_bytes: total,
                used_bytes: used,
                percent_used: percent,
            }
        })
        .collect();

    let info = SystemInfo {
        os_name,
        os_version,
        kernel_version,
        hostname,
        uptime_seconds: uptime,
        cpu_name,
        cpu_cores,
        memory_total_bytes: memory_total,
        memory_used_bytes: memory_used,
        swap_total_bytes: swap_total,
        swap_used_bytes: swap_used,
        disks: disk_infos,
    };

    if json_output {
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else {
        print_info(&info);
    }

    Ok(())
}

fn print_info(info: &SystemInfo) {
    println!("{}", "System Information".bold());
    println!();

    // OS
    println!("  {:12} {} {}", "OS:".cyan(), info.os_name, info.os_version);

    // Hostname
    println!("  {:12} {}", "Host:".cyan(), info.hostname);

    // Kernel
    println!("  {:12} {}", "Kernel:".cyan(), info.kernel_version);

    // Uptime
    let uptime_str = format_uptime(info.uptime_seconds);
    println!("  {:12} {}", "Uptime:".cyan(), uptime_str);

    // CPU
    println!(
        "  {:12} {} ({} cores)",
        "CPU:".cyan(),
        info.cpu_name,
        info.cpu_cores
    );

    // Memory
    let mem_used_gb = info.memory_used_bytes as f64 / 1_073_741_824.0;
    let mem_total_gb = info.memory_total_bytes as f64 / 1_073_741_824.0;
    let mem_percent = if info.memory_total_bytes > 0 {
        (info.memory_used_bytes as f64 / info.memory_total_bytes as f64 * 100.0) as u8
    } else {
        0
    };
    let mem_str = format!(
        "{:.1} GB / {:.1} GB ({}%)",
        mem_used_gb, mem_total_gb, mem_percent
    );
    let mem_colored = if mem_percent > 80 {
        mem_str.red()
    } else if mem_percent > 60 {
        mem_str.yellow()
    } else {
        mem_str.green()
    };
    println!("  {:12} {}", "Memory:".cyan(), mem_colored);

    // Swap
    if info.swap_total_bytes > 0 {
        let swap_used_gb = info.swap_used_bytes as f64 / 1_073_741_824.0;
        let swap_total_gb = info.swap_total_bytes as f64 / 1_073_741_824.0;
        println!(
            "  {:12} {:.1} GB / {:.1} GB",
            "Swap:".cyan(),
            swap_used_gb,
            swap_total_gb
        );
    }

    // Disks (show first 3)
    for disk in info.disks.iter().take(3) {
        let used_gb = disk.used_bytes as f64 / 1_073_741_824.0;
        let total_gb = disk.total_bytes as f64 / 1_073_741_824.0;
        let disk_str = format!(
            "{:.1} GB / {:.1} GB ({}%)",
            used_gb, total_gb, disk.percent_used
        );
        let disk_colored = if disk.percent_used > 90 {
            disk_str.red()
        } else if disk.percent_used > 75 {
            disk_str.yellow()
        } else {
            disk_str.green()
        };
        let label = format!("Disk ({}):", disk.mount_point);
        // Truncate long mount points
        let label = if label.len() > 20 {
            format!("{}...:", &disk.mount_point[..10])
        } else {
            label
        };
        println!("  {:12} {}", label.cyan(), disk_colored);
    }
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours, {} minutes", days, hours, minutes)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    }
}
