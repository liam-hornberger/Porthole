// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use directories::ProjectDirs;
use futures_util::StreamExt;

slint::include_modules!();

// Gets the url for the Holesail binarys download
fn get_url() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "https://github.com/holesail/holesail/releases/download/2.4.1/win32-x64.zip",
        ("windows", "aarch64") => "https://github.com/holesail/holesail/releases/download/2.4.1/win32-arm64.zip",
        ("linux", "x86_64") => "https://github.com/holesail/holesail/releases/download/2.4.1/linux-x64.zip",
        ("linux", "aarch64") => "https://github.com/holesail/holesail/releases/download/2.4.1/linux-arm64.zip",
        _ => panic!("This Platform Is Not Supported"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();
    let ui_weak = ui.as_weak();

    // Get Holesail binarys path
    let project_dir = ProjectDirs::from("com", "holesail", "porthole").expect("Failed to locate local application path.");
    let target_dir = project_dir.data_dir().to_path_buf();

    let bin_name = if  cfg!(windows) { "holesail.exe" } else { "holesail" };
    let final_bin_path = target_dir.join(bin_name);

// Spawn downloader thread
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        
        rt.block_on(async {
            // Check if binary's already exist
            if final_bin_path.exists() {
                let ui_clone = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_clone.upgrade() {
                        ui.set_activepage(1);
                    }
                }).ok();
                return;
            }

            // Install Holesail binary's

            fs::create_dir_all(&target_dir).ok();
            let client = reqwest::Client::new();
            let url = get_url();

            let response = match client.get(url).send().await {
                Ok(res) => res,
                Err(err) => {
                    let err_string = err.to_string();
                    let ui_clone = ui_weak.clone();
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_clone.upgrade() {
                            ui.set_loadingtext(format!("Network Error: {} (Try restarting the app)", err_string).into());
                        }
                    }).ok();
                    return;
                }
            };

            let total_size = response.content_length().unwrap_or(1);
            let mut file = File::create(&final_bin_path).expect("Failed to get file destination handler.");
            let mut downloaded: u64 = 0;
            let mut stream = response.bytes_stream();

            while let Some(chunk_result) = stream.next().await {
                if let Ok(chunk) = chunk_result {
                    file.write_all(&chunk).ok();
                    downloaded += chunk.len() as u64;

                    let percentage = (downloaded as f32 / total_size as f32) * 100.0;
                    
                    let ui_clone = ui_weak.clone();
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_clone.upgrade() {
                            ui.set_loadingbar(percentage);
                            ui.set_loadingtext(format!("Downloading Core Components: {:.0}%", percentage).into());
                        }
                    }).ok();
                }
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = fs::metadata(&final_bin_path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&final_bin_path, perms).ok();
                }
            }

            let ui_clone = ui_weak.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_clone.upgrade() {
                    ui.set_activepage(1);
                }
            }).ok();
        });
    });


    ui.run().unwrap();
    
    Ok(())
}
