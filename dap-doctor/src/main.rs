use colored::*;
use sysinfo::System;

fn line() {
    println!("{}", "════════════════════════════════════════════".blue());
}

fn ok(name: &str, value: String) {
    println!("{:<20} {} {}", name, "✔".green(), value);
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    line();
    println!("{}", "DAP DOCTOR v0.1".bold().cyan());
    line();

    ok("Hostname", System::host_name().unwrap_or_else(|| "Unknown".into()));
    ok("Kernel", System::kernel_version().unwrap_or_else(|| "Unknown".into()));
    ok("OS", System::long_os_version().unwrap_or_else(|| "Unknown".into()));
    ok("CPU Threads", format!("{}", sys.cpus().len()));
    ok(
        "Memory",
        format!(
            "{:.1} GB / {:.1} GB",
            sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
            sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
        ),
    );

    line();
}
