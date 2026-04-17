use clap::Parser;
use colored::*;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "adi", about = "Android Device Info — fast system info via ADB")]
struct Cli {
    #[arg(short, long)]
    json: bool,
    #[arg(short, long)]
    filter: Option<String>,
}

fn adb_shell(cmd: &str) -> String {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn prop(key: &str) -> String {
    adb_shell(&format!("getprop {}", key)).trim().to_string()
}

#[derive(serde::Serialize)]
struct DeviceInfo {
    model: String,
    brand: String,
    codename: String,
    android_version: String,
    api_level: String,
    security_patch: String,
    fingerprint: String,
    build_type: String,
    cpu_abi: String,
    cpu_cores: String,
    ram_total: String,
    storage_internal: String,
    battery_level: String,
    battery_health: String,
}

fn main() {
    let cli = Cli::parse();

    let device = DeviceInfo {
        model: prop("ro.product.model"),
        brand: prop("ro.product.brand"),
        codename: prop("ro.product.device"),
        android_version: prop("ro.build.version.release"),
        api_level: prop("ro.build.version.sdk"),
        security_patch: prop("ro.build.version.security_patch"),
        fingerprint: prop("ro.build.fingerprint"),
        build_type: prop("ro.build.type"),
        cpu_abi: prop("ro.product.cpu.abi"),
        cpu_cores: adb_shell("nproc").trim().to_string(),
        ram_total: adb_shell("cat /proc/meminfo | grep MemTotal").trim().to_string(),
        storage_internal: adb_shell("df -h /data | tail -1").trim().to_string(),
        battery_level: adb_shell("dumpsys battery | grep level").trim().to_string(),
        battery_health: adb_shell("dumpsys battery | grep health").trim().to_string(),
    };

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&device).unwrap());
        return;
    }

    println!("
{}", "Device Information".bold().blue());
    println!("{}", "─".repeat(50));
    println!("  {} {}", "Model:".cyan(), device.model.white());
    println!("  {} {}", "Brand:".cyan(), device.brand.white());
    println!("  {} {}", "Device:".cyan(), device.codename.white());
    println!("  {} {} (API {})", "Android:".cyan(), device.android_version.white(), device.api_level.white());
    println!("  {} {}", "Security patch:".cyan(), device.security_patch.white());
    println!("  {} {}", "Build type:".cyan(), device.build_type.white());
    println!("  {} {}", "CPU:".cyan(), device.cpu_abi.white());
    println!("  {} {} cores", "Cores:".cyan(), device.cpu_cores.white());
    println!("  {} {}", "RAM:".cyan(), device.ram_total.white());
    println!("  {} {}", "Storage:".cyan(), device.storage_internal.white());
    println!("  {} {}", "Battery:".cyan(), device.battery_level.white());
    println!();
}
