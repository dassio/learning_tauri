#[tauri::command]
pub fn greet(name: &str) -> String {
   app::greet(name)
}