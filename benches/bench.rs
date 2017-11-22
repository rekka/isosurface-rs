#![feature(test)]

extern crate isosurface;
extern crate ndarray;
extern crate test;

use ndarray::Array;
use isosurface::marching_tetrahedra;
use test::Bencher;


#[bench]
fn marching_tetrahedra32(b: &mut Bencher) {
    let res = 32;
    let xs = Array::linspace(-0.5f64, 0.5, res);
    let ys = Array::linspace(-0.5, 0.5, res);
    let zs = Array::linspace(-0.5, 0.5, res);

    let dim = (xs.len(), ys.len(), zs.len());

    let u = {
        let mut u = Array::from_elem(dim, 0.);

        let r = 0.4;
        for ((i, j, k), u) in u.indexed_iter_mut() {
            let (x, y, z) = (xs[i], ys[j], zs[k]);
            *u = (x * x + y * y + z * z).sqrt() - r;
        }
        u
    };

    let level = 0.0;

    b.iter(|| {
        marching_tetrahedra(u.as_slice().unwrap(), dim, level);
    });
}
