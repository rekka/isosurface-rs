//! Implementations of various isosurface algorithms.
//!
//! See `examples/` for sample code.

mod isoline;
pub use isoline::marching_triangles;

mod isosurface;
pub use isosurface::marching_tetrahedra;
pub use isosurface::marching_tetrahedra_with_data;
