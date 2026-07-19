use crate::*;
use layered_canvas_gdi::{GL_LINE_STRIP, GL_POINTS, GL_LINES, GL_LINE_LOOP, GlCamera, GlCanvas};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::SystemTime;

pub static OVERLAY_CANVAS: Lazy<Mutex<Option<GlCanvas>>> = Lazy::new(|| Mutex::new(None));
pub static CURVE_POINTS: Lazy<Mutex<Option<Vec<RU_3dPoint>>>> = Lazy::new(|| Mutex::new(None));
pub static CURVE_COLOR: Lazy<Mutex<RU_Color>> =
    Lazy::new(|| Mutex::new(RU_Color::new(1.0, 0.0, 0.0, 0.8)));
pub static CURVE_WIDTH: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(2.0));
pub static CURVE_NAME: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static ASSIST_POINTS: Lazy<Mutex<Vec<(RU_2iPoint, f32, RU_Color)>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ASSIST_ARROWS: Lazy<Mutex<Vec<(RU_2iPoint, RU_2iPoint, RU_Color, f32, f32)>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ASSIST_LINES: Lazy<Mutex<Vec<(String, i32, f32, RU_Color)>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ASSIST_RECTANGLES: Lazy<Mutex<Vec<(RU_2iPoint, RU_2iPoint, f32, RU_Color)>>> = Lazy::new(|| Mutex::new(Vec::new()));
static IS_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static LAST_REDRAW_TIME: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
static LAST_CAMERA_INFO: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
const REDRAW_INTERVAL_MS: u64 = 10;

pub fn tessellate_curve(alcurve: &AlCurve, num_points: usize) -> Result<Vec<RU_3dPoint>, String> {
    let knots: Vec<f64> = alcurve.knot_vector()?;
    if knots.len() < 2 {
        return Err("curve has no knots".to_string());
    }
    let t_min = knots[0];
    let t_max = knots[knots.len() - 1];

    let mut points = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let t = t_min + (t_max - t_min) * (i as f64) / ((num_points - 1) as f64);
        let mut p = [0.0; 3];
        let mut dp = [0.0; 3];
        alcurve.eval(t, true, &mut p, &mut dp)?;
        points.push(RU_3dPoint::new(p[0], p[1], p[2]));
    }
    Ok(points)
}

pub fn build_camera(camera_obj: AlCamera) -> Result<(GlCamera, String), String> {
    let is_orthographic = camera_obj.type_() == AlObjectType::kOrthographicCameraType;
    if is_orthographic {
        let window = AlUniverse::current_window().ok_or("No current window".to_string())?;
        let (origin, right, up, top_right, center, view_dir) =
            window.orthographic_camera_params()?;
        let camera = GlCamera::from_orthographic(origin, right, up, top_right, center, view_dir);
        Ok((camera, camera.to_string()))
    } else {
        let perspective = camera_obj.as_perspective_camera()?;
        let (
            eye_pos,
            center_pos,
            up_vec,
            fov,
            near,
            far,
            film_back_w,
            film_back_h,
            focal_length,
            fit_code,
        ) = perspective.camera_params()?;
        let camera = GlCamera::from_perspective(
            eye_pos,
            center_pos,
            up_vec,
            fov,
            near,
            far,
            film_back_w,
            film_back_h,
            focal_length,
            fit_code as i32,
        );
        Ok((camera, camera.to_string()))
    }
}

pub fn build_current_camera() -> Result<((GlCamera, String), (i32, i32, i32, i32)), String> {
    let window = AlUniverse::current_window().ok_or("当前没有窗口".to_string())?;
    let camera_obj = window.camera().ok_or("当前没有相机".to_string())?;
    let (x, y, w, h) = window.position_absolute()?;
    Ok((build_camera(camera_obj)?, (x, y, w, h)))
}

