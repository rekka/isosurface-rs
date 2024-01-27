//! Implementations of isoline and isosurface algorithms based on the marching tetrahedra
//! algorithm.
//!
//! A. Doi and A. Koide, _An Efficient Method of Triangulating Equi-Valued Surfaces by Using
//! Tetrahedral Cells_, IEICE TRANSACTIONS on Information and Systems **E74-D** (1991), no. 1,
//! 214--224.
//!
//! See `examples/` for sample code.

mod isoline;
pub use isoline::marching_triangles;
pub use isoline::marching_triangles_with_data_emit;
pub use isoline::Isoline;

mod isosurface;
pub use isosurface::marching_tetrahedra;
pub use isosurface::marching_tetrahedra_with_data;
pub use isosurface::marching_tetrahedra_with_data_emit;
pub use isosurface::tetrahedron;

mod interpolate;
