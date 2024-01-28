use crate::interpolate::Interpolate;

pub trait Nudge {
    fn nudge(self) -> Self;
}

impl Nudge for f32 {
    fn nudge(self) -> Self {
        self + 1e-15
    }
}

impl Nudge for f64 {
    fn nudge(self) -> Self {
        self + 1e-15
    }
}

/// Find the intersection of the zero level set of a linear function with a tetrahedron.
///
/// The level set is determined by the function values `u` at vertices `v`. The resulting
/// intersection emits a vertex (at most 4), and each triangle emits a face with indices into the
/// emitted vertices.
pub fn tetrahedron<D, T, FV, FF>(u: [D; 4], v: [T; 4], mut emit_vertex: FV, mut emit_face: FF)
where
    D: PartialOrd + Default + Nudge + Copy,
    T: Interpolate<D> + Copy,
    FV: FnMut(T),
    FF: FnMut([u32; 3]),
{
    let u0 = u[0];
    let u1 = u[1];
    let u2 = u[2];
    let u3 = u[3];

    let v0 = v[0];
    let v1 = v[1];
    let v2 = v[2];
    let v3 = v[3];

    let zero = Default::default();

    // START GENERATED: generated by `scripts/tetrahedron_cuts.py`
    if u0 >= zero {
        let u0 = u0.nudge();
        if u1 >= zero {
            let u1 = u1.nudge();
            if u2 >= zero {
                let u2 = u2.nudge();
                if u3 >= zero {
                } else {
                    emit_vertex(v3.interpolate(&v0, u3, u0));
                    emit_vertex(v3.interpolate(&v1, u3, u1));
                    emit_vertex(v3.interpolate(&v2, u3, u2));
                    emit_face([0, 1, 2]);
                }
            } else {
                if u3 >= zero {
                    emit_vertex(v2.interpolate(&v0, u2, u0));
                    emit_vertex(v2.interpolate(&v1, u2, u1));
                    emit_vertex(v2.interpolate(&v3, u2, u3));
                    emit_face([0, 1, 2]);
                } else {
                    emit_vertex(v2.interpolate(&v0, u2, u0));
                    emit_vertex(v2.interpolate(&v1, u2, u1));
                    emit_vertex(v3.interpolate(&v0, u3, u0));
                    emit_vertex(v3.interpolate(&v1, u3, u1));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                }
            }
        } else {
            if u2 >= zero {
                let u2 = u2.nudge();
                if u3 >= zero {
                    emit_vertex(v1.interpolate(&v0, u1, u0));
                    emit_vertex(v1.interpolate(&v2, u1, u2));
                    emit_vertex(v1.interpolate(&v3, u1, u3));
                    emit_face([0, 1, 2]);
                } else {
                    emit_vertex(v1.interpolate(&v0, u1, u0));
                    emit_vertex(v1.interpolate(&v2, u1, u2));
                    emit_vertex(v3.interpolate(&v0, u3, u0));
                    emit_vertex(v3.interpolate(&v2, u3, u2));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                }
            } else {
                if u3 >= zero {
                    emit_vertex(v1.interpolate(&v0, u1, u0));
                    emit_vertex(v1.interpolate(&v3, u1, u3));
                    emit_vertex(v2.interpolate(&v0, u2, u0));
                    emit_vertex(v2.interpolate(&v3, u2, u3));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                } else {
                    emit_vertex(v1.interpolate(&v0, u1, u0));
                    emit_vertex(v2.interpolate(&v0, u2, u0));
                    emit_vertex(v3.interpolate(&v0, u3, u0));
                    emit_face([0, 1, 2]);
                }
            }
        }
    } else {
        if u1 >= zero {
            let u1 = u1.nudge();
            if u2 >= zero {
                let u2 = u2.nudge();
                if u3 >= zero {
                    emit_vertex(v0.interpolate(&v1, u0, u1));
                    emit_vertex(v0.interpolate(&v2, u0, u2));
                    emit_vertex(v0.interpolate(&v3, u0, u3));
                    emit_face([0, 1, 2]);
                } else {
                    emit_vertex(v0.interpolate(&v1, u0, u1));
                    emit_vertex(v0.interpolate(&v2, u0, u2));
                    emit_vertex(v3.interpolate(&v1, u3, u1));
                    emit_vertex(v3.interpolate(&v2, u3, u2));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                }
            } else {
                if u3 >= zero {
                    emit_vertex(v0.interpolate(&v1, u0, u1));
                    emit_vertex(v0.interpolate(&v3, u0, u3));
                    emit_vertex(v2.interpolate(&v1, u2, u1));
                    emit_vertex(v2.interpolate(&v3, u2, u3));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                } else {
                    emit_vertex(v0.interpolate(&v1, u0, u1));
                    emit_vertex(v2.interpolate(&v1, u2, u1));
                    emit_vertex(v3.interpolate(&v1, u3, u1));
                    emit_face([0, 1, 2]);
                }
            }
        } else {
            if u2 >= zero {
                let u2 = u2.nudge();
                if u3 >= zero {
                    emit_vertex(v0.interpolate(&v2, u0, u2));
                    emit_vertex(v0.interpolate(&v3, u0, u3));
                    emit_vertex(v1.interpolate(&v2, u1, u2));
                    emit_vertex(v1.interpolate(&v3, u1, u3));
                    emit_face([0, 1, 2]);
                    emit_face([2, 1, 3]);
                } else {
                    emit_vertex(v0.interpolate(&v2, u0, u2));
                    emit_vertex(v1.interpolate(&v2, u1, u2));
                    emit_vertex(v3.interpolate(&v2, u3, u2));
                    emit_face([0, 1, 2]);
                }
            } else {
                if u3 >= zero {
                    emit_vertex(v0.interpolate(&v3, u0, u3));
                    emit_vertex(v1.interpolate(&v3, u1, u3));
                    emit_vertex(v2.interpolate(&v3, u2, u3));
                    emit_face([0, 1, 2]);
                } else {
                }
            }
        }
    }
    // END GENERATED
}

