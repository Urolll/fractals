use rayon::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    pub fn add(&self, other: &Complex) -> Self {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (f64::powi(self.real, 2) + f64::powi(self.imag, 2)).sqrt()
    }

    pub fn square(&self) -> Self {
        Complex {
            real: f64::powi(self.real, 2) - f64::powi(self.imag, 2),
            imag: 2.0 * self.real * self.imag,
        }
    }
}

fn mandelbrot(c: Complex, iter: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..iter {
        if z.magnitude() > 4.0 {
            return i;
        }
        z = z.square().add(&c);
    }
    iter
}

// parallel computation of each pixel in mandelbrot
#[wasm_bindgen]
pub fn compute_mandelbrot(
    x: usize,
    y: usize,
    iter: u32,
    minr: f64,
    maxr: f64,
    mini: f64,
    maxi: f64,
) -> Vec<u32> {
    let rstep = (maxr - minr) / x as f64;
    let istep = (maxi - mini) / y as f64;

    let result: Vec<u32> = (0..y)
        .into_par_iter()
        .flat_map(|yy| {
            (0..x).into_par_iter().map(move |xx| {
                let real = minr + xx as f64 * rstep;
                let imag = mini + yy as f64 * istep;
                let c = Complex::new(real, imag);
                mandelbrot(c, iter)
            })
        })
        .collect();
    result
}
