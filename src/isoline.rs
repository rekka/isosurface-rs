use std::collections::HashMap;
use interpolate::Interpolate;

#[derive(Clone, Debug)]
pub struct Isoline {
    verts: Vec<[f64; 2]>,
    components: Vec<usize>,
}

impl Isoline {
    pub fn components(&self) -> Components {
        Components {
            isoline: self,
            component: 0,
        }
    }
}

pub struct Components<'a> {
    isoline: &'a Isoline,
    component: usize,
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a [[f64; 2]];

    fn next(&mut self) -> Option<Self::Item> {
        let comps = &self.isoline.components;
        if self.component + 1 < comps.len() {
            let c = self.component;
            self.component += 1;
            return Some(&self.isoline.verts[comps[c]..comps[c + 1]]);
        }
        None
    }
}

impl<'a> ExactSizeIterator for Components<'a> {
    fn len(&self) -> usize {
        self.isoline.components.len() - self.component - 1
    }
}

/// Finds the isoline (level set) at `level` of a function given by its values `u` on a regular
/// grid using a marching triangles algorithm.
///
/// `dim` is the dimension of the array `u` assumed to be in _row-major order_ (C order).
///
/// Each square of the grid is split into two triangles, and the function is assumed to be  linear
/// on each triangle.
///
/// Returns a `Vec` of all connected components of the isoline.
pub fn marching_triangles(u: &[f64], dim: (usize, usize), level: f64) -> Isoline {
    let (ni, nj) = dim;
    assert_eq!(ni * nj, u.len());

    let mut verts: Vec<[f64; 2]> = Vec::new();
    let mut components: Vec<usize> = Vec::new();

    // first find all the edges connecting the sides of the triangles
    let mut edges: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let (si, sj) = (nj, 1);

    // vertices 3, 2, 1, 0
    // index is (s - vs[i])
    let vs = [0, si, sj, si + sj];
    // six sides of the two triangles within a square (diagonal is duplicated)
    // connecting vertices 0,...,3
    let es = [[0, 2], [2, 3], [0, 3], [0, 3], [0, 1], [1, 3]];

    let bit = |k: u32, bit: u32| (k & (1 << bit)) >> bit;

    // a map between a bit mask and the list between connected triangle sides
    let table = {
        let mut table = [[6; 4]; 16];

        for i in 0..16 {
            let mut c = 0;
            for (ei, edge) in es.iter().enumerate() {
                if bit(i, edge[0] as u32) != bit(i, edge[1] as u32) {
                    table[i as usize][c] = ei;
                    c += 1;
                }
            }
        }

        table
    };

    // generate the map of edges connected by the level set
    for i in 1..ni {
        for j in 1..nj {
            let s = i * si + j * sj;

            let mask = {
                let mut mask = 0;

                for &sd in vs.iter() {
                    mask = (mask << 1) + (if u[s - sd] >= level { 1 } else { 0 });
                }

                mask
            };

            for edge in table[mask].chunks(2).take_while(|c| c[0] < 6) {
                let a = (s, edge[0]);
                let b = (s, edge[1]);

                edges.insert(a, b);
                edges.insert(b, a);
            }
        }
    }

    // each line is a side of two triangles, this is a mapping between them
    let dual = |(s, ei)| match ei {
        0 => (s - si, 5),
        1 => (s + sj, 4),
        2 => (s, 3),
        3 => (s, 2),
        4 => (s - sj, 1),
        5 => (s + si, 0),
        _ => unreachable!(),
    };

    // finds the intersection between the isoline and an edge
    let to_coord = |(s, ei): (usize, usize)| {
        let s1 = s - vs[3 - es[ei][0]];
        let s2 = s - vs[3 - es[ei][1]];

        let coord = |s| {
            let div = s as f64 * (1. / nj as f64);

            [div.trunc(), div.fract() * nj as f64]
        };

        coord(s1).interpolate(&coord(s2), u[s1] - level, u[s2] - level)
    };

    // find connected components in the edge graph
    while !edges.is_empty() {
        let start = *edges.iter().next().unwrap().0;

        let comp_begin = verts.len();
        components.push(comp_begin);

        verts.push(to_coord(start));

        let trace = |start, edges: &mut HashMap<_, _>, verts: &mut Vec<_>| {
            let mut next = start;

            while let Some(other) = edges.remove(&next) {
                edges.remove(&other);

                next = dual(other);

                verts.push(to_coord(next));
            }
        };

        trace(start, &mut edges, &mut verts);

        let start = dual(start);

        // if the component is not a closed curve, we might have to walk in the other direction
        // from the starting side to find the full component
        if edges.contains_key(&start) {
            verts[comp_begin..].reverse();

            trace(start, &mut edges, &mut verts);
        }
    }

    components.push(verts.len());

    Isoline {
        verts: verts,
        components: components,
    }
}

