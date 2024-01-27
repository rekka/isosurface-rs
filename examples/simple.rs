extern crate isosurface;
extern crate ndarray;

use isosurface::marching_tetrahedra;
use ndarray::Array;

fn main() {
    let xs = Array::linspace(-0.5, 0.5, 3);
    let ys = Array::linspace(-0.5, 0.5, 3);
    let zs = Array::linspace(-0.5, 0.5, 3);

    let dim = (xs.len(), ys.len(), zs.len());

    let u = {
        let mut u = Array::from_elem(dim, 0.);

        for ((i, j, k), u) in u.indexed_iter_mut() {
            let (x, y, z) = (xs[i], ys[j], zs[k]);
            *u = x * x + y * y + z * z - 0.3;
        }
        u
    };

    println!("{}", u);

    let (verts, faces, normals) = marching_tetrahedra(u.as_slice().unwrap(), dim, 0.);

    println!("{:?}", verts);
    println!("{:?}", faces);
    println!("{:?}", normals);
}
