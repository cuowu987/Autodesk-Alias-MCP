//全局忽视命名警告
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
pub mod layered_canvas;
use base_geometry_lib::*;
use openalias_rs::AlOutputType::kPrompt;
use openalias_rs::*;
use serde_json::{Value, json};
use std::ffi::{CStr, c_char, c_void};
pub mod console;
pub use console::*;

use std::io::{self, Read};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use once_cell::sync::Lazy;
use std::sync::Mutex;

mod funcs;
mod functions;
mod functionsinfo;
mod http;
mod imageruler;

pub use functions::handle_mcp_request;
pub use functionsinfo::mcp_tool_list;
pub use http::{
    HttpPart, build_http_response, build_options_response, find_subsequence, process_http_buffer,
    try_parse_http_request,
};
pub use imageruler::{add_ruler_to_image, add_ruler_to_image_with_params};

static ALEDITOR_X: RU_Atomic<AlEditor_X> = RU_Atomic::new();
static DIR: RU_AtomicString = RU_AtomicString::new();
static IS_MESSAGE_ON: AtomicBool = AtomicBool::new(false);
static MCP_MESSAGE_ID: AtomicI32 = AtomicI32::new(0);

// Global state for our TCP Server
static TCP_SERVER: Lazy<Mutex<Option<TcpListener>>> = Lazy::new(|| Mutex::new(None));
static CLIENTS: Lazy<Mutex<Vec<TcpStream>>> = Lazy::new(|| Mutex::new(Vec::new()));
static READ_BUFFERS: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| Mutex::new(Vec::new()));

