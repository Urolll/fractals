use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

mod mandelbrot;

#[wasm_bindgen]
pub fn start() {
    let window = web_sys::window().expect("no global `window` exists");

    let document = window
        .document()
        .expect("should have a document on the window");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("document should contain a canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("element is not an HtmlCanvasElement");

    web_sys::console::log_1(&"Canvas initialized!".into());

    canvas.set_width(800);
    canvas.set_height(600);
}
