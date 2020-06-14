use num_complex::Complex;

fn abs(c: Complex<f64>) -> f64 {
    (c * c.conj()).re
}

pub fn mandelbrot(c: Complex<f64>, threshold: f64, max_steps: u8) -> u8 {
    let mut z = c.clone();
    let mut i = 1u8;
    while i < max_steps && abs(z) < threshold {
        z = z * z + c;
        i += 1;
    }

    // see: "Renormalizing the Mandelbrot Escape" for this derivation.
    // https://linas.org/art-gallery/escape/escape.html
    if i == max_steps {
        max_steps
    } else {
        i + 1 - abs(z).log2().log10().round() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mandelbrot(Complex { re: -2.0, im: 3.0 }, 4.0, 50u8), 1);
        assert_eq!(
            mandelbrot(
                Complex {
                    re: -10000.0,
                    im: 10000.0
                },
                4.0,
                50u8
            ),
            1
        );
        assert_eq!(mandelbrot(Complex { re: 0.0, im: 0.0 }, 4.0, 50u8), 50);
        assert_eq!(mandelbrot(Complex { re: -1.0, im: 0.12 }, 4.0, 50u8), 50);
        assert_eq!(
            mandelbrot(
                Complex {
                    re: -0.88,
                    im: -0.44
                },
                4.0,
                50u8
            ),
            7
        );
    }
}