extern "C" fn bicycle_callback(msg_type: i32, _data: *mut c_void) {
    safe_run(
        || {
            // 1. Process TCP Server
            if let Ok(server_guard) = TCP_SERVER.try_lock() {
                if let Some(ref listener) = *server_guard {
                    match listener.accept() {
                        Ok((stream, addr)) => {
                            stream.set_nonblocking(true).ok();
                            println!("New HTTP MCP connection from: {}", addr);
                            if let Ok(mut clients) = CLIENTS.lock() {
                                clients.push(stream);
                            }
                            if let Ok(mut bufs) = READ_BUFFERS.lock() {
                                bufs.push(Vec::with_capacity(8192));
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                        Err(e) => println!("TCP Accept Error: {}", e),
                    }
                }
            }

            // 2. Process Existing Clients (HTTP requests)
            let mut to_remove = Vec::new();
            if let (Ok(mut clients), Ok(mut bufs)) = (CLIENTS.try_lock(), READ_BUFFERS.try_lock()) {
                let min_len = clients.len().min(bufs.len());
                for i in 0..min_len {
                    let mut tmp_buf = [0; 8192];
                    match clients[i].read(&mut tmp_buf) {
                        Ok(0) => to_remove.push(i),
                        Ok(n) => {
                            bufs[i].extend_from_slice(&tmp_buf[..n]);
                            process_http_buffer(&mut bufs[i], &mut clients[i]);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                        Err(_) => to_remove.push(i),
                    }
                }
                for i in to_remove.into_iter().rev() {
                    let _ = clients[i].shutdown(Shutdown::Both);
                    clients.remove(i);
                    if i < bufs.len() {
                        bufs.remove(i);
                    }
                }
            }

            let _ = AlMessage::send_message_i32(
                msg_type,
                std::ptr::null_mut(),
                AlPriorityType::kIdleQueue,
            );
        },
        |err| {
            printf!(AlOutputType::kPrompt, "{}", err);
        },
    );
}

fn init_func_1() -> Result<(), String> {
    if IS_MESSAGE_ON.load(Ordering::Relaxed) == false {
        // Initialize TCP Server on Port 9000
        let listener = TcpListener::bind("127.0.0.1:9000").map_err(|e| e.to_string())?;
        listener.set_nonblocking(true).map_err(|e| e.to_string())?;
        *TCP_SERVER.lock().unwrap() = Some(listener);

        // 1. 添加或获取自定义消息类型
        let msg_user = "Mcp_Message";
        let msg_id = match AlMessage::add_message_type(msg_user) {
            Ok(handle) => handle.type_(),
            Err(_) => AlMessage::get_message_type(msg_user)?,
        };
        MCP_MESSAGE_ID.store(msg_id, Ordering::Relaxed);

        // 2. 注册处理器
        // 注意：callback 必须符合 AlMessage 要求的函数签名
        AlMessage::add_message_handler_i32(msg_id, bicycle_callback as *mut c_void)?;

        // 3. 首次触发消息
        AlMessage::send_message_i32(msg_id, std::ptr::null_mut(), AlPriorityType::kIdleQueue)?;
        IS_MESSAGE_ON.store(true, Ordering::Relaxed);
    }
    Ok(())
}

extern "C" fn init_func() {
    safe_run(
        || {
            safe_run2(|| {
                layered_canvas::init_canvas().unwrap_or_else(|e| {
                    println!("Failed to initialize curve highlight canvas: {}", e);
                });


                console_win();
                init_func_1()?;
                
                ////////
                let mut aleditor_binding = ALEDITOR_X.lock("ALEDITOR_X lock is error")?;
                let aleditor_x = aleditor_binding.as_mut().ok_or("aleditor_x is none")?;

                aleditor_x.alfunc.finished()?;

                Ok(())
            })
            .unwrap_or_else(|e| {layered_canvas::init_canvas().unwrap_or_else(|e| {
                    println!("Failed to initialize curve highlight canvas: {}", e);
                });
                println!("Failed to initialize message handlers: {}", e);
            })
        },
        |err| {
            printf!(AlOutputType::kPrompt, "{}", err);
        },
    );
}

fn shutdown_tcp() {
    if let Ok(mut clients) = CLIENTS.lock() {
        for client in clients.iter_mut() {
            let _ = client.shutdown(Shutdown::Both);
        }
        clients.clear();
    }
    if let Ok(mut bufs) = READ_BUFFERS.lock() {
        bufs.clear();
    }
    if let Ok(mut server) = TCP_SERVER.lock() {
        *server = None;
    }
}

extern "C" fn TextButton_ClearAssist() {
    safe_run(
        || {
            safe_run2(|| {
                let had_content = {
                    let mut curve_guard = layered_canvas::CURVE_POINTS.lock().unwrap();
                    let had_curve = curve_guard.is_some();
                    *curve_guard = None;

                    let mut name_guard = layered_canvas::CURVE_NAME.lock().unwrap();
                    *name_guard = String::new();

                    let mut assist_points_guard = layered_canvas::ASSIST_POINTS.lock().unwrap();
                    let had_points = !assist_points_guard.is_empty();
                    assist_points_guard.clear();

                    let mut assist_arrows_guard = layered_canvas::ASSIST_ARROWS.lock().unwrap();
                    let had_arrows = !assist_arrows_guard.is_empty();
                    assist_arrows_guard.clear();

                    let mut assist_lines_guard = layered_canvas::ASSIST_LINES.lock().unwrap();
                    let had_lines = !assist_lines_guard.is_empty();
                    assist_lines_guard.clear();

                    let mut assist_rectangles_guard = layered_canvas::ASSIST_RECTANGLES.lock().unwrap();
                    let had_rectangles = !assist_rectangles_guard.is_empty();
                    assist_rectangles_guard.clear();

                    had_curve || had_points || had_arrows || had_lines || had_rectangles
                };

                if had_content {
                    if let Ok(window) = AlUniverse::current_window().ok_or("no alwindow") {
                        if let Ok(alcamera) = window.camera().ok_or("no camera") {
                            if let Ok((camera, camera_info)) = layered_canvas::build_camera(alcamera) {
                                let _ = layered_canvas::redraw_canvas(camera, camera_info);
                            }
                        }
                    }
                    printf!(kPrompt, "Screen assist display overlay cleared");
                } else {
                    printf!(kPrompt, "No screen assist display overlay to clear");
                }

                Ok(())
            })
            .unwrap();
        },
        |e| {
            printf!(kPrompt, "panic: {}", e);
        },
    );
}
extern "C" fn Close_MCP() {
    safe_run(
        || {
            safe_run2(|| {
                unsafe {
                    let _ = FreeConsole();
                }
                layered_canvas::close_overlay_if_exists();
                shutdown_tcp();
                if IS_MESSAGE_ON.load(Ordering::Relaxed) == true {
                    let _ = AlMessage::remove_message_handler_i32(
                        MCP_MESSAGE_ID.load(Ordering::Relaxed),
                        bicycle_callback as *mut c_void,
                    );
                    IS_MESSAGE_ON.store(false, Ordering::Relaxed);
                }

                Ok(())
            })
            .unwrap();
        },
        |e| {
            printf!(kPrompt, "panic: {}", e);
        },
    );
}

/// DLL 初始化函数
#[unsafe(no_mangle)]
extern "C" fn plugin_init(dir_name: *const c_char) -> i32 {
    safe_run(
        || {
            safe_run2(|| {
                let dir_rust = unsafe { CStr::from_ptr(dir_name).to_str().unwrap_or("") };
                if dir_rust.is_empty() {
                    return Err("dir_name is empty".to_string());
                }
                DIR.set(dir_rust.to_string(), "DIR is error")?; //保存目录
                let dir_cpp = dir_rust.replace("\\RustPulgin\\target\\debug", "\\bin");
                let icon_path = &format!("{}/a.jpg", dir_cpp);
                let aleditor_x = AlEditor_X::Init(
                    "AliasMCPserve",
                    "AliasMCPserveFunc",
                    "AliasMCPserve",
                    "mp_objtools",
                    &icon_path,
                    Some(init_func),
                    None,
                    None,
                    None,
                    None,
                )?;

                aleditor_x
                    .aleditor
                    .add_text_button("Clear Assist Display", TextButton_ClearAssist)?;
                aleditor_x
                    .aleditor
                    .add_button_auto_close("close MCP", Close_MCP, true)?;

                aleditor_x.End()?;

                let mut aleditor_binding = ALEDITOR_X.lock("ALEDITOR_X lock is error")?;
                *aleditor_binding = Some(aleditor_x);

                Ok(0)
            })
            .unwrap()
        },
        |e| {
            printf!(kPrompt, "panic: {}", e);
        },
    )
    .unwrap_or(1)
}

/// DLL 卸载函数
#[unsafe(no_mangle)]
extern "C" fn plugin_exit() -> i32 {
    Close_MCP();
    ALEDITOR_X.clear("ALEDITOR_X clear is error").ok();

    0
}
