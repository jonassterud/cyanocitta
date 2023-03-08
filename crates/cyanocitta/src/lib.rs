mod app_state;

use app_state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
