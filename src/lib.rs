extern crate nalgebra as na;
use rayon::prelude::*;

fn get_iter(c: na::Complex<f64>, threshold: f64, max_steps: u8) -> u8 {
    let mut z = c.clone();
    let mut i = 1u8;
    while i < max_steps && (z * z.conj()).re < threshold {
        z = z * z + c;
        i += 1;
    }
    i
}

pub fn generate(n: usize, threshold: f64, max_steps: u8) -> na::DMatrix<u8> {
    let x_lower = 0;
    let x_upper = (5 * n) / 8;
    let y_lower = (2 * n) / 10;
    let y_upper = (8 * n) / 10;

    let vec_storage: Vec<u8> = (x_lower..x_upper)
        .into_par_iter()
        .flat_map(move |x| {
            (y_lower..y_upper).into_par_iter().map(move |y| {
                let c = na::Complex {
                    re: 4f64 * (x as f64 - n as f64 / 2f64) / n as f64,
                    im: 4f64 * (y as f64 - n as f64 / 2f64) / n as f64,
                };
                255u8 - get_iter(c, threshold, max_steps)
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
            254, 254, 253, 253, 253, 254, 253, 252, 251, 205, 254, 252, 250, 205, 205, 254, 205,
            205, 205, 205, 254, 252, 250, 205, 205,
        ]);
        assert_eq!(img, correct);
    }

    #[test]
    fn speedtest() {
        let img = generate(10000, 4f64, 255);
    }
}