/// Finds the isosurface at `level` of a function given by its values `u` on a regular grid  using
/// the marching tetrahedra algorithm.
///
/// `dim` is the dimension of the array `u` assumed to be in _row-major order_ (C order).
///
/// Returns vertices, faces and normals of the generated triangular mesh.
pub fn marching_tetrahedra<D>(
    u: &[D],
    dim: (usize, usize, usize),
    level: D,
) -> (Vec<[D; 3]>, Vec<[u32; 3]>, Vec<[D; 3]>)
where
    D: Interpolate<D>
        + From<f32>
        + PartialOrd
        + std::ops::Sub<D, Output = D>
        + Copy
        + Default
        + Nudge
        + std::ops::Add<D, Output = D>,
{
    let (verts, faces, normals, _) =
        marching_tetrahedra_with_data(u, dim, level, &vec![(); u.len()]);

    (verts, faces, normals)
}

/// As `marching_tetrahedra`, but also linearly interpolates the provided data for each vertex.
pub fn marching_tetrahedra_with_data<D, T>(
    u: &[D],
    dim: (usize, usize, usize),
    level: D,
    data: &[T],
) -> (Vec<[D; 3]>, Vec<[u32; 3]>, Vec<[D; 3]>, Vec<T>)
where
    D: Interpolate<D>
        + From<f32>
        + PartialOrd
        + std::ops::Sub<D, Output = D>
        + Copy
        + Default
        + Nudge
        + std::ops::Add<D, Output = D>,
    T: Interpolate<D> + Default + Copy,
{
    let (ni, nj, nk) = dim;
    assert_eq!(ni * nj * nk, u.len());
    assert_eq!(ni * nj * nk, data.len());

    let mut verts: Vec<[D; 3]> = Vec::new();
    let mut normals: Vec<[D; 3]> = Vec::new();
    let mut faces: Vec<[u32; 3]> = Vec::new();
    let mut interp_data: Vec<T> = Vec::new();

    // permutations of [0, 1, 2]
    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let strides = [nj * nk, nk, 1];

    let vert_offsets = {
        let mut vert_offsets = [0; 8];

        let mut c = 0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    vert_offsets[c] = i * nj * nk + j * nk + k;
                    c += 1;
                }
            }
        }
        vert_offsets
    };

    for i in 1..ni {
        for j in 1..nj {
            for k in 1..nk {
                // split each cube into 6 tetrahedra, emit triangles
                // vertex position
                let s = (i - 1) * nj * nk + (j - 1) * nk + (k - 1);
                let ps = [
                    D::from((i - 1) as f32),
                    D::from((j - 1) as f32),
                    D::from((k - 1) as f32),
                ];

                let n_above = vert_offsets
                    .iter()
                    .filter(|&offset| u[s + offset] >= level)
                    .count();

                if n_above == 0 || n_above == 8 {
                    continue;
                }

                for perm in perms {
                    // find tetrahedron data by walking along the edges of a cube
                    // in the order given by the permutation
                    let (us, vs) = {
                        let mut us = [D::from(0.); 4];
                        let mut vs = [([D::from(0.); 3], T::default()); 4];
                        let mut vi = s;
                        let mut vp = ps;
                        us[0] = u[vi] - level;
                        vs[0] = (vp, data[vi]);
                        for m in 0..3 {
                            let t = perm[m];
                            vi += strides[t];
                            vp[t] = vp[t] + D::from(1.);
                            us[m + 1] = u[vi] - level;
                            vs[m + 1] = (vp, data[vi]);
                        }
                        (us, vs)
                    };

                    let cur = verts.len() as u32;
                    tetrahedron(
                        us,
                        vs,
                        |(v, d)| {
                            verts.push(v);
                            interp_data.push(d);
                        },
                        |f| {
                            faces.push([f[0] + cur, f[1] + cur, f[2] + cur]);
                        },
                    );

                    // normals
                    let mut n = [D::from(0.); 3];
                    for i in 0..3 {
                        // invert the permutation
                        n[perm[i]] = us[i + 1] - us[i];
                    }

                    for _ in 0..verts.len() - normals.len() {
                        normals.push(n);
                    }
                }
            }
        }
    }

    (verts, faces, normals, interp_data)
}

