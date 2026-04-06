// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod ledger;
mod parser;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 设置窗口属性
            let window = app.get_webview_window("main").unwrap();

            // 设置为悬浮窗
            #[cfg(target_os = "windows")]
            {
                use tauri::utils::WindowEffect;
                window.set_decorations(false).ok();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::parse_and_record,
            commands::get_stats,
            commands::get_config,
            commands::save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}