use tauri::{AppHandle, Manager};

pub fn get_platform() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        "linux" => "linux",
        _ => "unknown",
    }
}

// 获取 macOS 系统深浅色模式
#[cfg(target_os = "macos")]
fn get_macos_theme() -> bool {
    use std::process::Command;
    
    let output = Command::new("defaults")
        .args(&["read", "-g", "AppleInterfaceStyle"])
        .output();
    
    match output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            result.trim() == "Dark"
        }
        Err(_) => false, // 如果命令失败，默认为浅色模式
    }
}

// 非 macOS 平台的空实现
#[cfg(not(target_os = "macos"))]
fn get_macos_theme() -> bool {
    false // 非 macOS 平台默认返回浅色模式
}

// 设置 macOS 窗口背景色
#[cfg(target_os = "macos")]
fn set_macos_window_background(window: &tauri::WebviewWindow, is_dark: bool) {
    use cocoa::appkit::{NSColor, NSWindow};
    use cocoa::base::{id, nil};

    if let Ok(ns_window) = window.ns_window() {
        let ns_window = ns_window as id;
        unsafe {
            let bg_color = if is_dark {
                // 深色模式背景色 (#242424)
                NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    36.0 / 255.0,
                    36.0 / 255.0,
                    36.0 / 255.0,
                    1.0,
                )
            } else {
                // 浅色模式背景色 (#ffffff)
                NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    255.0 / 255.0,
                    255.0 / 255.0,
                    255.0 / 255.0,
                    1.0,
                )
            };
            ns_window.setBackgroundColor_(bg_color);
        }
    }
}

// 简化的非 macOS 实现
#[cfg(not(target_os = "macos"))]
fn set_macos_window_background(_window: &tauri::WebviewWindow, _is_dark: bool) {
    // 非 macOS 平台不需要特殊处理
}

// 初始化窗口背景色（根据系统主题）
pub fn init_window_background(window: &tauri::WebviewWindow) {
    let is_dark = get_macos_theme();
    set_macos_window_background(window, is_dark);
}

#[tauri::command]
pub fn platform_cmd() -> String {
    get_platform().to_string()
}

#[tauri::command]
pub fn say_hello(user_name: &str) -> String {
    format!("Hello, {}! Welcome to Rust Tauri for desktop", user_name)
}

// 切换窗口背景色的命令
#[tauri::command]
pub fn set_window_theme(app_handle: AppHandle, is_dark: bool) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        set_macos_window_background(&window, is_dark);
        Ok(())
    } else {
        Err("无法找到主窗口".to_string())
    }
}

// 获取系统主题的命令
#[tauri::command]
pub fn get_system_theme() -> bool {
    #[cfg(target_os = "macos")]
    {
        get_macos_theme()
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        false // 非 macOS 平台默认返回浅色模式
    }
}

// 开启 DevTools 的命令
#[tauri::command]
pub fn open_devtools(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        #[cfg(debug_assertions)]
        {
            window.open_devtools();
            Ok(())
        }
        #[cfg(not(debug_assertions))]
        {
            Err("DevTools 仅在开发模式下可用".to_string())
        }
    } else {
        Err("无法找到主窗口".to_string())
    }
}

// 关闭 DevTools 的命令
#[tauri::command]
pub fn close_devtools(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        #[cfg(debug_assertions)]
        {
            window.close_devtools();
            Ok(())
        }
        #[cfg(not(debug_assertions))]
        {
            Err("DevTools 仅在开发模式下可用".to_string())
        }
    } else {
        Err("无法找到主窗口".to_string())
    }
}