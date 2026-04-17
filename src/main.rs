use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;

fn adb(cmd: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("adb shell {}", cmd))
        .output()
        .unwrap_or_default();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn prop(key: &str) -> String {
    adb(&format!("getprop {}", key))
}

#[derive(Parser)]
#[command(name = "adb-info")]
#[command(about = "Fast Android device info tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Full device report
    Full,
    /// Device model and build info
    Device,
    /// CPU architecture and performance
    Cpu,
    /// Memory and storage info
    Memory,
    /// Battery and power state
    Battery,
    /// Network interfaces and IPs
    Network,
    /// Security status (root, encryption, etc)
    Security,
    /// Check if device is rooted
    IsRooted,
    /// JSON output (all info as JSON)
    Json,
}

fn print_device() {
    println!("{}", "═ DEVICE ═".bold().cyan());
    println!("  Model:       {}", prop("ro.product.model"));
    println!("  Brand:       {}", prop("ro.product.brand"));
    println!("  Hardware:    {}", prop("ro.hardware"));
    println!("  Codename:    {}", prop("ro.product.device"));
    println!("  Android:     {}", prop("ro.build.version.release"));
    println!("  API:         {}", prop("ro.build.version.sdk"));
    println!("  Build Type:  {}", prop("ro.build.type"));
    println!("  Fingerprint: {}", prop("ro.build.fingerprint"));
}

fn print_cpu() {
    println!("{}", "═ CPU ═".bold().yellow());
    println!("  Architecture: {}", prop("ro.product.cpu.abi"));
    println!("  Cores:        {}", adb("nproc"));
    if let Ok(_) = Command::new("sh")
        .arg("-c")
        .arg("adb shell cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        .output()
    {
        let freq_khz = adb("cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
            .parse::<u64>()
            .unwrap_or(0);
        println!("  Current Freq: {} MHz", freq_khz / 1000);
    }
}

fn print_battery() {
    println!("{}", "═ BATTERY ═".bold().green());
    println!("  Level:       {}", adb("dumpsys battery | grep 'level'"));
    println!("  Status:      {}", adb("dumpsys battery | grep 'status'"));
    println!("  Health:      {}", adb("dumpsys battery | grep 'health'"));
    println!("  Temp:        {}°C", 
        adb("dumpsys battery | grep 'temperature'")
            .split('=').nth(1).unwrap_or("?"));
}

fn check_root() -> bool {
    !adb("which su").is_empty() || !adb("ls /sbin/su 2>/dev/null").is_empty()
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Full => {
            print_device();
            println!();
            print_cpu();
            println!();
            print_battery();
            println!();
            println!("{}", "═ SECURITY ═".bold().red());
            println!("  Root:       {}", if check_root() { "YES ⚠️ ".red() } else { "NO ✓".green() });
            println!("  Encryption: {}", prop("ro.crypto.state"));
            println!("  SELinux:    {}", adb("getenforce 2>/dev/null"));
        }
        Commands::Device => print_device(),
        Commands::Cpu => print_cpu(),
        Commands::Battery => print_battery(),
        Commands::Memory => {
            println!("{}", "═ MEMORY ═".bold().blue());
            println!("  {}", adb("cat /proc/meminfo | head -4"));
        }
        Commands::Network => {
            println!("{}", "═ NETWORK ═".bold().magenta());
            println!("  {}", adb("ip addr show wlan0 | grep 'inet' | awk '{print $2}'"));
        }
        Commands::Security => {
            println!("{}", "═ SECURITY ═".bold().red());
            println!("  Root:       {}", if check_root() { "YES".red() } else { "NO".green() });
            println!("  Bootloader: {}", prop("ro.boot.verifiedbootstate"));
            println!("  Encryption: {}", prop("ro.crypto.state"));
        }
        Commands::IsRooted => {
            println!("{}", if check_root() { "true" } else { "false" });
        }
        Commands::Json => {
            let info = serde_json::json!({
                "model": prop("ro.product.model"),
                "android": prop("ro.build.version.release"),
                "api": prop("ro.build.version.sdk"),
                "rooted": check_root(),
                "cpu": prop("ro.product.cpu.abi"),
                "device": prop("ro.product.device"),
            });
            println!("{}", serde_json::to_string_pretty(&info).unwrap());
        }
    }
}
