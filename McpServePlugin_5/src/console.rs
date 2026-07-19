use winapi::um::winuser::GetAsyncKeyState;
use windows::Win32::Foundation::BOOL;
pub use windows::Win32::System::Console::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PCWSTR;

//捕捉Ctrl+C事件
unsafe extern "system" fn console_ctrl_handler(ctrl_type: u32) -> BOOL {
    match ctrl_type {
        CTRL_C_EVENT => BOOL(1),
        //CTRL_CLOSE_EVENT => BOOL(1),没有效果
        _ => BOOL(0),
    }
}
pub fn clear_console() {
    use crossterm::{cursor, terminal, execute};
    use std::io::stdout;
    let _ = execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0));
}

pub fn console_win() {
    unsafe {
        // 1. 申请控制台窗口
        if AllocConsole().is_ok() {
            // 捕捉控制台关闭事件
            let _ = SetConsoleCtrlHandler(Some(console_ctrl_handler), true);
            // 设置控制台窗口标题
            let title: Vec<u16> = "Alias MCP Server\0".encode_utf16().collect();
            let _ = SetConsoleTitleW(PCWSTR(title.as_ptr()));

            print!(r#"==========================================
        Alias MCP Server Active           
==========================================

MCP Server Configuration:
{{
  "mcpServers": {{
    "alias-serve": {{
      "url": "http://127.0.0.1:9000/mcp"
    }}
  }}
}}

"#);

            let hwnd = GetConsoleWindow();

            if hwnd.0 != 0 {
                let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);

                // 移除工具窗口样式，使其显示任务栏按钮和最小化按钮
                let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                let new_style = ex_style & !(WS_EX_TOOLWINDOW.0 as i32);
                let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);

                // 添加窗口样式，确保有最小化和最大化按钮
                let style = GetWindowLongW(hwnd, GWL_STYLE);
                let new_style = style | WS_MINIMIZEBOX.0 as i32 | WS_MAXIMIZEBOX.0 as i32;
                let _ = SetWindowLongW(hwnd, GWL_STYLE, new_style);

                let h_menu = GetSystemMenu(hwnd, false);
                if !h_menu.is_invalid() {
                    // 禁用关闭按钮，防止 Alias 崩溃，但保留最小化和最大化按钮
                    let _ = DeleteMenu(h_menu, SC_CLOSE, MF_BYCOMMAND);
                    // 重新启用最小化和最大化按钮
                    let _ = EnableMenuItem(h_menu, SC_MINIMIZE, MF_ENABLED);
                    let _ = EnableMenuItem(h_menu, SC_MAXIMIZE, MF_ENABLED);
                }

                println!("==========================================");
            }
        }
    }
}

pub fn is_any_input_pressed() -> bool {
    // 检查VK_0到VK_Z范围内的任意键
    for vk in 0..256 {
        if unsafe { GetAsyncKeyState(vk) } < 0 {
            return true;
        }
    }
    false
}

// 鼠标按键虚拟键码常量
const VK_LBUTTON: i32 = 0x01; // 鼠标左键
const VK_RBUTTON: i32 = 0x02; // 鼠标右键
const VK_MBUTTON: i32 = 0x04; // 鼠标中键
//const VK_XBUTTON1: i32 = 0x05; // 鼠标侧键1
//const VK_XBUTTON2: i32 = 0x06; // 鼠标侧键2

/// 检测鼠标左键是否按下
pub fn is_mouse_left_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_LBUTTON) < 0 }
}

/// 检测鼠标右键是否按下
pub fn is_mouse_right_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_RBUTTON) < 0 }
}

/// 检测鼠标中键是否按下
pub fn is_mouse_middle_pressed() -> bool {
    unsafe { GetAsyncKeyState(VK_MBUTTON) < 0 }
}

/// 检测任意鼠标按键是否按下
pub fn is_mouse_pressed() -> bool {
    is_mouse_left_pressed() || is_mouse_right_pressed() || is_mouse_middle_pressed()
}