/// Emits indices of edges of the triangular mesh connect by the `level` level set curve of
/// function given by values `u` at the nodes.
pub fn marching_triangles_emit_connected_edges<F>(
    u: &[f64],
    dim: (usize, usize),
    level: f64,
    mut emit: F,
) where
    F: FnMut((usize, usize), (usize, usize)),
{
    let (ni, nj) = dim;
    assert_eq!(ni * nj, u.len());

    let (si, sj) = (nj, 1);

    // vertices 3, 2, 1, 0
    // index is (s - vs[i])
    let vs = [0, si, sj, si + sj];
    // six sides of the two triangles within a square (diagonal is duplicated)
    // connecting vertices 0,...,3
    let es = [[0, 2], [2, 3], [0, 3], [0, 3], [0, 1], [1, 3]];

    let bit = |k: u32, bit: u32| (k & (1 << bit)) >> bit;

    // a map between a bit mask and the list between connected triangle sides
    let table = {
        let mut table = [[6; 4]; 16];

        for i in 0..16 {
            let mut c = 0;
            for (ei, edge) in es.iter().enumerate() {
                if bit(i, edge[0] as u32) != bit(i, edge[1] as u32) {
                    table[i as usize][c] = ei;
                    c += 1;
                }
            }
        }

        table
    };

    // emit indices of connected edges
    for i in 1..ni {
        for j in 1..nj {
            let s = i * si + j * sj;

            let mask = {
                let mut mask = 0;

                for &sd in vs.iter() {
                    mask = (mask << 1) + (if u[s - sd] >= level { 1 } else { 0 });
                }

                mask
            };

            for edge in table[mask].chunks(2).take_while(|c| c[0] < 6) {
                let a = (s, edge[0]);
                let b = (s, edge[1]);

                emit(a, b);
            }
        }
    }
}

/// Emit the line segments of the level set of u intersecting triangles of the mesh.
///
/// For each segment, emits the coordinates of its endpoints and the linearly interpolated data
/// there.
///
/// The coordinate system is chosen so that the node (i, j) with index i * dim.1 + j has coordinate
/// (i, j).
pub fn marching_triangles_with_data_emit<F, D>(
    u: &[f64],
    data: &[D],
    dim: (usize, usize),
    level: f64,
    mut emit: F,
) where
    F: FnMut([[f64; 2]; 2], [D; 2]),
    D: Interpolate<f64>,
{
    let (ni, nj) = dim;
    assert_eq!(ni * nj, u.len());
    assert_eq!(ni * nj, data.len());
    let (si, sj) = (nj, 1);
    let vs = [0, si, sj, si + sj];
    // six sides of the two triangles within a square (diagonal is duplicated)
    // connecting vertices 0,...,3
    let es = [[0, 2], [2, 3], [0, 3], [0, 3], [0, 1], [1, 3]];
    // finds the intersection between the level set and an edge
    let to_coord_with_data = |(s, ei): (usize, usize)| {
        let s1 = s - vs[3 - es[ei][0]];
        let s2 = s - vs[3 - es[ei][1]];

        let coord = |s| {
            let div = s as f64 * (1. / nj as f64);

            [div.trunc(), div.fract() * nj as f64]
        };

        let x = u[s1] - level;
        let y = u[s2] - level;
        (
            coord(s1).interpolate(&coord(s2), x, y),
            data[s1].interpolate(&data[s2], x, y),
        )
    };

    marching_triangles_emit_connected_edges(u, dim, level, |a, b| {
        let (c0, d0) = to_coord_with_data(a);
        let (c1, d1) = to_coord_with_data(b);
        emit([c0, c1], [d0, d1]);
    });
}
