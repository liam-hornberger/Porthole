use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

pub fn main(code: String) {
    println!("Downloading");
    let _junkdata = download();
    println!("Done! (yippe!) now i can conenct to []");
}

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
    
    // 1. Fetch ZIP package directly into memory bytes
    let bytes = reqwest::blocking::get(get_holesail_url())
        .map_err(|e| e.to_string())?
        .bytes()
        .map_err(|e| e.to_string())?;

    // 2. Unpack everything inside the bytes array straight into target directory
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    zip_extract::extract(std::io::Cursor::new(bytes), &target_dir, true)
        .map_err(|e| format!("Extraction failure: {}", e))?;

    // 3. Return path to the extracted binary (zip-extract preserves executable flags automatically!)
    let bin_name = if cfg!(windows) { "holesail.exe" } else { "holesail" };
    Ok(target_dir.join(bin_name))
}