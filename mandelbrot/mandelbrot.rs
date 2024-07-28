use num::Complex;
use super::MAX_ITER;

#[inline]
fn iterate_mandelbrot(last_n: Complex<f64>, constant: Complex<f64>) -> Complex<f64> {
    last_n * last_n + constant
}

pub fn is_in_set(constant: Complex<f64>) -> (bool, usize) {
    let mut start = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        start = iterate_mandelbrot(start, constant);
        if start.norm_sqr() > 4.0 {
            return (false, i);
        }
    }
    (true, MAX_ITER-1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Complex;

    #[test]
    fn test_mandelbrot_cycle_check() {
        let answer_in_set = is_in_set(Complex::new(0.0, 1.0));
        let answer_not_in_set = is_in_set(Complex::new(0.0, 2.0));

        assert!(matches!(answer_in_set, (true, _)));
        assert!(matches!(answer_not_in_set, (false, _)));
    }
}
