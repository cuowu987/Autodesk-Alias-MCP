use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;

static GDI_CLASS_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct LayeredCanvas {
    hwnd: HWND,
    mem_dc: HDC,
    hbitmap: HBITMAP,
    bits_ptr: *mut u8,
    canvas_w: u32,
    canvas_h: u32,
    class_name: Vec<u16>,
}

unsafe impl Send for LayeredCanvas {}

impl LayeredCanvas {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        bg_r: u8,
        bg_g: u8,
        bg_b: u8,
        bg_a: u8,
    ) -> Result<Self> {
        unsafe {
            let h_instance: HINSTANCE = GetModuleHandleW(PCWSTR::null())?.into();

            let class_name = Self::generate_class_name();
            Self::register_window_class(h_instance, &class_name)?;

            let hwnd = Self::create_window(h_instance, x, y, w, h, &class_name)?;
            let (mem_dc, hbitmap, bits_ptr, canvas_w, canvas_h) = Self::create_canvas(w, h)?;

            let mut this = Self {
                hwnd,
                mem_dc,
                hbitmap,
                bits_ptr,
                canvas_w,
                canvas_h,
                class_name,
            };

            this.fill(bg_r, bg_g, bg_b, bg_a);
            this.set_alpha_for_colored(255);
            this.update_window()?;

            ShowWindow(hwnd, SW_SHOWNOACTIVATE);

            Ok(this)
        }
    }

    fn generate_class_name() -> Vec<u16> {
        let counter = GDI_CLASS_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("LayeredCanvasGdi_Class_{}", counter);
        name.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn register_window_class(h_instance: HINSTANCE, class_name: &[u16]) -> Result<()> {
        unsafe {
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),
                hInstance: h_instance,
                hCursor: LoadCursorW(None, IDC_ARROW)?,
                hbrBackground: HBRUSH(0),
                lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
                ..Default::default()
            };
            if RegisterClassExW(&wc) == 0 {
                return Err(Error::from_win32());
            }
            Ok(())
        }
    }

    fn create_window(h_instance: HINSTANCE, x: i32, y: i32, w: i32, h: i32, class_name: &[u16]) -> Result<HWND> {
        unsafe {
            let ex_style = WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOOLWINDOW;
            let hwnd = CreateWindowExW(
                ex_style,
                PCWSTR::from_raw(class_name.as_ptr()),
                w!(""),
                WS_POPUP,
                x,
                y,
                w,
                h,
                None,
                None,
                h_instance,
                None,
            );
            if hwnd.0 == 0 {
                return Err(Error::from_win32());
            }
            set_window_above_alias(hwnd);
            Ok(hwnd)
        }
    }

    pub fn create_canvas(
        canvas_w: i32,
        canvas_h: i32,
    ) -> Result<(HDC, HBITMAP, *mut u8, u32, u32)> {
        unsafe {
            let screen_dc = GetDC(None);
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: canvas_w,
                    biHeight: canvas_h,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits_ptr: *mut u8 = std::ptr::null_mut();
            let hbitmap = CreateDIBSection(
                screen_dc,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits_ptr as *mut *mut u8 as *mut *mut _,
                None,
                0,
            )?;
            ReleaseDC(None, screen_dc);

            std::ptr::write_bytes(bits_ptr, 0, (canvas_w * canvas_h * 4) as usize);

            let mem_dc = CreateCompatibleDC(None);
            SelectObject(mem_dc, hbitmap);

            Ok((mem_dc, hbitmap, bits_ptr, canvas_w as u32, canvas_h as u32))
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
    pub fn mem_dc(&self) -> HDC {
        self.mem_dc
    }
    pub fn hbitmap(&self) -> HBITMAP {
        self.hbitmap
    }
    pub fn bits_ptr(&self) -> *mut u8 {
        self.bits_ptr
    }
    pub fn canvas_w(&self) -> u32 {
        self.canvas_w
    }
    pub fn canvas_h(&self) -> u32 {
        self.canvas_h
    }
    #[inline]
    pub fn canvas_size(&self) -> (u32, u32) {
        (self.canvas_w, self.canvas_h)
    }

    #[inline]
    pub fn flip_y(&self, user_y: i32) -> i32 {
        self.canvas_h as i32 - user_y
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        unsafe {
            let total = (self.canvas_w * self.canvas_h) as usize;
            let color: u32 = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            let color_ptr = &color as *const u32;
            std::ptr::copy_nonoverlapping(color_ptr, self.bits_ptr as *mut u32, total);
        }
    }

    pub fn clear(&mut self) {
        self.fill(0, 0, 0, 0);
    }

    pub fn set_alpha_for_colored(&mut self, alpha: u8) {
        unsafe {
            let total = (self.canvas_w * self.canvas_h) as usize;
            for i in 0..total {
                let idx = i * 4;
                let b = *self.bits_ptr.add(idx);
                let g = *self.bits_ptr.add(idx + 1);
                let r = *self.bits_ptr.add(idx + 2);
                let a = *self.bits_ptr.add(idx + 3);
                if (b != 0 || g != 0 || r != 0) && a == 0 {
                    *self.bits_ptr.add(idx + 3) = alpha;
                }
            }
        }
    }

    pub fn update_window(&self) -> Result<()> {
        unsafe {
            let screen_dc = GetDC(None);
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };
            let size = SIZE {
                cx: self.canvas_w as i32,
                cy: self.canvas_h as i32,
            };
            let pt_src = POINT { x: 0, y: 0 };
            UpdateLayeredWindow(
                self.hwnd,
                screen_dc,
                None,
                Some(&size),
                self.mem_dc,
                Some(&pt_src),
                rgb(0, 0, 0),
                Some(&blend),
                ULW_ALPHA,
            )?;
            ReleaseDC(None, screen_dc);
            Ok(())
        }
    }

    pub fn reposition_and_resize(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<()> {
        unsafe {
            DeleteDC(self.mem_dc);
            self.mem_dc = HDC(0);
            DeleteObject(self.hbitmap);
            self.hbitmap = HBITMAP(0);
            self.bits_ptr = std::ptr::null_mut();

            let (new_dc, new_bmp, new_bits, new_w, new_h) = Self::create_canvas(w, h)?;
            self.mem_dc = new_dc;
            self.hbitmap = new_bmp;
            self.bits_ptr = new_bits;
            self.canvas_w = new_w;
            self.canvas_h = new_h;

            let flags = SET_WINDOW_POS_FLAGS(SWP_NOZORDER.0 | SWP_NOACTIVATE.0);
            SetWindowPos(self.hwnd, HWND(0), x, y, w, h, flags)?;

            Ok(())
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) -> Result<()> {
        unsafe {
            let flags = SET_WINDOW_POS_FLAGS(SWP_NOZORDER.0 | SWP_NOACTIVATE.0 | SWP_NOSIZE.0);
            SetWindowPos(self.hwnd, HWND(0), x, y, 0, 0, flags)?;
            Ok(())
        }
    }

    pub fn resize(&mut self, w: i32, h: i32) -> Result<()> {
        unsafe {
            let mut r = RECT::default();
            GetWindowRect(self.hwnd, &mut r)?;
            self.reposition_and_resize(r.left, r.top, w, h)
        }
    }

    pub fn process_iter(&self) -> bool {
        unsafe {
            if !IsWindow(self.hwnd).as_bool() {
                return false;
            }
            let mut msg: MSG = Default::default();
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    return false;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            true
        }
    }

    pub fn process(&self) {
        unsafe {
            let mut msg: MSG = Default::default();
            loop {
                if IsWindow(self.hwnd).as_bool() == false {
                    break;
                }
                if GetMessageW(&mut msg, None, 0, 0).as_bool() == false {
                    break;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
        match msg {
            WM_NCHITTEST => LRESULT(HTTRANSPARENT as isize),
            _ => DefWindowProcW(hwnd, msg, wp, lp),
        }
    }
}

impl Drop for LayeredCanvas {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.mem_dc);
            self.mem_dc = HDC(0);
            DeleteObject(self.hbitmap);
            self.hbitmap = HBITMAP(0);
            self.bits_ptr = std::ptr::null_mut();
            let _ = DestroyWindow(self.hwnd);
            self.hwnd = HWND(0);
            let h_instance = GetModuleHandleW(PCWSTR::null()).ok();
            if let Some(hinst) = h_instance {
                let _ = UnregisterClassW(PCWSTR::from_raw(self.class_name.as_ptr()), hinst);
            }
        }
    }
}
