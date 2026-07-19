use base_geometry_lib::{RU_2dPoint, RU_2iPoint, RU_Color};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;

const FONT_BASE: u32 = 1000;
static GL_CLASS_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct GlCanvas {
    hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
    width: i32,
    height: i32,
    camera: GlCamera,
    line_width: f32,
    mem_dc: HDC,
    hbitmap: HBITMAP,
    bits_ptr: *mut u8,
    class_name: Vec<u16>,
}

unsafe impl Send for GlCanvas {}

impl GlCanvas {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str, camera: GlCamera) -> Result<Self> {
        unsafe {
            let h_instance: HINSTANCE = GetModuleHandleW(PCWSTR::null())?.into();

            let class_name = Self::generate_class_name();
            Self::register_window_class(h_instance, &class_name)?;

            let hwnd = Self::create_window(h_instance, x, y, w, h, title, &class_name)?;

            let hdc = GetDC(hwnd);
            if hdc == HDC(0) {
                return Err(Error::from_win32());
            }

            Self::setup_pixel_format(hdc)?;

            let hglrc = wglCreateContext(hdc)?;
            if hglrc == HGLRC(0) {
                ReleaseDC(hwnd, hdc);
                return Err(Error::from_win32());
            }

            wglMakeCurrent(hdc, hglrc)
                .ok()
                .ok_or_else(|| Error::from_win32())?;

            Self::generate_font_display_list(hdc);

            let (mem_dc, hbitmap, bits_ptr) = Self::create_dib_section(w, h)?;

            let this = Self {
                hwnd,
                hdc,
                hglrc,
                width: w,
                height: h,
                camera,
                line_width: 1.0,
                mem_dc,
                hbitmap,
                bits_ptr,
                class_name,
            };

            ShowWindow(hwnd, SW_SHOW);
            UpdateWindow(hwnd);

            this.clear();
            this.update();

            Ok(this)
        }
    }

    fn generate_class_name() -> Vec<u16> {
        let counter = GL_CLASS_COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("GlCanvas_Class_{}", counter);
        name.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn register_window_class(h_instance: HINSTANCE, class_name: &[u16]) -> Result<()> {
        unsafe {
            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
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

    fn create_window(
        h_instance: HINSTANCE,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        title: &str,
        class_name: &[u16],
    ) -> Result<HWND> {
        unsafe {
            let hwnd = CreateWindowExW(
                WS_EX_LAYERED,
                PCWSTR::from_raw(class_name.as_ptr()),
                PCWSTR::from_raw(
                    title
                        .encode_utf16()
                        .chain(std::iter::once(0))
                        .collect::<Vec<_>>()
                        .as_ptr(),
                ),
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
            if hwnd == HWND(0) {
                return Err(Error::from_win32());
            }

            set_window_above_alias(hwnd);

            Ok(hwnd)
        }
    }

    fn create_dib_section(w: i32, h: i32) -> Result<(HDC, HBITMAP, *mut u8)> {
        unsafe {
            let screen_dc = GetDC(None);
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w,
                    biHeight: h,
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

            std::ptr::write_bytes(bits_ptr, 0, (w * h * 4) as usize);

            let mem_dc = CreateCompatibleDC(None);
            SelectObject(mem_dc, hbitmap);

            Ok((mem_dc, hbitmap, bits_ptr))
        }
    }

    fn setup_pixel_format(hdc: HDC) -> Result<()> {
        unsafe {
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_FLAGS((PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER).0),
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 8,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 0,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE.0 as u8,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let pixfmt = ChoosePixelFormat(hdc, &pfd);
            if pixfmt == 0 {
                return Err(Error::from_win32());
            }

            SetPixelFormat(hdc, pixfmt, &pfd)
                .ok()
                .ok_or_else(|| Error::from_win32())
        }
    }

    fn generate_font_display_list(hdc: HDC) {
        unsafe {
            let stock_gdi = GetStockObject(DEFAULT_GUI_FONT);
            let prev = SelectObject(hdc, stock_gdi);

            while glGetError() != GL_NO_ERROR {}

            let _ = wglUseFontBitmapsA(hdc, 0, 255, FONT_BASE);

            if prev != HGDIOBJ(0) {
                let _ = SelectObject(hdc, prev);
            }
        }
    }

    pub fn hwnd(&self) -> HWND {
        self.hwnd
    }
    pub fn hdc(&self) -> HDC {
        self.hdc
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn read_pixels_rgba(&self) -> Vec<u8> {
        unsafe {
            let (w, h) = self.get_window_size();
            let size = (w * h * 4) as usize;
            let mut rgba = vec![0u8; size];
            
            let stride = (w * 4) as usize;
            
            for row in 0..h as usize {
                let src_row = (h - 1 - row as i32) as usize;
                let src_base = src_row * stride;
                let dst_base = row * stride;
                
                std::ptr::copy_nonoverlapping(
                    self.bits_ptr.add(src_base),
                    rgba.as_mut_ptr().add(dst_base),
                    stride,
                );
            }
            
            for i in (0..size).step_by(4) {
                rgba.swap(i, i + 2);
            }
            
            rgba
        }
    }
    pub fn camera(&self) -> GlCamera {
        self.camera
    }
    pub fn make_current(&self) -> bool {
        unsafe {
            wglMakeCurrent(self.hdc, self.hglrc).is_ok()
        }
    }

    pub fn with_context<F, T>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&Self) -> T,
    {
        unsafe {
            let prev_dc = wglGetCurrentDC();
            let prev_ctx = wglGetCurrentContext();

            if wglMakeCurrent(self.hdc, self.hglrc).is_err() {
                return None;
            }

            let result = f(self);

            let _ = wglMakeCurrent(prev_dc, prev_ctx);

            Some(result)
        }
    }

    pub fn set_camera(&mut self, camera: GlCamera) {
        self.camera = camera;
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width;
    }

    fn get_window_size(&self) -> (i32, i32) {
        unsafe {
            let mut r = RECT::default();
            if GetClientRect(self.hwnd, &mut r).is_ok() {
                ((r.right - r.left).max(1), (r.bottom - r.top).max(1))
            } else {
                (self.width.max(1), self.height.max(1))
            }
        }
    }

    pub fn clear(&self) {
        unsafe {
            let (w, h) = self.get_window_size();
            glViewport(0, 0, w, h);
            glClearColor(0.0, 0.0, 0.0, 0.0);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }

    fn setup_gl_state(line_width: f32) {
        unsafe {
            glEnable(GL_DEPTH_TEST);
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            glEnable(GL_LINE_SMOOTH);
            glEnable(GL_POINT_SMOOTH);
            glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);
            glHint(GL_POINT_SMOOTH_HINT, GL_NICEST);
            glLineWidth(line_width);
            glPointSize(line_width);
        }
    }

    fn cleanup_gl_state() {
        unsafe {
            glDisable(GL_BLEND);
            glDisable(GL_DEPTH_TEST);
        }
    }

    pub fn draw_3d<F>(&self, mode: u32, width: Option<f32>, draw_fn: F)
    where
        F: FnOnce(&Self),
    {
        match self.camera.camera_type {
            CameraType::Perspective => self.draw_perspective(mode, width, draw_fn),
            CameraType::Orthographic => self.draw_orthographic(mode, width, draw_fn),
        }
    }

    fn draw_perspective<F>(&self, mode: u32, width: Option<f32>, draw_fn: F)
    where
        F: FnOnce(&Self),
    {
        unsafe {
            let (w, h) = self.get_window_size();
            glViewport(0, 0, w, h);

            let line_width = width.unwrap_or(self.line_width);
            Self::setup_gl_state(line_width);

            let window_aspect = (w as f64) / (h as f64);
            let film_back_aspect = self.camera.film_back_w / self.camera.film_back_h;
            let fov_horizontal = self.camera.fov.to_radians();

            let fov_vertical = match self.camera.fit_type {
                CameraFitType::HorizontalFit => {
                    2.0 * ((fov_horizontal / 2.0).tan() / window_aspect).atan()
                }
                CameraFitType::VerticalFit => {
                    2.0 * ((fov_horizontal / 2.0).tan() / film_back_aspect).atan()
                }
                CameraFitType::FillFit => {
                    let aspect = window_aspect.max(film_back_aspect);
                    2.0 * ((fov_horizontal / 2.0).tan() / aspect).atan()
                }
            };

            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            gluPerspective(
                fov_vertical.to_degrees(),
                window_aspect,
                self.camera.near,
                self.camera.far,
            );

            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            gluLookAt(
                self.camera.eye.x,
                self.camera.eye.y,
                self.camera.eye.z,
                self.camera.center.x,
                self.camera.center.y,
                self.camera.center.z,
                self.camera.up.x,
                self.camera.up.y,
                self.camera.up.z,
            );

            glBegin(mode);
            draw_fn(self);
            glEnd();

            Self::cleanup_gl_state();
        }
    }

    fn draw_orthographic<F>(&self, mode: u32, width: Option<f32>, draw_fn: F)
    where
        F: FnOnce(&Self),
    {
        unsafe {
            let (w, h) = self.get_window_size();
            glViewport(0, 0, w, h);

            let line_width = width.unwrap_or(self.line_width);
            Self::setup_gl_state(line_width);

            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glOrtho(-1.0, 1.0, -1.0, 1.0, -10000.0, 10000.0);

            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();

            glBegin(mode);
            draw_fn(self);
            glEnd();

            Self::cleanup_gl_state();
        }
    }

    pub fn add_vertex3d(&self, pt: base_geometry_lib::RU_3dPoint, color: (f32, f32, f32, f32)) {
        unsafe {
            glColor4f(color.0, color.1, color.2, color.3);
            match self.camera.camera_type {
                CameraType::Perspective => {
                    glVertex3f(pt.x as f32, pt.y as f32, pt.z as f32);
                }
                CameraType::Orthographic => {
                    let point = self.world_to_view_plane(pt);
                    glVertex2f(point.x as f32, point.y as f32);
                }
            }
        }
    }

    pub fn add_vertex2i(&self, point: RU_2iPoint, color: RU_Color) {
        let point = (point.x, self.height - point.y);
        unsafe {
            glColor4f(color.r, color.g, color.b, color.a);
            glVertex2i(point.0, point.1);
        }
    }

    pub fn world_to_screen(&self, world_pt: base_geometry_lib::RU_3dPoint) -> RU_2iPoint {
        let view_plane_pt = self.world_to_view_plane(world_pt);
        let w = self.width();
        let h = self.height();

        let x = ((view_plane_pt.x + 1.0) * w as f64 / 2.0) as i32;
        let y = ((1.0 - view_plane_pt.y) * h as f64 / 2.0) as i32;

        RU_2iPoint::new(x, self.height - y)
    }

    pub fn world_to_view_plane(&self, world_pt: base_geometry_lib::RU_3dPoint) -> RU_2dPoint {
        if self.camera.camera_type == CameraType::Perspective {
            let dir = world_pt - self.camera.eye;
            let view_dir = self.camera.center - self.camera.eye;
            let view_dir_len = view_dir.Length();
            
            if view_dir_len < 1e-10 {
                return RU_2dPoint::new(0.0, 0.0);
            }
            
            let view_dir_norm = view_dir / view_dir_len;
            let dot = dir * view_dir_norm;
            
            if dot < 1e-10 {
                return RU_2dPoint::new(1000.0, 1000.0);
            }
            
            let t = self.camera.near / dot;
            let near_pt = self.camera.eye + dir * t;
            
            let near_center = self.camera.eye + view_dir_norm * self.camera.near;
            let vec_to_pt = near_pt - near_center;
            
            let right = (self.camera.up ^ view_dir_norm).UnitVector();
            let up_corrected = (view_dir_norm ^ right).UnitVector();
            
            let (w, h) = self.get_window_size();
            let window_aspect = (w as f64) / (h as f64);
            let film_back_aspect = self.camera.film_back_w / self.camera.film_back_h;
            let fov_horizontal = self.camera.fov.to_radians();

            let fov_vertical = match self.camera.fit_type {
                CameraFitType::HorizontalFit => {
                    2.0 * ((fov_horizontal / 2.0).tan() / window_aspect).atan()
                }
                CameraFitType::VerticalFit => {
                    2.0 * ((fov_horizontal / 2.0).tan() / film_back_aspect).atan()
                }
                CameraFitType::FillFit => {
                    let aspect = window_aspect.max(film_back_aspect);
                    2.0 * ((fov_horizontal / 2.0).tan() / aspect).atan()
                }
            };

            let tan_half_fov_vertical = (fov_vertical / 2.0).tan();
            
            let u = -(vec_to_pt * right) / (self.camera.near * tan_half_fov_vertical * window_aspect);
            let v = -(vec_to_pt * up_corrected) / (self.camera.near * tan_half_fov_vertical);
            
            RU_2dPoint::new(u, v)
        } else {
            let vec_to_pt = world_pt - self.camera.view_plane.origin;

            let u = vec_to_pt * self.camera.view_plane.xaxis;
            let v = vec_to_pt * self.camera.view_plane.yaxis;

            let u_min = self.camera.view_plane_u_min;
            let u_max = self.camera.view_plane_u_max;
            let v_min = self.camera.view_plane_v_min;
            let v_max = self.camera.view_plane_v_max;

            let u_range = u_max - u_min;
            let v_range = v_max - v_min;

            let u_norm = if u_range > 1e-10 {
                2.0 * (u - u_min) / u_range - 1.0
            } else {
                0.0
            };
            let v_norm = if v_range > 1e-10 {
                2.0 * (v - v_min) / v_range - 1.0
            } else {
                0.0
            };

            RU_2dPoint::new(u_norm, v_norm)
        }
    }

    pub fn draw_screen<F>(&self, draw_fn: F)
    where
        F: FnOnce(),
    {
        unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            glMatrixMode(GL_PROJECTION);
            glPushMatrix();
            glLoadIdentity();

            let (w, h) = self.get_window_size();
            glOrtho(0.0, w as f64, h as f64, 0.0, -1.0, 1.0);

            glMatrixMode(GL_MODELVIEW);
            glPushMatrix();
            glLoadIdentity();

            draw_fn();

            glMatrixMode(GL_MODELVIEW);
            glPopMatrix();
            glMatrixMode(GL_PROJECTION);
            glPopMatrix();

            glDisable(GL_BLEND);
        }
    }

    pub fn draw_text(&self, x: i32, y: i32, color: (f32, f32, f32), alpha: f32, text: &str) {
        let lines: Vec<&str> = text.split('\n').collect();
        let line_height = 18;

        for (i, line) in lines.iter().enumerate() {
            let line_y = y + i as i32 * line_height;
            self.draw_screen(|| unsafe {
                glColor4f(color.0, color.1, color.2, alpha);
                glListBase(FONT_BASE);
                glRasterPos2i(x, line_y);

                let bytes = line.as_bytes();
                glCallLists(
                    bytes.len() as i32,
                    GL_UNSIGNED_BYTE,
                    bytes.as_ptr() as *const _,
                );
            });
        }
    }

    pub fn draw_2d<F>(&self, mode: u32, width: Option<f32>, draw_fn: F)
    where
        F: FnOnce(&Self),
    {
        self.draw_screen(|| unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
            glEnable(GL_LINE_SMOOTH);
            glEnable(GL_POINT_SMOOTH);
            glHint(GL_LINE_SMOOTH_HINT, GL_NICEST);
            glHint(GL_POINT_SMOOTH_HINT, GL_NICEST);
            
            let line_width = width.unwrap_or(self.line_width);
            glLineWidth(line_width);
            glPointSize(line_width);
            
            glBegin(mode);
            draw_fn(self);
            glEnd();
            
            glDisable(GL_BLEND);
        });
    }

    pub fn draw_arrow_2d(
        &self,
        a: RU_2iPoint,
        b: RU_2iPoint,
        color: RU_Color,
        arrow_size: f32,
        line_width: f32,
    ) {
        let dir = b - a;
        let len = dir.length() as f32;

        if len < 1.0 {
            return;
        }

        let (dx, dy) = dir.to_f64();
        let nx = (dx / len as f64) as f32;
        let ny = (dy / len as f64) as f32;

        let perp_x = -ny;
        let perp_y = nx;

        let arrow_width = arrow_size * 0.4;
        let shaft_end = RU_2iPoint::new(
            (b.x as f32 - nx * arrow_size) as i32,
            (b.y as f32 - ny * arrow_size) as i32,
        );

        let arrow_base_left = RU_2iPoint::new(
            (b.x as f32 - nx * arrow_size + perp_x * arrow_width) as i32,
            (b.y as f32 - ny * arrow_size + perp_y * arrow_width) as i32,
        );
        let arrow_base_right = RU_2iPoint::new(
            (b.x as f32 - nx * arrow_size - perp_x * arrow_width) as i32,
            (b.y as f32 - ny * arrow_size - perp_y * arrow_width) as i32,
        );

        self.draw_2d(GL_POINTS, Some(line_width), |canvas| {
            canvas.add_vertex2i(a, color);
        });

        self.draw_2d(GL_LINES, Some(line_width), |canvas| {
            canvas.add_vertex2i(a, color);
            canvas.add_vertex2i(shaft_end, color);
        });

        self.draw_2d(GL_TRIANGLES, None, |canvas| {
            canvas.add_vertex2i(b, color);
            canvas.add_vertex2i(arrow_base_left, color);
            canvas.add_vertex2i(arrow_base_right, color);
        });
    }

    pub fn draw_border(&self, color: (f32, f32, f32), alpha: f32) {
        self.draw_screen(|| unsafe {
            let (w, h) = self.get_window_size();
            glColor4f(color.0, color.1, color.2, alpha);
            glBegin(GL_QUADS);
            glVertex2i(0, 0);
            glVertex2i(w, 0);
            glVertex2i(w, 2);
            glVertex2i(0, 2);
            glVertex2i(0, h - 2);
            glVertex2i(w, h - 2);
            glVertex2i(w, h);
            glVertex2i(0, h);
            glVertex2i(0, 2);
            glVertex2i(2, 2);
            glVertex2i(2, h - 2);
            glVertex2i(0, h - 2);
            glVertex2i(w - 2, 2);
            glVertex2i(w, 2);
            glVertex2i(w, h - 2);
            glVertex2i(w - 2, h - 2);
            glEnd();
        });
    }

    pub fn update(&self) {
        unsafe {
            let (w, h) = self.get_window_size();

            glReadPixels(
                0,
                0,
                w,
                h,
                GL_BGRA_EXT,
                GL_UNSIGNED_BYTE,
                self.bits_ptr as *mut _,
            );

            let screen_dc = GetDC(None);
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };
            let size = SIZE { cx: w, cy: h };
            let pt_src = POINT { x: 0, y: 0 };
            let _ = UpdateLayeredWindow(
                self.hwnd,
                screen_dc,
                None,
                Some(&size),
                self.mem_dc,
                Some(&pt_src),
                rgb(0, 0, 0),
                Some(&blend),
                ULW_ALPHA,
            );
            ReleaseDC(None, screen_dc);
        }
    }

    pub fn process(&self) {
        unsafe {
            let mut msg: MSG = Default::default();
            loop {
                if !IsWindow(self.hwnd).as_bool() {
                    break;
                }

                if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    if msg.message == WM_QUIT {
                        break;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(16));
                }
            }
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

    unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
        match msg {
            WM_NCHITTEST => LRESULT(HTTRANSPARENT as isize),
            WM_DESTROY => LRESULT(0),
            _ => DefWindowProcW(hwnd, msg, wp, lp),
        }
    }
}

impl Drop for GlCanvas {
    fn drop(&mut self) {
        unsafe {
            let _ = wglMakeCurrent(HDC(0), HGLRC(0));
            let _ = wglDeleteContext(self.hglrc);
            ReleaseDC(self.hwnd, self.hdc);
            DeleteDC(self.mem_dc);
            DeleteObject(self.hbitmap);
            self.bits_ptr = std::ptr::null_mut();
            let _ = DestroyWindow(self.hwnd);
            let h_instance = GetModuleHandleW(PCWSTR::null()).ok();
            if let Some(hinst) = h_instance {
                let _ = UnregisterClassW(PCWSTR::from_raw(self.class_name.as_ptr()), hinst);
            }
        }
    }
}
