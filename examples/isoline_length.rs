extern crate isosurface;
extern crate ndarray;

use ndarray::Array;
use isosurface::marching_triangles_with_data_emit;

fn main() {
    let n = 16;
    let xs = Array::linspace(-0.5f64, 0.5, n);
    let ys = Array::linspace(-0.5, 0.5, n);

    let dim = (xs.len(), ys.len());

    let u = {
        let mut u = Array::from_elem(dim, 0.);

        for ((i, j), u) in u.indexed_iter_mut() {
            let (x, y) = (xs[i], ys[j]);
            *u = (x * x + y * y).sqrt();
        }
        u
    };

    // create dummy data, should not allocate anything
    let data = Array::from_elem(dim, ());

    let level = 0.3;
    let mut length = 0.;
    marching_triangles_with_data_emit(u.as_slice().unwrap(), data.as_slice().unwrap(), dim, level,
    |c, _| {
        length += (c[0][0] - c[1][0]).hypot(c[0][1] - c[1][1]);
    });

    println!("Isoline length = {}", length / (n - 1) as f64);
    println!("Expected length = {}", 2. * std::f64::consts::PI * level);
}
