#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::env;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Lokale root map van elysia.exe
    let root = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let core_path = root.join("core").join("core.exe");
    let ui_path   = root.join("ui").join("ui.exe");

    // Start kernel
    let _ = Command::new(&core_path)
        .current_dir(&root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Kernel kon niet starten");

    sleep(Duration::from_millis(300));

    // Start UI
    let _ = Command::new(&ui_path)
        .current_dir(&root)
        .spawn()
        .expect("UI kon niet starten");
}
