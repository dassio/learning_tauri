use tauri::{AppHandle, Manager};

pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
    .setup(|app| {
        app.listen_global("new_event",  move |_event|{
            println!("event handler called")
        });
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

#[tauri::command]
fn greet<R: tauri::Runtime>(name: &str, app_handle: AppHandle<R>) -> String {
    app_handle.emit_all("new_event", {}).unwrap();
    
    if name.len() == 0 || name.trim().len() == 0{
        return "please input your name".to_string();
    }

   format!("Hello, {}!", name)
}