//! Implementations of various isosurface algorithms.
//!
//! See `examples/` for sample code.

mod isoline;
pub use isoline::marching_triangles;
pub use isoline::marching_triangles_with_data_emit;
pub use isoline::Isoline;

mod isosurface;
pub use isosurface::marching_tetrahedra;
pub use isosurface::marching_tetrahedra_with_data;

mod interpolate;