/// Emits the triangles of the level set of u intersecting tetrahedra of the mesh.
///
/// For each triangle, emits the coordinates of its vertices and the linearly interpolated data
/// at these vertices.
///
/// The coordinate system is chosen so that the node (i, j, k) with index i * dim.1 * dim.2 + j *
/// dim.2 + k has coordinate
/// (i, j, k).
pub fn marching_tetrahedra_with_data_emit<F, D>(
    u: &[f64],
    data: &[D],
    dim: (usize, usize, usize),
    level: f64,
    mut emit: F,
) where
    F: FnMut([[f64; 3]; 3], [D; 3]),
    D: Interpolate<f64> + Default + Copy,
{
    let (ni, nj, nk) = dim;
    assert_eq!(ni * nj * nk, u.len());
    assert_eq!(ni * nj * nk, data.len());

    let mut verts: Vec<[f64; 3]> = Vec::with_capacity(4);
    let mut faces: Vec<[u32; 3]> = Vec::with_capacity(2);
    let mut interp_data: Vec<D> = Vec::with_capacity(4);

    // permutations of [0, 1, 2]
    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    let strides = [nj * nk, nk, 1];

    let vert_offsets = {
        let mut vert_offsets = [0; 8];

        let mut c = 0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    vert_offsets[c] = i * nj * nk + j * nk + k;
                    c += 1;
                }
            }
        }
        vert_offsets
    };

    for i in 1..ni {
        for j in 1..nj {
            for k in 1..nk {
                // split each cube into 6 tetrahedra, emit triangles
                // vertex position
                let s = (i - 1) * nj * nk + (j - 1) * nk + (k - 1);
                let ps = [(i - 1) as f64, (j - 1) as f64, (k - 1) as f64];

                let n_above = vert_offsets
                    .iter()
                    .filter(|&offset| u[s + offset] >= level)
                    .count();

                if n_above == 0 || n_above == 8 {
                    continue;
                }

                for perm in &perms {
                    // find tetrahedron data by walking along the edges of a cube
                    // in the order given by the permutation
                    let (us, vs) = {
                        let mut us = [0.; 4];
                        let mut vs = [([0.; 3], D::default()); 4];
                        let mut vi = s;
                        let mut vp = ps;
                        us[0] = u[vi] - level;
                        vs[0] = (vp, data[vi]);
                        for m in 0..3 {
                            let t = perm[m];
                            vi += strides[t];
                            vp[t] += 1.;
                            us[m + 1] = u[vi] - level;
                            vs[m + 1] = (vp, data[vi]);
                        }
                        (us, vs)
                    };

                    verts.clear();
                    interp_data.clear();
                    faces.clear();
                    tetrahedron(
                        us,
                        vs,
                        |(v, d)| {
                            verts.push(v);
                            interp_data.push(d);
                        },
                        |f| {
                            faces.push(f);
                        },
                    );
                    for f in &faces {
                        emit(
                            [
                                verts[f[0] as usize],
                                verts[f[1] as usize],
                                verts[f[2] as usize],
                            ],
                            [
                                interp_data[f[0] as usize],
                                interp_data[f[1] as usize],
                                interp_data[f[2] as usize],
                            ],
                        );
                    }
                }
            }
        }
    }
}

