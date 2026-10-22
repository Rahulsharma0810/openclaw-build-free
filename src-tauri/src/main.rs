// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn check_connection(url: String) -> Result<String, String> {
    // Simple validation check
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(format!("Valid URL: {}", url))
    } else {
        Err("URL must start with http:// or https://".to_string())
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
                    match handle.updater() {
                        Ok(updater) => {
                            if let Ok(Some(update)) = updater.check().await {
                                println!("Update available: {}", update.version);
                                // Auto-download and install
                                let _ = update.download_and_install(|_, _| {}, || {}).await;
                            } else {
                                println!("No updates available");
                            }
                        }
                        Err(e) => {
                            println!("Updater error: {}", e);
                        }
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
