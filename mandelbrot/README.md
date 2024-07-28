# Mandelbrot set renderer

This nannou app renders an image of the mandelbrot set and then draws it as a texture. 
You can zoom into the set by right-clicking, and unzoom by left-clicking.

Because of the limitations of f64 arithmetic the set loses resolution quickly when zooming.

![Mandelbrot](../assets/mandelbrot.png)
![Mandelbrot](../assets/mandelbrot_zoomed.png)
