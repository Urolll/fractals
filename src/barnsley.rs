use rand::distributions::{Distribution, Uniform};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn f1(coor: &Point) -> Point {
    let (_, y) = (coor.x, coor.y);
    Point::new(0.0, 0.16 * y)
}

fn f2(coor: &Point) -> Point {
    let (x, y) = (coor.x, coor.y);
    Point::new(0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6)
}

fn f3(coor: &Point) -> Point {
    let (x, y) = (coor.x, coor.y);
    Point::new(0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6)
}

fn f4(coor: &Point) -> Point {
    let (x, y) = (coor.x, coor.y);
    Point::new(-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44)
}

#[wasm_bindgen]
pub fn compute_barnsley(iter: u32) -> Vec<Point> {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut rng = rand::thread_rng();
    let uniform = Uniform::new(0.0, 1.0);
    let mut result = Vec::new();

    for _ in 0..iter {
        let p: f64 = uniform.sample(&mut rng);
        if p < 0.01 {
            let new_point = f1(&Point::new(x, y));
            x = new_point.x;
            y = new_point.y;
        } else if p < 0.86 {
            let new_point = f2(&Point::new(x, y));
            x = new_point.x;
            y = new_point.y;
        } else if p < 0.93 {
            let new_point = f3(&Point::new(x, y));
            x = new_point.x;
            y = new_point.y;
        } else {
            let new_point = f4(&Point::new(x, y));
            x = new_point.x;
            y = new_point.y;
        }

        result.push(Point::new(x, y));
    }

    result
}
