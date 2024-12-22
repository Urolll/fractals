use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

mod mandelbrot;

#[wasm_bindgen]
pub fn start() {
    let window = web_sys::window().expect("no global `window` exists");

    let document = window
        .document()
        .expect("should have a document on the window");

    let canvas = document
        .get_element_by_id("mandelbrotCanvas")
        .expect("document should contain a canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("element is not an HtmlCanvasElement");

    canvas.set_width(800);
    canvas.set_height(600);

    let context = canvas
        .get_context("2d")
        .expect("should have a 2d context")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("context is not a CanvasRenderingContext2d");

    let w = canvas.width() as usize;
    let h = canvas.height() as usize;
    let max_iter = 100;
    let mandelbrot_data = mandelbrot::compute_mandelbrot(w, h, max_iter, -2.0, 1.0, -1.5, 1.5);
    let mut image_data = vec![0; w * h * 4];

    for (i, &value) in mandelbrot_data.iter().enumerate() {
        let color = (value as f64 / max_iter as f64 * 255.0) as u8;
        let idx = i * 4;
        image_data[idx] = color;
        image_data[idx + 1] = color;
        image_data[idx + 2] = color;
        image_data[idx + 3] = 255;
    }

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&image_data),
        w as u32,
        h as u32,
    )
    .expect("failed to create ImageData");

    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
