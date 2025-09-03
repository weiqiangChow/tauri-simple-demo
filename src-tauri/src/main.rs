// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// 防止在Windows上发布额外的控制台窗口，不要删除！！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri_lib::run()
}
