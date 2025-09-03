mod handle_window;
mod define_fn;

use handle_window::set_macos_window_background;
use define_fn::{get_platform, get_macos_theme};
use tauri::{AppHandle, Manager};

// 重新导出 init_window_background 函数供 lib.rs 使用
pub use define_fn::init_window_background;

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
pub fn set_window_theme(app_handle: AppHandle, is_dark: bool) -> Result<String, String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        match set_macos_window_background(&window, is_dark) {
            Ok(()) => Ok("设置主题成功".to_string()),
            Err(e) => Err(e)
        }
    } else {
        Err("无法找到主窗口".to_string())
    }
}

/**
 * 获取系统主题的命令
 * 对于 macOS，返回 true 表示深色模式，false 表示浅色模式
 */
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
        window.open_devtools();
        Ok(())
    } else {
        Err("无法找到主窗口".to_string())
    }
}

// 关闭 DevTools 的命令
#[tauri::command]
pub fn close_devtools(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.close_devtools();
        Ok(())
    } else {
        Err("无法找到主窗口".to_string())
    }
}
