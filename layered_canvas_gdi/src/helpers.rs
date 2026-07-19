use super::*;

pub fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF(((b as u32) << 16) | ((g as u32) << 8) | (r as u32))
}

fn get_window_title(hwnd: HWND) -> String {
    unsafe {
        let mut len = GetWindowTextLengthW(hwnd);
        if len == 0 {
            return String::new();
        }
        len += 1;
        let mut buf: Vec<u16> = vec![0; len as usize];
        let _ = GetWindowTextW(hwnd, buf.as_mut_slice());
        String::from_utf16_lossy(&buf).trim().to_string()
    }
}

fn get_window_area(hwnd: HWND) -> i32 {
    unsafe {
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).is_err() {
            return 0;
        }
        (rect.right - rect.left) * (rect.bottom - rect.top)
    }
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data: *mut (Option<HWND>, i32, bool) = lparam.0 as *mut (Option<HWND>, i32, bool);
    unsafe {
        if GetWindowLongW(hwnd, GWL_STYLE) & WS_VISIBLE.0 as i32 != 0 {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == GetCurrentProcessId() {
                let _title = get_window_title(hwnd);
                let area = get_window_area(hwnd);
                let is_tool_window =
                    (GetWindowLongW(hwnd, GWL_EXSTYLE) & WS_EX_TOOLWINDOW.0 as i32) != 0;

                if !is_tool_window {
                    if !(*data).2 || area > (*data).1 {
                        (*data).0 = Some(hwnd);
                        (*data).1 = area;
                        (*data).2 = true;
                    }
                } else if !(*data).2 && area > (*data).1 {
                    (*data).0 = Some(hwnd);
                    (*data).1 = area;
                }
            }
        }
    }
    BOOL(1)
}

fn find_alias_hwnd() -> Option<HWND> {
    unsafe {
        let mut data = (None, 0, false);
        let _ = EnumWindows(
            Some(enum_windows_callback),
            LPARAM(&mut data as *mut (Option<HWND>, i32, bool) as isize),
        );
        data.0
    }
}

pub fn set_window_above_alias(hwnd: HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let new_style = ex_style & !(WS_EX_TOPMOST.0 as i32);
        let _ = SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
        let _ = SetWindowPos(
            hwnd,
            HWND_NOTOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        );

        if let Some(alias_hwnd) = find_alias_hwnd() {
            let _ = SetWindowLongPtrW(
                hwnd,
                WINDOW_LONG_PTR_INDEX(GWLP_HWNDPARENT.0 as i32),
                alias_hwnd.0 as isize,
            );
            let _ = SetWindowPos(
                hwnd,
                HWND_TOP,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
        } else {
            let _ = SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
        }
    }
}