pub fn redraw_canvas(camera: GlCamera, _camera_info: String) -> Result<(), String> {
    let mut canvas_guard = OVERLAY_CANVAS.lock().map_err(|e| e.to_string())?;
    let canvas = match canvas_guard.as_mut() {
        Some(c) => c,
        None => return Ok(()),
    };

    canvas.set_camera(camera);

    let curve_guard = CURVE_POINTS.lock().map_err(|e| e.to_string())?;
    let curve_points = curve_guard.as_ref();

    let color = *CURVE_COLOR.lock().map_err(|e| e.to_string())?;
    let width = *CURVE_WIDTH.lock().map_err(|e| e.to_string())?;

    let assist_points = ASSIST_POINTS.lock().map_err(|e| e.to_string())?;
    let assist_points_clone: Vec<_> = assist_points.iter().cloned().collect();

    let assist_lines = ASSIST_LINES.lock().unwrap();
    let assist_lines_clone: Vec<_> = assist_lines.iter().cloned().collect();

    let assist_rectangles = ASSIST_RECTANGLES.lock().unwrap();
    let assist_rectangles_clone: Vec<_> = assist_rectangles.iter().cloned().collect();

    let (canvas_w, canvas_h) = (canvas.width(), canvas.height());

    canvas.with_context(|canvas| {
        canvas.clear();
        canvas.draw_border((0.0, 1.0, 1.0), 1.0);

        if let Some(points) = curve_points {
            canvas.draw_3d(GL_LINE_STRIP, Some(width), |canvas| {
                for pt in points {
                    canvas.add_vertex3d(*pt, (color.r, color.g, color.b, color.a));
                }
            });
        }

        for &(point, w, color) in assist_points_clone.iter() {
            canvas.draw_2d(GL_POINTS, Some(w), |canvas| {
                canvas.add_vertex2i(point, color);
            });
        }

        let assist_arrows = ASSIST_ARROWS.lock().unwrap();
        let assist_arrows_clone: Vec<_> = assist_arrows.iter().cloned().collect();
        for &(a, b, color, arrow_size, line_width) in assist_arrows_clone.iter() {
            canvas.draw_arrow_2d(a, b, color, arrow_size, line_width);
        }

        for (line_type, pos, line_width, color) in assist_lines_clone {
            canvas.draw_2d(GL_LINES, Some(line_width), |canvas| {
                if line_type == "horizontal" {
                    let a = RU_2iPoint::new(0, pos);
                    let b = RU_2iPoint::new(canvas_w, pos);
                    canvas.add_vertex2i(a, color);
                    canvas.add_vertex2i(b, color);
                } else {
                    let a = RU_2iPoint::new(pos, 0);
                    let b = RU_2iPoint::new(pos, canvas_h);
                    canvas.add_vertex2i(a, color);
                    canvas.add_vertex2i(b, color);
                }
            });
        }

        for (p1, p2, line_width, color) in assist_rectangles_clone {
            canvas.draw_2d(GL_LINE_LOOP, Some(line_width), |canvas| {
                canvas.add_vertex2i(RU_2iPoint::new(p1.x, p1.y), color);
                canvas.add_vertex2i(RU_2iPoint::new(p2.x, p1.y), color);
                canvas.add_vertex2i(RU_2iPoint::new(p2.x, p2.y), color);
                canvas.add_vertex2i(RU_2iPoint::new(p1.x, p2.y), color);
            });
        }

        canvas.update();
    });
    Ok(())
}

