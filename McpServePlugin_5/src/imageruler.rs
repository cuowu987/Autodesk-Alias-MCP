use image::{self, GenericImageView, ColorType, imageops};

fn get_char_bitmap(c: char) -> [u8; 7] {
    match c {
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x06, 0x08, 0x10, 0x1F],
        '3' => [0x0E, 0x11, 0x01, 0x06, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        _ => [0x0E, 0x11, 0x01, 0x06, 0x04, 0x00, 0x04],
    }
}

// 自动计算适合图片尺寸的标尺参数
fn auto_calculate_ruler_params(width: u32, height: u32) -> (u32, u32, f64) {
    let min_dim = width.min(height);
    
    // 根据图片最小边长动态调整参数
    let (major_tick, minor_tick, font_scale) = if min_dim < 100 {
        // 极小图片：使用非常小的参数
        (20, 5, 0.8)
    } else if min_dim < 200 {
        // 小图片：使用较小的参数
        (50, 10, 1.0)
    } else if min_dim < 400 {
        // 中等图片
        (100, 20, 1.5)
    } else {
        // 大图片：使用默认参数
        (100, 20, 2.0)
    };
    
    (major_tick, minor_tick, font_scale)
}

pub fn add_ruler_to_image(input_path: &str, output_path: &str, img_w: u32, img_h: u32) -> Result<f64, String> {
    let img = image::open(input_path).map_err(|e| format!("Open image failed: {}", e))?;
    let (orig_width, orig_height) = img.dimensions();
    
    let upscale_factor = img_w as f64 / orig_width as f64;
    let upscale_factor = upscale_factor.min(img_h as f64 / orig_height as f64).round();
    
    let (scaled_img, scaled_width, scaled_height) = if upscale_factor > 1.0 {
        // 放大图片
        let new_width = (orig_width as f64 * upscale_factor) as u32;
        let new_height = (orig_height as f64 * upscale_factor) as u32;
        let scaled = imageops::resize(&img, new_width, new_height, imageops::FilterType::Nearest);
        (scaled, new_width, new_height)
    } else {
        // 不需要放大，保持原样
        (img.to_rgba8(), orig_width, orig_height)
    };
    
    // 为放大后的图片计算合适的标尺参数
    let (major_tick, minor_tick, font_scale) = auto_calculate_ruler_params(scaled_width, scaled_height);
    
    // 直接在放大的图片上绘制标尺
    add_ruler_to_image_with_params_internal(
        &scaled_img,
        scaled_width,
        scaled_height,
        output_path,
        major_tick,
        minor_tick,
        font_scale,
    )?;
    
    Ok(upscale_factor)
}