/// Marching tetrahedra on a axes aligned cube; linearly interpolates the provided data for each
/// vertex.
///
/// `corner` is the position of the cube corner with smallest coordinates, so that other corners
/// can be produces by adding `size`.
///
/// The ordering of `u` and `data` is so that 
///
/// - `u[i]` and `u[i + 4]` are at corners differing in _x_ (first) coordinate,
/// - `u[i]` and `u[i + 2]` differ by the _y_ (middle) coordinate,
/// - `u[i]` and `u[i + 1]` differ by the _z_ (last) coordinate.
///
/// `emit_vertex` emits the vertex coord, normal and interpolated data.
///
/// `emit_face` emits the 3 indices of the face vertices.
/// The emitted face indices are relative to `vertext_index_offset`.
pub fn marching_tetrahedra_with_data_cube<D, T>(
    corner: [D; 3],
    size: D,
    u: [D; 8],
    level: D,
    data: [T; 8],
    mut vertext_index_offset: u32,
    mut emit_vertex: impl FnMut([D; 3], [D; 3], T),
    mut emit_face: impl FnMut([u32; 3]),
) where
    D: Interpolate<D>
        + From<f32>
        + PartialOrd
        + std::ops::Sub<D, Output = D>
        + Copy
        + Default
        + Nudge
        + std::ops::Add<D, Output = D>,
    T: Interpolate<D> + Default + Copy,
{
    // permutations of [0, 1, 2]
    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    // return early if the cube does not intersect the level set
    let n_above = u
        .into_iter()
        .filter(|&u| u >= level)
        .count();

    if n_above == 0 || n_above == 8 {
        return;
    }

    for perm in perms {
        // find tetrahedron data by walking along the edges of a cube
        // in the order given by the permutation
        let (us, vs) = {
            let mut us = [D::from(0.); 4];
            let mut vs = [([D::from(0.); 3], T::default()); 4];
            let mut vi = 0;
            let mut vp = corner;
            us[0] = u[vi] - level;
            vs[0] = (vp, data[vi]);
            for m in 0..3 {
                let t = perm[m];
                vi += 1 << (2 - t);
                vp[t] = vp[t] + size;
                us[m + 1] = u[vi] - level;
                vs[m + 1] = (vp, data[vi]);
            }
            (us, vs)
        };

        // normal
        let mut n = [D::from(0.); 3];
        for i in 0..3 {
            // invert the permutation
            n[perm[i]] = us[i + 1] - us[i];
        }

        let cur = vertext_index_offset;
        tetrahedron(
            us,
            vs,
            |(v, d)| {
                emit_vertex(v, n, d);
                vertext_index_offset += 1;
            },
            |f| {
                emit_face([f[0] + cur, f[1] + cur, f[2] + cur]);
            },
        );

    }
}
