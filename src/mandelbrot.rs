use num::{complex::{self, ComplexFloat}, Complex};

#[inline]
fn iterate_mandelbrot(last_n: Complex<f32>, constant: Complex<f32>) -> Complex<f32> {
    last_n*last_n + constant
}

/// A constant is in the set if it does NOT escape to infinity
pub fn is_in_set(constant: Complex<f32>) -> bool {
    let mut start = Complex::new(0.0, 0.0);
    // 10 iterations to test for cicle
    for _ in 0..10 {
        let next = iterate_mandelbrot(start, constant);
        // println!("last: {:?} | next: {:?}", start, next);
        if next.norm() < start.norm() {
            return true;
        }
        start = next;
    }
    return false;
}


#[cfg(test)]
mod tests {
    use num::Complex;
    use super::*;


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

        assert_eq!(answer_in_set, true);
        assert_eq!(answer_not_in_set, false);
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