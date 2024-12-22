use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

mod barnsley;
mod mandelbrot;

#[wasm_bindgen]
pub fn start() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window
        .document()
        .expect("should have a document on the window");

    // Mandelbrot Canvas Setup
    let mandelbrot_canvas = document
        .get_element_by_id("mandelbrotCanvas")
        .expect("document should contain a canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("element is not an HtmlCanvasElement");

    mandelbrot_canvas.set_width(800);
    mandelbrot_canvas.set_height(600);

    let mandelbrot_context = mandelbrot_canvas
        .get_context("2d")
        .expect("should have a 2d context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context is not a CanvasRenderingContext2d");

    let w = mandelbrot_canvas.width() as usize;
    let h = mandelbrot_canvas.height() as usize;
    let max_iter = 100;
    let mandelbrot_data = mandelbrot::compute_mandelbrot(w, h, max_iter, -2.0, 1.0, -1.5, 1.5);
    let mut mandelbrot_image_data = vec![0; w * h * 4];

    for (i, &value) in mandelbrot_data.iter().enumerate() {
        let color = (value as f64 / max_iter as f64 * 255.0) as u8;
        let idx = i * 4;
        mandelbrot_image_data[idx] = color;
        mandelbrot_image_data[idx + 1] = color;
        mandelbrot_image_data[idx + 2] = color;
        mandelbrot_image_data[idx + 3] = 255;
    }

    let mandelbrot_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mandelbrot_image_data),
        w as u32,
        h as u32,
    )
    .expect("failed to create ImageData");

    mandelbrot_context
        .put_image_data(&mandelbrot_image_data, 0.0, 0.0)
        .unwrap();

    // Barnsley Fern Canvas Setup
    let barnsley_canvas = document
        .get_element_by_id("barnsleyCanvas")
        .expect("document should contain a canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("element is not an HtmlCanvasElement");

    barnsley_canvas.set_width(800);
    barnsley_canvas.set_height(600);

    let barnsley_context = barnsley_canvas
        .get_context("2d")
        .expect("should have a 2d context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context is not a CanvasRenderingContext2d");

    let barnsley_data = barnsley::compute_barnsley(100000);

    let barnsley_w = barnsley_canvas.width() as usize;
    let barnsley_h = barnsley_canvas.height() as usize;

    let mut barnsley_image_data = vec![0; barnsley_w * barnsley_h * 4];

    for point in barnsley_data {
        let x = point.x;
        let y = point.y;

        let px = ((x + 5.0) * barnsley_w as f64 / 10.0) as usize;
        let py = (y * barnsley_h as f64 / 10.0) as usize;

        if px < barnsley_w && py < barnsley_h {
            let idx = (py * barnsley_w + px) * 4;
            barnsley_image_data[idx] = 0;
            barnsley_image_data[idx + 1] = 255;
            barnsley_image_data[idx + 2] = 0;
            barnsley_image_data[idx + 3] = 255;
        }
    }

    let barnsley_image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&barnsley_image_data),
        barnsley_w as u32,
        barnsley_h as u32,
    )
    .expect("failed to create ImageData");

    barnsley_context
        .put_image_data(&barnsley_image_data, 0.0, 0.0)
        .unwrap();
}
