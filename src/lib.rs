mod mandelbrot;

extern crate nalgebra as na;
use crate::mandelbrot::mandelbrot;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
use rayon::prelude::*;

/// Generate an image from the Mandelbrot set.
///
/// # Examples
///
/// ```
/// extern crate nalgebra as na;
///
/// let img = mandelbrot_common::generate(8, 4f64, 50);
/// let correct = na::Matrix5::from_row_slice(&[
///     253, 253, 252, 252, 252,
///     253, 252, 252, 251, 205,
///     253, 251, 249, 205, 205,
///     253, 205, 205, 205, 205,
///     253, 251, 249, 205, 205,
/// ]);
/// assert_eq!(img, correct);
/// ```
pub fn generate(n: usize, threshold: f64, max_steps: u8) -> na::DMatrix<u8> {
    let x_lower = 0;
    let x_upper = (5 * n) / 8;
    let y_lower = (2 * n) / 10;
    let y_upper = (8 * n) / 10;

    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    let vec_storage: Vec<u8> = (x_lower..x_upper)
        .into_par_iter()
        .flat_map(move |x| {
            (y_lower..y_upper).into_par_iter().map(move |y| {
                let c = na::Complex {
                    re: 4f64 * (x as f64 - n as f64 / 2f64) / n as f64,
                    im: 4f64 * (y as f64 - n as f64 / 2f64) / n as f64,
                };
                255u8 - mandelbrot(c, threshold, max_steps)
            })
        })
        .collect();

    #[cfg(all(target_arch =	"wasm32", not(target_os	= "emscripten")))]
    let vec_storage: Vec<u8> = (x_lower..x_upper)
        .flat_map(move |x| {
            (y_lower..y_upper).map(move |y| {
                let c = na::Complex {
                    re: 4f64 * (x as f64 - n as f64 / 2f64) / n as f64,
                    im: 4f64 * (y as f64 - n as f64 / 2f64) / n as f64,
                };
                255u8 - mandelbrot(c, threshold, max_steps)
            })
        })
        .collect();

    na::DMatrix::from_vec(x_upper, y_upper - y_lower, vec_storage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let img = generate(8, 4f64, 50);
        let correct = na::Matrix5::from_row_slice(&[
            253, 253, 252, 252, 252,
            253, 252, 252, 251, 205,
            253, 251, 249, 205, 205,
            253, 205, 205, 205, 205,
            253, 251, 249, 205, 205,
        ]);
        assert_eq!(img, correct);
    }
}
