mod commands;
use commands::{platform_cmd, say_hello, set_window_theme, get_system_theme, init_window_background, open_devtools, close_devtools};
use tauri::tray::TrayIconBuilder;
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Tauri Theme Demo")
        .inner_size(800.0, 600.0)
        .devtools(true); // 始终启用 DevTools

    let window = win_builder
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // 初始化窗口背景色（根据系统主题）
    init_window_background(&window);

    // 创建系统托盘图标 - 跨平台兼容
    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("Tauri Simple Demo") // 添加工具提示
        .build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup_app)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![platform_cmd, say_hello, set_window_theme, get_system_theme, open_devtools, close_devtools])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
