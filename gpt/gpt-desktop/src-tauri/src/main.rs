// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpt::{Config, Message, OpenAiClient};
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
/*
   通过窗口控制不同的工作空间？

   剪切板：https://github.com/CrossCopy/tauri-plugin-clipboard
    流程图：https://github.com/wbkd/react-flow
*/

/*
支持block和stream两种和chatGPT交互的方式。
/ 触发提示，选择对应的prompt
    prompt内置一些
    支持指定接口获取prompt
    prompt支持占位符，用户输入对应的值，生成完整的prompt
 */

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// TODO: 错误处理怎么用anyhow？
#[tauri::command]
fn chat_with_messages(messages: Vec<Message>) -> Result<String, String> {
    dbg!(&messages);
    // format!("Hello, {}! You've been greeted from Rust!", name)
    let client = OpenAiClient::new(Config::default()).unwrap();

    client
        .send_message_block(&messages)
        .map_err(|err| err.to_string())
}

fn main() {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            close_splashscreen,
            chat_with_messages
        ])
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
