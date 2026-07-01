use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;
use sysinfo::System;

#[derive(Parser)]
#[command(name = "dap-doctor")]
#[command(version = "0.3.0")]
#[command(about = "DAP Homelab Health Checker")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Show noisy raw command output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    System,
    Docker,
    Network,
    Storage,
    Report,
}

fn header(title: &str) {
    println!();
    println!("{}", "════════════════════════════════════════════".blue());
    println!("{}", title.bold().cyan());
    println!("{}", "════════════════════════════════════════════".blue());
}

fn ok(name: &str, value: impl std::fmt::Display) {
    println!("{:<24} {} {}", name, "✔".green(), value);
}

fn warn(name: &str, value: impl std::fmt::Display) {
    println!("{:<24} {} {}", name, "⚠".yellow(), value);
}

fn bad(name: &str, value: impl std::fmt::Display) {
    println!("{:<24} {} {}", name, "✘".red(), value);
}

fn cmd_output(cmd: &str, args: &[&str]) -> String {
    match Command::new(cmd).args(args).output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => String::new(),
    }
}

fn command_exists(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", cmd))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn system_report() {
    let mut sys = System::new_all();
    sys.refresh_all();

    header("DAP DOCTOR :: SYSTEM");

    ok("Host", System::host_name().unwrap_or_else(|| "Unknown".into()));
    ok("Kernel", System::kernel_version().unwrap_or_else(|| "Unknown".into()));
    ok("OS", System::long_os_version().unwrap_or_else(|| "Unknown".into()));
    ok("CPU Threads", sys.cpus().len());

    let used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let pct = if total > 0.0 { used / total * 100.0 } else { 0.0 };

    if pct > 90.0 {
        warn("Memory", format!("{:.1}/{:.1} GB ({:.0}%)", used, total, pct));
    } else {
        ok("Memory", format!("{:.1}/{:.1} GB ({:.0}%)", used, total, pct));
    }
}

fn docker_report(verbose: bool) {
    header("DAP DOCTOR :: DOCKER");

    if !command_exists("docker") {
        bad("Docker", "not installed or not in PATH");
        return;
    }

    ok("Docker", cmd_output("docker", &["--version"]).trim());

    let ps = cmd_output("docker", &["ps", "--format", "{{.Names}}|{{.Status}}"]);
    let mut total = 0;
    let mut healthy = 0;
    let mut unhealthy = 0;
    let mut restarting = 0;

    for line in ps.lines() {
        total += 1;
        let lower = line.to_lowercase();

        if lower.contains("healthy") {
            healthy += 1;
        }
        if lower.contains("unhealthy") {
            unhealthy += 1;
        }
        if lower.contains("restarting") {
            restarting += 1;
        }
    }

    ok("Running Containers", total);
    ok("Healthy Containers", healthy);

    if unhealthy > 0 {
        bad("Unhealthy Containers", unhealthy);
    } else {
        ok("Unhealthy Containers", 0);
    }

    if restarting > 0 {
        warn("Restarting Containers", restarting);
    } else {
        ok("Restarting Containers", 0);
    }

    let names = ["traefik", "dap-manual", "plex", "jellyfin", "sonarr", "radarr", "decypharr", "romm"];

    println!();
    println!("{}", "Key Services".bold());

    for name in names {
        let check = cmd_output("docker", &["ps", "--filter", &format!("name={}", name), "--format", "{{.Names}}"]);
        if check.lines().any(|l| l == name) {
            ok(name, "running");
        } else {
            warn(name, "not found/running");
        }
    }

    if verbose {
        header("DOCKER :: VERBOSE");
        println!("{}", cmd_output("docker", &["ps"]));
    }
}

