/// Find the point on a line `v`--`w` where the given linear function is zero.
///
/// `a` and `b` are the values of the linear function at points `v`, `w`.
#[inline]
fn interpolate(a: f64, b: f64, v: [f64; 3], w: [f64; 3]) -> [f64; 3] {
    let x = a / (a - b);
    [(1. - x) * v[0] + x * w[0], (1. - x) * v[1] + x * w[1], (1. - x) * v[2] + x * w[2]]
}

/// Find the intersection of a level set of a linear function with a tetrahedron.
///
/// The level set is given by its values `u` at vertices `v`. The resulting intersection is added
/// to `verts` (vertex buffer) and `faces` (index buffer into the vertex buffer).
fn tetrahedron(u: [f64; 4],
               v: [[f64; 3]; 4],
               verts: &mut Vec<[f64; 3]>,
               faces: &mut Vec<[u32; 3]>) {
    let u0 = u[0];
    let u1 = u[1];
    let u2 = u[2];
    let u3 = u[3];

    let v0 = v[0];
    let v1 = v[1];
    let v2 = v[2];
    let v3 = v[3];

    // avoid problems with level set going through an edge
    let tiny = 1e-15;

    // START GENERATED: generated by `scripts/tetrahedron_cuts.py`
    if u0 >= 0. {
        let u0 = u0 + tiny;
        if u1 >= 0. {
            let u1 = u1 + tiny;
            if u2 >= 0. {
                let u2 = u2 + tiny;
                if u3 >= 0. {
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u3, u0, v3, v0));
                    verts.push(interpolate(u3, u1, v3, v1));
                    verts.push(interpolate(u3, u2, v3, v2));
                    faces.push([i, i + 1, i + 2]);
                }
            } else {
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u2, u0, v2, v0));
                    verts.push(interpolate(u2, u1, v2, v1));
                    verts.push(interpolate(u2, u3, v2, v3));
                    faces.push([i, i + 1, i + 2]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u2, u0, v2, v0));
                    verts.push(interpolate(u2, u1, v2, v1));
                    verts.push(interpolate(u3, u0, v3, v0));
                    verts.push(interpolate(u3, u1, v3, v1));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                }
            }
        } else {
            if u2 >= 0. {
                let u2 = u2 + tiny;
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u1, u0, v1, v0));
                    verts.push(interpolate(u1, u2, v1, v2));
                    verts.push(interpolate(u1, u3, v1, v3));
                    faces.push([i, i + 1, i + 2]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u1, u0, v1, v0));
                    verts.push(interpolate(u1, u2, v1, v2));
                    verts.push(interpolate(u3, u0, v3, v0));
                    verts.push(interpolate(u3, u2, v3, v2));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                }
            } else {
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u1, u0, v1, v0));
                    verts.push(interpolate(u1, u3, v1, v3));
                    verts.push(interpolate(u2, u0, v2, v0));
                    verts.push(interpolate(u2, u3, v2, v3));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u1, u0, v1, v0));
                    verts.push(interpolate(u2, u0, v2, v0));
                    verts.push(interpolate(u3, u0, v3, v0));
                    faces.push([i, i + 1, i + 2]);
                }
            }
        }
    } else {
        if u1 >= 0. {
            let u1 = u1 + tiny;
            if u2 >= 0. {
                let u2 = u2 + tiny;
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u1, v0, v1));
                    verts.push(interpolate(u0, u2, v0, v2));
                    verts.push(interpolate(u0, u3, v0, v3));
                    faces.push([i, i + 1, i + 2]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u1, v0, v1));
                    verts.push(interpolate(u0, u2, v0, v2));
                    verts.push(interpolate(u3, u1, v3, v1));
                    verts.push(interpolate(u3, u2, v3, v2));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                }
            } else {
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u1, v0, v1));
                    verts.push(interpolate(u0, u3, v0, v3));
                    verts.push(interpolate(u2, u1, v2, v1));
                    verts.push(interpolate(u2, u3, v2, v3));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u1, v0, v1));
                    verts.push(interpolate(u2, u1, v2, v1));
                    verts.push(interpolate(u3, u1, v3, v1));
                    faces.push([i, i + 1, i + 2]);
                }
            }
        } else {
            if u2 >= 0. {
                let u2 = u2 + tiny;
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u2, v0, v2));
                    verts.push(interpolate(u0, u3, v0, v3));
                    verts.push(interpolate(u1, u2, v1, v2));
                    verts.push(interpolate(u1, u3, v1, v3));
                    faces.push([i, i + 1, i + 2]);
                    faces.push([i + 2, i + 1, i + 3]);
                } else {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u2, v0, v2));
                    verts.push(interpolate(u1, u2, v1, v2));
                    verts.push(interpolate(u3, u2, v3, v2));
                    faces.push([i, i + 1, i + 2]);
                }
            } else {
                if u3 >= 0. {
                    let i = verts.len() as u32;
                    verts.push(interpolate(u0, u3, v0, v3));
                    verts.push(interpolate(u1, u3, v1, v3));
                    verts.push(interpolate(u2, u3, v2, v3));
                    faces.push([i, i + 1, i + 2]);
                } else {
                }
            }
        }
    }
    // END GENERATED

}

/// Finds the isosurface of `u` at `level` using the marching tetrahedra algorithm.
///
/// `dim` is the dimension of the array (in C order).
///
/// Returns vertices, faces and normals of the generated triangular mesh.
pub fn marching_tetrahedra(u: &[f64],
                           dim: (usize, usize, usize),
                           level: f64)
                           -> (Vec<[f64; 3]>, Vec<[u32; 3]>, Vec<[f64; 3]>) {

    let (ni, nj, nk) = dim;
    assert_eq!(ni * nj * nk, u.len());

    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<[u32; 3]> = Vec::new();

    // permutations of [0, 1, 2]
    let perms = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

    // inverse permutation (index where the element goes)
    let inv_perms = {
        let mut inv_perms = [[0; 3]; 6];
        for i in 0..6 {
            for j in 0..3 {
                inv_perms[i][perms[i][j]] = j;
            }
        }
        inv_perms
    };

    let strides = [nj * nk, nk, 1];

    for i in 1..ni {
        for j in 1..nj {
            for k in 1..nk {
                // split each cube into 6 tetrahedra, emit triangles
                // vertex position
                let s = (i - 1) * nj * nk + (j - 1) * nk + (k - 1);
                let ps = [(i - 1) as f64, (j - 1) as f64, (k - 1) as f64];

                for (perm, inv_perm) in perms.iter().zip(inv_perms.iter()) {
                    // find tetrahedron data by walking along the edges of a cube
                    // in the order given by the permutation
                    let (us, vs) = {
                        let mut us = [0.; 4];
                        let mut vs = [[0.; 3]; 4];
                        let mut vi = s;
                        let mut vp = ps;
                        us[0] = u[vi] - level;
                        vs[0] = vp;
                        for m in 0..3 {
                            let t = perm[m];
                            vi += strides[t];
                            vp[t] += 1.;
                            us[m + 1] = u[vi] - level;
                            vs[m + 1] = vp;
                        }
                        (us, vs)
                    };

                    tetrahedron(us, vs, &mut verts, &mut faces);
                    // normals
                    let n = [us[1] - us[0], us[2] - us[1], us[3] - us[2]];
                    let n = [n[inv_perm[0]], n[inv_perm[1]], n[inv_perm[2]]];

                    for _ in 0..verts.len() - normals.len() {
                        normals.push(n);
                    }
                }
            }
        }
    }

    (verts, faces, normals)
}
