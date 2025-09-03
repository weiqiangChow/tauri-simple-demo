#![allow(unexpected_cfgs)]

// 设置 macOS 窗口背景色和标题栏
#[cfg(target_os = "macos")]
pub fn set_macos_window_background(window: &tauri::WebviewWindow, is_dark: bool) -> Result<(), String> {
    use cocoa::appkit::{NSColor, NSWindow};
    use cocoa::base::{id, nil};
    use cocoa::foundation::NSString;

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

            // 设置窗口背景色
            ns_window.setBackgroundColor_(bg_color);

            // 使用更好的标题栏设置方式，保持拖拽功能
            use objc::runtime::Class;
            use objc::{msg_send, sel, sel_impl};

            // 设置窗口外观以影响标题栏颜色
            let appearance_class = Class::get("NSAppearance").unwrap();
            let appearance_name = if is_dark {
                NSString::alloc(nil).init_str("NSAppearanceNameDarkAqua")
            } else {
                NSString::alloc(nil).init_str("NSAppearanceNameAqua")
            };
            let appearance: id = msg_send![appearance_class, appearanceNamed: appearance_name];
            let _: () = msg_send![ns_window, setAppearance: appearance];
        }
        Ok(())
    } else {
        Err("无法获取 macOS 窗口".to_string())
    }
}

// 简化的非 macOS 实现
#[cfg(not(target_os = "macos"))]
pub fn set_macos_window_background(
    _window: &tauri::WebviewWindow,
    _is_dark: bool,
) -> Result<(), String> {
    // 非 macOS 平台不支持切换主题色
    Err("非 macOS 平台暂不支持切换主题色".to_string())
}
