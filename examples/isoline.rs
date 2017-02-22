extern crate isosurface;
extern crate gnuplot;
extern crate ndarray;

use ndarray::Array;
use isosurface::marching_triangles;
use gnuplot::{Figure, AxesCommon};

fn main() {
    let n = 6;
    let xs = Array::linspace(-0.5f64, 0.5, n);
    let ys = Array::linspace(-0.5, 0.5, n);

    let dim = (xs.len(), ys.len());

    let u = {
        let mut u = Array::from_elem(dim, 0.);

        for ((i, j), u) in u.indexed_iter_mut() {
            let (x, y) = (xs[i], ys[j]);
            *u = (x * x + y * y).sqrt() - 0.5;
        }
        u
    };

    let mut fg = Figure::new();

    {
        let mut axes = fg.axes2d();
        axes.set_aspect_ratio(gnuplot::AutoOption::Fix(1.));

        let grid_opt = [gnuplot::PlotOption::LineStyle(gnuplot::DashType::DotDash)];

        for x in 0..n {
            axes.lines(&[x, x], &[0, n - 1], &grid_opt);
            axes.lines(&[0, n - 1], &[x, x], &grid_opt);
            axes.lines(&[0, x], &[n - 1 - x, n - 1], &grid_opt);
            axes.lines(&[x, n - 1], &[0, n - 1 - x], &grid_opt);
        }

        for &level in &[-0.1, 0., 0.1] {
            let verts = marching_triangles(u.as_slice().unwrap(), dim, level);

            println!("level {} has {} connected component(s)", level, verts.len());

            for line in verts {
                axes.lines(line.iter().map(|p| p[0]), line.iter().map(|p| p[1]), &[]);
            }
        }
    }
    fg.show();
}
