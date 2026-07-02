use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;


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