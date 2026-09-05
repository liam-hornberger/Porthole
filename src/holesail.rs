use std::fs;
use directories::ProjectDirs;
use std::process::Command;
use std::path::{Path, PathBuf};
use sysinfo::{System, ProcessesToUpdate};

fn get_holesail_url() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "https://github.com/holesail/holesail/releases/download/2.4.1/win32-x64.zip",
        ("windows", "aarch64") => "https://github.com/holesail/holesail/releases/download/2.4.1/win32-arm64.zip",
        ("linux", "x86_64")   => "https://github.com/holesail/holesail/releases/download/2.4.1/linux-x64.zip",
        ("linux", "aarch64")  => "https://github.com/holesail/holesail/releases/download/2.4.1/linux-arm64.zip",    
        _ => panic!("This app is not supported on your os"),
    }
}

pub fn download() -> Result<PathBuf, String> {
    let proj = ProjectDirs::from("com", "holesail", "porthole").ok_or("No data directory found")?;
    let target_dir = proj.data_dir().to_path_buf();
    let bin_name = if cfg!(windows) { "holesail.exe" } else { "holesail" };
    let final_path = target_dir.join(bin_name);
    println!(""); // REMOVE
    println!("Checking For Holesail"); // REMOVE
    // Checks if already installed
    if final_path.exists() {
        println!("Already Exsits"); // REMOVE
        return Ok(final_path);
    }
    println!("Does not exsist, Downloading"); // REMOVE
    // Download Holesail
    let bytes = reqwest::blocking::get(get_holesail_url())
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?;

    println!("Downloaded! Extracting..."); // REMOVE
    // Unzip it
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    zip_extract::extract(std::io::Cursor::new(bytes), &target_dir, true)
        .map_err(|e| format!("Extraction failure: {}", e))?;

    println!("Completed"); // REMOVE
    Ok(final_path)
}

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

// Connects to holesail in a background service
pub fn connect(holesail_path: &Path, passphrase: &str) {
    println!(""); // REMOVE
    println!("Finding Holesail"); // REMOVE
    if !holesail_path.exists() {
        panic!("Could not find holesail!");
    }

    println!("Creating command"); // REMOVE
    let mut command = Command::new(holesail_path);
    command.arg(passphrase);

    // Windows Only: Prevents windows from spawning a window
    #[cfg(target_os = "windows")]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    // Execute Command
    println!("Executing Command"); // REMOVE
    let _ = command.spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to launch holesail: {}", e));

}

/// Terminates any detached Holesail daemon running in the background.
pub fn stop() {
    println!("Stopping all background Holesail instances...");

    if cfg!(windows) {
        // Windows forcefully terminates holesail.exe by name
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "holesail.exe"])
            .output();
    } else {
        // Linux/macOS terminates holesail by name
        let _ = Command::new("pkill")
            .arg("-f")
            .arg("holesail")
            .output();
    }
    
    println!("Stop command completed.");
}

pub fn is_active() -> bool {
    let mut sys = System::new();
    
    sys.refresh_processes(ProcessesToUpdate::All, true);

    // Scan through everything
    for (_pid, process) in sys.processes() {
        let process_name = process.name().to_string_lossy().to_lowercase();
        if process_name.contains("holesail") {
            println!("Found active Holesail instance: [PID: {}]", _pid);
            return true;
        }
    }

    false
}   