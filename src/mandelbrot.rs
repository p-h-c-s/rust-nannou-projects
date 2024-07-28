use num::Complex;

#[inline]
fn iterate_mandelbrot(last_n: Complex<f32>, constant: Complex<f32>) -> Complex<f32> {
    last_n * last_n + constant
}

pub fn is_in_set(constant: Complex<f32>) -> (bool, usize) {
    let mut start = Complex::new(0.0, 0.0);
    for i in 0..50 {
        start = iterate_mandelbrot(start, constant);
        if start.norm_sqr() > 4.0 {
            return (false, i);
        }
    }
    (true, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Complex;

    #[test]
    fn test_mandelbrot_iteration() {
        let start = Complex::new(0.0, 0.0);

        let new_n = iterate_mandelbrot(start, Complex::new(0.0, 1.0));
        let new_n2 = iterate_mandelbrot(new_n, Complex::new(0.0, 1.0));
        println!("{:?}", new_n2);
    }

    #[test]
    fn test_mandelbrot_cycle_check() {
        let answer_in_set = is_in_set(Complex::new(0.0, 1.0));
        let answer_not_in_set = is_in_set(Complex::new(0.0, 2.0));

        assert!(matches!(answer_in_set, (true, _)));
        assert!(matches!(answer_not_in_set, (false, _)));
    }

    #[test]
    fn test_mandelbrot_generation() {
        let mut start_fp = 0.0;
        let mut num = Complex::new(0.0, start_fp);
        for _ in 1..100 {
            let ans = is_in_set(num);
            println!("{:?} | {:?}", num, ans);
            start_fp += 0.1;
            num.im = start_fp;
        }
    }
}
