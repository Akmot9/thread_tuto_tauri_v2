// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod state;
use state::thread_manager::ThreadManager;

mod commands;
use commands::{add_thread, stop_thread, get_thread_ids};


fn main() {
    tauri::Builder::default()
        .manage(ThreadManager::new())
        .invoke_handler(tauri::generate_handler![
            add_thread, 
            stop_thread, 
            get_thread_ids
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