fn add_ruler_to_image_with_params_internal(
    img_data: &image::RgbaImage,
    width: u32,
    height: u32,
    output_path: &str,
    major_tick: u32,
    minor_tick: u32,
    font_scale: f64,
) -> Result<(), String> {
    let mut buffer = img_data.to_vec();

    let color: [u8; 4] = [255, 0, 0, 255];
    let light_color: [u8; 4] = [255, 128, 128, 255];

    let put_pixel = |buf: &mut [u8], x: i32, y: i32, color: &[u8; 4]| {
        if x >= 0 && y >= 0 && (x as u32) < width && (y as u32) < height {
            let idx = ((y as u32 * width + x as u32) * 4) as usize;
            buf[idx] = color[0];
            buf[idx + 1] = color[1];
            buf[idx + 2] = color[2];
            buf[idx + 3] = color[3];
        }
    };

    let draw_hline = |buf: &mut [u8], y: i32, x_start: i32, x_end: i32, color: &[u8; 4]| {
        let x_min = x_start.min(x_end);
        let x_max = x_start.max(x_end);
        for x in x_min..=x_max {
            put_pixel(buf, x, y, color);
        }
    };

    let draw_vline = |buf: &mut [u8], x: i32, y_start: i32, y_end: i32, color: &[u8; 4]| {
        let y_min = y_start.min(y_end);
        let y_max = y_start.max(y_end);
        for y in y_min..=y_max {
            put_pixel(buf, x, y, color);
        }
    };

    let draw_char = |buf: &mut [u8], x: i32, y: i32, c: char, color: &[u8; 4], scale: f64| {
        let font = get_char_bitmap(c);
        let base_width = 5;
        let base_height = 7;
        let scaled_width_char = (base_width as f64 * scale) as i32;
        let scaled_height_char = (base_height as f64 * scale) as i32;

        for fy_scaled in 0..scaled_height_char {
            for fx_scaled in 0..scaled_width_char {
                let fx = (fx_scaled as f64 / scale) as usize;
                let fy = (fy_scaled as f64 / scale) as usize;
                if fx < base_width && fy < base_height && (font[fy] >> (4 - fx)) & 1 != 0 {
                    put_pixel(buf, x + fx_scaled, y + fy_scaled, color);
                }
            }
        }
    };

    let draw_string = |buf: &mut [u8], mut x: i32, y: i32, s: &str, color: &[u8; 4], scale: f64| {
        let char_spacing = (6.0 * scale) as i32;
        for c in s.chars() {
            draw_char(buf, x, y, c, color, scale);
            x += char_spacing;
        }
    };

    let scale = font_scale;
    let tick_major = (15.0 * scale) as i32;
    let tick_minor = (8.0 * scale) as i32;
    let label_offset = (2.0 * scale) as i32;
    let char_width = (6.0 * scale) as i32;

    // 顶部标尺
    let top_y = (5.0 * scale) as i32;
    for x in (0..=width).step_by(minor_tick as usize) {
        let x_i32 = x as i32;
        let is_major = x % major_tick == 0;
        let tick_len = if is_major { tick_major } else { tick_minor };
        draw_vline(&mut buffer, x_i32, top_y, top_y + tick_len - 1, if is_major { &color } else { &light_color });
        if is_major {
            let label = format!("{}", x);
            let label_width = (label.len() as i32) * char_width;
            let label_x = x_i32 - label_width / 2;
            let label_y = top_y + tick_len + label_offset;
            draw_string(&mut buffer, label_x, label_y, &label, &color, scale);
        }
    }

    // 底部标尺
    let bottom_y = (height as f64 - 30.0 * scale) as i32;
    let bottom_y_clamped = bottom_y.max(0);
    for x in (0..=width).step_by(minor_tick as usize) {
        let x_i32 = x as i32;
        let is_major = x % major_tick == 0;
        let tick_len = if is_major { tick_major } else { tick_minor };
        draw_vline(&mut buffer, x_i32, bottom_y_clamped, bottom_y_clamped + tick_len - 1, if is_major { &color } else { &light_color });
        if is_major {
            let label = format!("{}", x);
            let label_width = (label.len() as i32) * char_width;
            let label_x = x_i32 - label_width / 2;
            let label_y = bottom_y_clamped + tick_len + label_offset;
            draw_string(&mut buffer, label_x, label_y, &label, &color, scale);
        }
    }

    // 左侧标尺（以左下角为原点，凑百标注）
    let left_x = (5.0 * scale) as i32;
    // 先画所有小刻度
    for y in (0..=height).step_by(minor_tick as usize) {
        let y_i32 = y as i32;
        let is_major = (height - y) % major_tick == 0;
        let tick_len = if is_major { tick_major } else { tick_minor };
        draw_hline(&mut buffer, y_i32, left_x, left_x + tick_len as i32 - 1, if is_major { &color } else { &light_color });
    }
    // 再只在凑百处画文字
    let mut y_label = 0u32;
    while y_label <= height {
        let y_i32 = (height - y_label) as i32;
        let label = format!("{}", y_label);
        let label_x = left_x + tick_major + label_offset;
        let label_y = y_i32 - (5.0 * scale) as i32;
        draw_string(&mut buffer, label_x, label_y, &label, &color, scale);
        y_label += major_tick;
    }

    // 右侧标尺（以左下角为原点，凑百标注）
    let right_x = (width as f64 - 30.0 * scale) as i32;
    let right_x_clamped = right_x.max(0);
    // 先画所有小刻度
    for y in (0..=height).step_by(minor_tick as usize) {
        let y_i32 = y as i32;
        let is_major = (height - y) % major_tick == 0;
        let tick_len = if is_major { tick_major } else { tick_minor };
        draw_hline(&mut buffer, y_i32, right_x_clamped, right_x_clamped + tick_len as i32 - 1, if is_major { &color } else { &light_color });
    }
    // 再只在凑百处画文字
    let mut y_label = 0u32;
    while y_label <= height {
        let y_i32 = (height - y_label) as i32;
        let label = format!("{}", y_label);
        let label_width = (label.len() as i32) * char_width;
        let label_x = right_x_clamped - label_width - label_offset;
        let label_y = y_i32 - (5.0 * scale) as i32;
        draw_string(&mut buffer, label_x, label_y, &label, &color, scale);
        y_label += major_tick;
    }

    image::save_buffer(output_path, &buffer, width, height, ColorType::Rgba8)
        .map_err(|e| format!("Save image failed: {}", e))
}

pub fn add_ruler_to_image_with_params(
    input_path: &str,
    output_path: &str,
    major_tick: u32,
    minor_tick: u32,
    font_scale: f64,
) -> Result<(), String> {
    let img = image::open(input_path).map_err(|e| format!("Open image failed: {}", e))?;
    let (width, height) = img.dimensions();
    let img_data = img.to_rgba8();
    
    add_ruler_to_image_with_params_internal(
        &img_data,
        width,
        height, output_path,
        major_tick,
        minor_tick,
        font_scale,
    )
}
