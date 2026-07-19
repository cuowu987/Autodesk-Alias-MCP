// ============================================================================
// layered_canvas_gdi — GDI 分层透明画布库 + OpenGL 透明窗口库
// ============================================================================
// 模块结构：
//   ├── helpers.rs         rgb()、窗口查找、窗口置顶等基础工具
//   ├── layered_canvas.rs  GDI 分层透明画布（逐像素透明）
//   ├── gl_camera.rs       相机参数（透视/正交）
//   └── gl_canvas.rs       OpenGL 透明窗口（黑色透明 + 3D绘制）
//
// 最简使用案例：
//
// ▶ LayeredCanvas（GDI 透明画布）
//   let mut canvas = LayeredCanvas::new(100, 100, 400, 300, 0, 0, 0, 0)?;
//   canvas.fill(255, 0, 0, 128);  // 半透明红色
//   canvas.update_window()?;
//   canvas.process();
//
// ▶ GlCanvas（OpenGL 透明窗口 - 透视模式）
//   let camera = GlCamera::from_perspective(
//       RU_3dPoint::new(5.0, 5.0, 5.0),    // eye
//       RU_3dPoint::new(0.0, 0.0, 0.0),    // center
//       RU_3dVector::Y_AXIS,                // up
//       45.0, 0.1, 100.0,                  // fov, near, far
//       36.0, 24.0, 50.0,                  // film_back_w, film_back_h, focal_length
//       CameraFitType::HorizontalFit,       // fit_type
//   );
//   let canvas = GlCanvas::new(100, 100, 800, 600, "OpenGL", camera)?;
//   canvas.clear();
//   canvas.draw_3d(GL_TRIANGLES, |c| {
//       c.add_vertex3d(RU_3dPoint::new(0, 1, 0), (1.0, 0.0, 0.0, 1.0));
//       c.add_vertex3d(RU_3dPoint::new(-1, -1, 0), (0.0, 1.0, 0.0, 1.0));
//       c.add_vertex3d(RU_3dPoint::new(1, -1, 0), (0.0, 0.0, 1.0, 1.0));
//   });
//   canvas.update();
//
// ▶ GlCanvas（OpenGL 透明窗口 - 正交模式，从 Alias 窗口获取参数）
//   let (origin, right, up, top_right, center, view_dir) = window.orthographic_camera_params()?;
//   let camera = GlCamera::from_orthographic(origin, right, up, top_right, center, view_dir);
//   let canvas = GlCanvas::new(x, y, w, h, "Overlay", camera)?;
//   canvas.clear();
//   canvas.draw_3d(GL_QUADS, |c| {
//       c.add_vertex3d(point1, (1.0, 0.0, 0.0, 0.5));
//       c.add_vertex3d(point2, (0.0, 1.0, 0.0, 0.5));
//       ...
//   });
//   canvas.update();
// ============================================================================

pub extern crate windows;
pub use base_geometry_lib::Vector_Trait;

pub use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::Graphics::OpenGL::*,
    Win32::System::LibraryLoader::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

mod helpers;
mod layered_canvas;
mod gl_camera;
mod gl_canvas;

pub use helpers::*;
pub use layered_canvas::*;
pub use gl_camera::*;
pub use gl_canvas::*;

