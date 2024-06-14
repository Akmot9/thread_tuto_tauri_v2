// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod state;
use state::ThreadManager;

mod commands;
use commands::{add_thread, stop_thread, get_thread_ids};

use tauri::Manager;


fn main() {

    tauri::Builder::default()
        .setup(|app| {
          app.manage(ThreadManager::new(app));
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_thread, 
            stop_thread, 
            get_thread_ids
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}