fn dag_node_func(_msg_type: AlMessageType, dagnode_ptr: *mut AlDagNode_ptr) {
    let dag: AlDagNode = AlDagNode { ptr: dagnode_ptr };
    safe_run(
        || {
            safe_run2(|| {
                let name = dag.name();
                if name == "Persp_view" || name == "Persp_up" {
                    return Ok(());
                }
                let window = AlUniverse::current_window().ok_or("no alwindow")?;
                let alcamera = window.camera().ok_or("no camera")?;
                let (camera, camera_info) = match build_camera(alcamera) {
                    Ok(c) => c,
                    Err(_) => return Ok(()),
                };

                if camera.eye == RU_3dPoint::ORIGIN {
                    return Ok(());
                }

                let curve_name_guard = CURVE_NAME.lock().map_err(|e| e.to_string())?;
                let curve_name = curve_name_guard.as_str();

                let mut need_redraw = false;
                let s =  dag.name_ex() == curve_name;

                if !curve_name.is_empty() && s {
                    let alcurve = dag.copy_wrapper().as_curve_node()?.curve().unwrap();
                    let curve_points = tessellate_curve(&alcurve, 100)?;
                    let mut guard = CURVE_POINTS.lock().map_err(|e| e.to_string())?;
                    *guard = Some(curve_points);
                    need_redraw = true;
                }

                drop(curve_name_guard);

                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);

                let mut last_redraw_guard = LAST_REDRAW_TIME.lock().map_err(|e| e.to_string())?;
                let time_elapsed = now.saturating_sub(*last_redraw_guard);

                let mut last_camera_guard = LAST_CAMERA_INFO.lock().map_err(|e| e.to_string())?;
                let camera_changed = *last_camera_guard != camera_info;

                if need_redraw || (time_elapsed >= REDRAW_INTERVAL_MS && camera_changed) {
                    {
                        *last_redraw_guard = now;
                    }
                    {
                        *last_camera_guard = camera_info.clone();
                    }
                    redraw_canvas(camera, camera_info)?;
                }

                Ok(())
            })
            .unwrap();
        },
        |err| {
            printf!(AlOutputType::kPrompt, "{}", err);
        },
    );
    std::mem::forget(dag);
}

pub fn init_canvas() -> Result<(), String> {
    let mut initialized = IS_INITIALIZED.lock().map_err(|e| e.to_string())?;
    if *initialized {
        return Ok(());
    }

    let ((camera, _), (x, y, w, h)) = build_current_camera()?;
    let canvas =
        GlCanvas::new(x, y, w, h, "Curve Assist Display", camera).map_err(|e| e.to_string())?;
    {
        let mut canvas_guard = OVERLAY_CANVAS.lock().map_err(|e| e.to_string())?;
        *canvas_guard = Some(canvas);
    }

    redraw_canvas(camera, String::new())?;

    let handlers = [
        AlMessageType::DagNodeModifiedGeometry,
        AlMessageType::DagNodeModified,
    ];

    for &msg_type in &handlers {
        AlMessage::add_message_handler(msg_type, dag_node_func as *mut std::ffi::c_void)?;
    }

    *initialized = true;
    Ok(())
}

pub fn close_overlay_if_exists() -> bool {
    let mut initialized = IS_INITIALIZED.lock().unwrap();
    *initialized = false;

    let handlers = [
        AlMessageType::DagNodeModifiedGeometry,
        AlMessageType::DagNodeModified,
    ];

    for &msg_type in &handlers {
        let _ = AlMessage::remove_message_handler(msg_type, dag_node_func as *mut std::ffi::c_void);
    }

    let mut guard = OVERLAY_CANVAS.lock().unwrap();
    let taken = guard.take();

    let mut curve_guard = CURVE_POINTS.lock().unwrap();
    *curve_guard = None;

    let mut name_guard = CURVE_NAME.lock().unwrap();
    *name_guard = String::new();

    let mut assist_points_guard = ASSIST_POINTS.lock().unwrap();
    assist_points_guard.clear();

    let mut assist_arrows_guard = ASSIST_ARROWS.lock().unwrap();
    assist_arrows_guard.clear();

    let mut assist_lines_guard = ASSIST_LINES.lock().unwrap();
    assist_lines_guard.clear();

    let mut assist_rectangles_guard = ASSIST_RECTANGLES.lock().unwrap();
    assist_rectangles_guard.clear();

    match taken {
        Some(canvas) => {
            drop(canvas);
            true
        }
        None => false,
    }
}
