// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn check_connection(url: String) -> Result<String, String> {
    // Simple connection check
    match url.parse::<url::Url>() {
        Ok(_) => Ok(format!("Valid URL: {}", url)),
        Err(e) => Err(format!("Invalid URL: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![check_connection])
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_updater::UpdaterExt;
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Ok(update) = handle.updater().expect("updater unavailable").check().await {
                        if let Some(update) = update {
                            println!("Update available: {}", update.version);
                            // Auto-download and install
                            let _ = update.download_and_install(|_, _| {}, || {}).await;
                        } else {
                            println!("No updates available");
                        }
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
