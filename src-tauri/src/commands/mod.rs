use tauri::AppHandle;
use tauri::State;
use tauri::command;

use crate::state::thread_manager::ThreadManager;


#[command]
pub async fn add_thread(state: State<'_, ThreadManager>, rate: u32, app: AppHandle) -> Result<u32, String> {
    Ok(state.add_thread(rate, app))
}

#[command]
pub async fn stop_thread(state: State<'_, ThreadManager>, id: u32) -> Result<bool, String> {
    println!("get order Thread {} to be stopped", id);
    let res = state.stop_thread(id);
    println!("Thread status: {} ", res);
    Ok(res)
}

#[command]
pub async fn get_thread_ids(state: State<'_, ThreadManager>) -> Result<Vec<u32>, String> {
    Ok(state.get_thread_ids())
}
