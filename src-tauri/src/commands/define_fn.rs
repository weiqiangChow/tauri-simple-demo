use tauri::WebviewWindow;

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
pub fn get_macos_theme() -> bool {
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
pub fn get_macos_theme() -> bool {
    false // 非 macOS 平台默认返回浅色模式
}

// 初始化窗口背景色（根据系统主题）
pub fn init_window_background(window: &WebviewWindow) {
    use super::handle_window::set_macos_window_background;
    let is_dark = get_macos_theme();
    let _ = set_macos_window_background(window, is_dark);
}
