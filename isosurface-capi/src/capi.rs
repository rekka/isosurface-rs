use std::slice;
use std::ptr;
use isosurface;
use libc::size_t;

pub struct Isosurface {
    verts: Vec<[f64; 3]>,
    faces: Vec<[u32; 3]>,
    normals: Vec<[f64; 3]>,
    data: Option<Vec<f64>>,
}

ffi_fn!{
    fn marching_tetrahedra(u: *const f64, ni: size_t, nj: size_t, nk: size_t, level: f64)
        -> *const Isosurface {
        let u = unsafe { slice::from_raw_parts(u, ni * nj * nk) };

        let (verts, faces, normals) = isosurface::marching_tetrahedra(u, (ni, nj, nk), level);

        let iso = Isosurface {
            verts, faces, normals, data: None,
        };

        Box::into_raw(Box::new(iso))
    }
}

ffi_fn!{
    fn iso_num_verts(iso: *const Isosurface) -> size_t {
        let iso = unsafe { &*iso };

        iso.verts.len()
    }
}

ffi_fn!{
    fn iso_num_faces(iso: *const Isosurface) -> size_t {
        let iso = unsafe { &*iso };

        iso.faces.len()
    }
}

ffi_fn!{
    fn iso_verts(iso: *const Isosurface) -> *const [f64; 3] {
        let iso = unsafe { &*iso };

        iso.verts.as_ptr()
    }
}

ffi_fn!{
    fn iso_normals(iso: *const Isosurface) -> *const [f64; 3] {
        let iso = unsafe { &*iso };

        iso.normals.as_ptr()
    }
}

ffi_fn!{
    fn iso_faces(iso: *const Isosurface) -> *const [u32; 3] {
        let iso = unsafe { &*iso };

        iso.faces.as_ptr()
    }
}

ffi_fn!{
    fn iso_data(iso: *const Isosurface) -> *const f64 {
        let iso = unsafe { &*iso };

        if let Some(ref data) = iso.data {
            data.as_ptr()
        } else {
            ptr::null()
        }
    }
}

ffi_fn! {
    fn iso_free(re: *const Isosurface) {
        unsafe { Box::from_raw(re as *mut Isosurface); }
    }
}
