/// Finds the isosurface of `u` at `level` using the marching tetrahedra algorithm.
///
/// Returns vertices, faces and normals.
pub fn marching_tetrahedra(u: &[f64],
                           dim: (usize, usize, usize),
                           level: f64)
                           -> (Vec<[f64; 3]>, Vec<[u32; 3]>, Vec<[f64; 3]>) {

    let (ni, nj, nk) = dim;
    assert_eq!(ni * nj * nk, u.len());

    (Vec::new(), Vec::new(), Vec::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
