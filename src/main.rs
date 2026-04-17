use clap::{Parser, Subcommand};
use std::process::Command;
use serde_json::json;

#[derive(Parser)]
#[command(name = "android-info", about = "Fast Android device info via ADB")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get device model, CPU, RAM
    Device,
    /// Get all permissions from app
    Perms { app: String },
    /// Monitor real-time network connections
    Net,
    /// Full device audit JSON
    Audit,
}

fn adb(args: &[&str]) -> String {
    let output = Command::new("adb")
        .args(args)
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Device => {
            let model = adb(&["shell", "getprop", "ro.product.model"]).trim().to_string();
            let android = adb(&["shell", "getprop", "ro.build.version.release"]).trim().to_string();
            let arch = adb(&["shell", "getprop", "ro.product.cpu.abi"]).trim().to_string();
            let ram = adb(&["shell", "cat", "/proc/meminfo"]).lines()
                .find(|l| l.starts_with("MemTotal"))
                .map(|l| l.split_whitespace().nth(1).unwrap_or("?").to_string())
                .unwrap_or_else(|| "?".to_string());
            
            println!("📱 Device Info");
            println!("  Model:      {}", model);
            println!("  Android:    {}", android);
            println!("  CPU:        {}", arch);
            println!("  RAM:        {} KB", ram);
        }
        Commands::Perms { app } => {
            let output = adb(&["shell", "dumpsys", "package", &app]);
            let perms: Vec<&str> = output.lines()
                .filter(|l| l.contains("android.permission") && l.contains("granted=true"))
                .collect();
            println!("🔐 Permissions for {}", app);
            for perm in perms {
                if let Some(p) = perm.split("android.permission.").nth(1) {
                    println!("  ✓ {}", p.split(' ').next().unwrap_or(p));
                }
            }
        }
        Commands::Net => {
            let output = adb(&["shell", "netstat", "-tulnp"]);
            println!("📡 Network Connections");
            for line in output.lines().skip(2).take(15) {
                println!("  {}", line);
            }
        }
        Commands::Audit => {
            let device = adb(&["shell", "getprop", "ro.product.model"]).trim().to_string();
            let pkgs = adb(&["shell", "pm", "list", "packages", "-3"]);
            let count = pkgs.lines().count();
            let json = json!({
                "device": device,
                "user_apps": count,
                "timestamp": chrono::Local::now().to_rfc3339(),
            });
            println!("{}", json.to_string_pretty());
        }
    }
}
