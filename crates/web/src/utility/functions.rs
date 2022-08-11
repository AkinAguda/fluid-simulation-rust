use num_traits::ToPrimitive;

pub fn resize_canvas_to_display_size(canvas: web_sys::HtmlCanvasElement) -> bool {
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
