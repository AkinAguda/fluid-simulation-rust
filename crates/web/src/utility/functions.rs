use num_traits::ToPrimitive;

pub fn resize_canvas_to_display_size(canvas: &web_sys::HtmlCanvasElement) -> bool {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio();
    let dom_rect = canvas.get_bounding_client_rect();
    let display_width = (dom_rect.width() * dpr).round();
    let display_height = (dom_rect.height() * dpr).round();

    let need_resize = canvas.width().to_f64().unwrap() != display_width
        || canvas.height().to_f64().unwrap() != display_height;

    if need_resize {
        canvas.set_width(display_width.to_u32().unwrap());
        canvas.set_height(display_height.to_u32().unwrap());
    }

    need_resize
}

pub fn get_display_dimensions(width: u32, height: u32) -> (u32, u32) {
    let mut count: u32 = 220;
    let div = (u32::max(width, height) / u32::min(width, height)) as f32;
    if div as f32 <= 1.5 {
        count = 180;
    }
    if width > height {
        (
            count as u32,
            (count as f32 / (width / height) as f32) as u32,
        )
    } else if height > width {
        (
            (count as f32 / (height / width) as f32) as u32,
            count as u32,
        )
    } else {
        (width, height)
    }
}
