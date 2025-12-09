// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// kernel_ui/src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    kernel_ui_lib::run()
}
