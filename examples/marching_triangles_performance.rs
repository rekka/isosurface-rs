extern crate isosurface;
extern crate ndarray;

use ndarray::Array;
use isosurface::marching_triangles;

fn main() {
    let n = 256;
    let xs = Array::linspace(-0.5f64, 0.5, n);
    let ys = Array::linspace(-0.5, 0.5, n);

    let dim = (xs.len(), ys.len());

    let u = {
        let mut u = Array::from_elem(dim, 0.);

        for ((i, j), u) in u.indexed_iter_mut() {
            let (x, y) = (xs[i], ys[j]);
            *u = (x * x + y * y).sqrt() - 0.4;
        }
        u
    };

    let mut m = 0;
    for _ in 0..1000 {
        let verts = marching_triangles(u.as_slice().unwrap(), dim, 0.);
        m += verts.len();
    }

    println!("total vert num = {}", m);
}
