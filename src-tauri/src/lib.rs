
pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[tauri::command]
fn greet(name: &str) -> String {
    if name.len() == 0 || name.trim().len() == 0{
        return "please input your name".to_string();
    }
   format!("Hello, {}!", name)
}