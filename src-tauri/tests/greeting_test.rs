use app::create_app;
use serde_json::json;
use tauri::Manager;

use tauri::test::{assert_ipc_response, mock_builder};


#[test]
fn greet_empty_name_correct() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    assert_ipc_response(
        &window,
        tauri::InvokePayload {
            cmd: "greet".to_string(),
            tauri_module: None,
            callback: tauri::api::ipc::CallbackFn(0),
            error: tauri::api::ipc::CallbackFn(1),
            inner: json!({
                "name" : ""
            }),
        },
        Ok("please input your name"),
    );
}

#[test]
fn greet_single_space_name_correct() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    assert_ipc_response(
        &window,
        tauri::InvokePayload {
            cmd: "greet".to_string(),
            tauri_module: None,
            callback: tauri::api::ipc::CallbackFn(0),
            error: tauri::api::ipc::CallbackFn(1),
            inner: json!({
                "name" : " "
            }),
        },
        Ok("please input your name"),
    );
}

#[test]
fn greet_spaces_name_correct() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    assert_ipc_response(
        &window,
        tauri::InvokePayload {
            cmd: "greet".to_string(),
            tauri_module: None,
            callback: tauri::api::ipc::CallbackFn(0),
            error: tauri::api::ipc::CallbackFn(1),
            inner: json!({
                "name" : "   "
            }),
        },
        Ok("please input your name"),
    );
}

#[test]
fn greet_name_correct() {
    let app = create_app(mock_builder());
    let window = app.get_window("main").unwrap();

    assert_ipc_response(
        &window,
        tauri::InvokePayload {
            cmd: "greet".to_string(),
            tauri_module: None,
            callback: tauri::api::ipc::CallbackFn(0),
            error: tauri::api::ipc::CallbackFn(1),
            inner: json!({
                "name" : "danny"
            }),
        },
        Ok("Hello, danny!"),
    );
}

