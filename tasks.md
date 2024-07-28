# Random notes:

task 1: monte-carlo pi simulation

task 2: logistic map graph

task 3: mandelbrot set

task4 : emergent behavior

Mandelbrot definitions
f(z) = z^2 + c
...
f(z_n) = z_n-1^2 + c

The Mandelbrot set consists of all of those (complex) c-values for which the corresponding orbit of 0 under x2 + c does not escape to infinity.

Iterator cheat-sheet:
iter(), which iterates over &T.
iter_mut(), which iterates over &mut T.
into_iter(), which iterates over T.


https://www.karlsims.com/julia.html


Normalized Iteration Count:
This method normalizes the iteration count based on the logarithm of the escape radius.

let log_zn = z.norm().ln() / 2.0;
let nu = (log_zn / 2.0_f64.ln()).ln() / 2.0_f64.ln();
let smooth_i = iterations as f64 + 1.0 - nu;

---

Histogram Coloring:
This technique distributes colors more evenly across the image by creating a histogram of iteration counts.
---

// First pass: count iterations
let mut histogram = vec![0; max_iterations];
for pixel in pixels {
    histogram[pixel.iterations] += 1;
}

// Second pass: calculate total
let total: u32 = histogram.iter().sum();

// Third pass: normalize and color
let mut hue = 0.0;
for pixel in pixels {
    for i in 0..pixel.iterations {
        hue += histogram[i] as f64 / total as f64;
    }
    pixel.color = hue_to_rgb(hue);
}
---

Distance Estimation:
This method uses an estimate of the distance to the Mandelbrot set boundary for smoother coloring.

let de = 2.0 * z.norm() * z.norm().ln() / z.norm_sqr().ln();
let smooth_i = iterations as f64 - de.ln() / 2.0_f64.ln();

---

Continuous Dwell:
This technique provides a continuous transition between iteration counts.

let smooth_i = iterations as f64 + 1.0 - (z.norm().ln() / 2.0_f64.ln()).ln() / 2.0_f64.ln();

---

Exterior Distance Estimation:
This method provides smoother coloring in the areas outside the Mandelbrot set.

let de = (z.norm().ln() * z.norm()) / z.norm_sqr().derivative().norm();
let smooth_i = iterations as f64 + 1.0 - de.ln() / 2.0_f64.ln();