fn network_report(verbose: bool) {
    header("DAP DOCTOR :: NETWORK");

    let route = cmd_output("ip", &["route"]);
    let default_route = route.lines().find(|l| l.starts_with("default")).unwrap_or("No default route");

    if default_route == "No default route" {
        bad("Default Route", default_route);
    } else {
        ok("Default Route", default_route);
    }

    let ip_brief = cmd_output("ip", &["-brief", "addr"]);
    let primary = ip_brief
        .lines()
        .find(|l| l.contains("192.168.1.78"))
        .unwrap_or("Primary interface not detected");

    if primary.contains("192.168.1.78") {
        ok("Saltbox IP", "192.168.1.78");
    } else {
        warn("Saltbox IP", primary);
    }

    let networks = cmd_output("docker", &["network", "ls", "--format", "{{.Name}}"]);
    if networks.lines().any(|n| n == "saltbox") {
        ok("Docker Network", "saltbox present");
    } else {
        bad("Docker Network", "saltbox missing");
    }

    if verbose {
        header("NETWORK :: VERBOSE");
        println!("{}", cmd_output("ip", &["addr"]));
        println!("{}", cmd_output("ip", &["route"]));
    }
}

fn storage_report(verbose: bool) {
    header("DAP DOCTOR :: STORAGE");

    let df = cmd_output("df", &["-h", "/"]);
    if df.contains('/') {
        ok("Root Filesystem", "available");
    } else {
        warn("Root Filesystem", "could not read");
    }

    let mounts = cmd_output("findmnt", &["-rn", "-o", "TARGET,FSTYPE"]);

    let important = [
        "/mnt/unionfs",
        "/mnt/decypharr",
        "/mnt/altmount",
        "/mnt/nvme2",
        "/mnt/remote/google",
    ];

    for mount in important {
        if mounts.lines().any(|l| l.starts_with(mount)) {
            ok(mount, "mounted");
        } else {
            warn(mount, "not mounted");
        }
    }

    let suspicious = cmd_output("sh", &["-c", "findmnt -rn -o TARGET | grep -E '/mnt/(games|ngc|saturn|psx|gba|x32|x68000)|Shenmue' || true"]);

    if suspicious.trim().is_empty() {
        ok("ROM/FUSE Mounts", "no obvious stale mounts found");
    } else {
        warn("ROM/FUSE Mounts", "possible stale mounts detected");
    }

    if verbose {
        header("STORAGE :: VERBOSE");
        println!("{}", cmd_output("df", &["-h"]));
        println!("{}", cmd_output("findmnt", &[]));
    }
}

fn markdown_report() {
    use std::fs;

    let mut sys = System::new_all();
    sys.refresh_all();

    let host = System::host_name().unwrap_or_else(|| "Unknown".into());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".into());
    let os = System::long_os_version().unwrap_or_else(|| "Unknown".into());

    let report = format!(
        r#"# DAP Doctor Health Report

Generated automatically by DAP Doctor v0.3.

## System

| Item | Value |
|---|---|
| Host | {} |
| Kernel | {} |
| OS | {} |
| CPU Threads | {} |
| Memory Used | {:.1} GB |
| Memory Total | {:.1} GB |

## Status

🟢 Initial health report generated.

!!! note
    This report is currently basic. Future versions will include Docker, storage, network, AI, gaming, and Proxmox checks.
"#,
        host,
        kernel,
        os,
        sys.cpus().len(),
        sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
    );

    fs::create_dir_all("../docs/99-Inventory").unwrap();
    fs::write("../docs/99-Inventory/Health-Report.md", report).unwrap();

    ok("Generated", "../docs/99-Inventory/Health-Report.md");
}

fn all(verbose: bool) {
    system_report();
    docker_report(verbose);
    network_report(verbose);
    storage_report(verbose);

    println!();
    println!("{}", "════════════════════════════════════════════".green());
    println!("{}", "DAP DOCTOR COMPLETE".bold().green());
    println!("{}", "════════════════════════════════════════════".green());
}

fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::System) {
        Commands::System => system_report(),
        Commands::Docker => docker_report(cli.verbose),
        Commands::Network => network_report(cli.verbose),
        Commands::Storage => storage_report(cli.verbose),
        Commands::Report => markdown_report(),
    }

    if std::env::args().len() == 1 {
        all(false);
    }
}
