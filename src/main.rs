use std::process::Command;
use std::env;

fn adb(cmd: &str) -> String {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args[1] == "--all" {
        println!("📱 Android Device Info");
        println!("Model:     {}", adb("getprop ro.product.model"));
        println!("Android:   {}", adb("getprop ro.build.version.release"));
        println!("API:       {}", adb("getprop ro.build.version.sdk"));
        println!("CPU ABI:   {}", adb("getprop ro.product.cpu.abi"));
        println!("Build:     {}", adb("getprop ro.build.fingerprint"));
    } else if args[1] == "--model" {
        println!("{}", adb("getprop ro.product.model"));
    } else if args[1] == "--version" {
        println!("{}", adb("getprop ro.build.version.release"));
    } else if args[1] == "--storage" {
        println!("{}", adb("df -h /data | tail -1"));
    } else if args[1] == "--json" {
        println!("{{");
        println!("  \"model\": \"{}\",", adb("getprop ro.product.model"));
        println!("  \"android\": \"{}\",", adb("getprop ro.build.version.release"));
        println!("  \"api\": \"{}\"", adb("getprop ro.build.version.sdk"));
        println!("}}");
    }
}
