#[macro_use]
extern crate glium;
extern crate isosurface;
extern crate ndarray;

use ndarray::Array;
use isosurface::marching_tetrahedra;
use glium::glutin;

mod support;

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

    let (verts, faces, normals) = marching_tetrahedra(u.as_slice().unwrap(), dim, 0.);

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                        .with_depth_buffer(24)
                        .build_glium().unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let positions = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        let verts: Vec<_> = verts.iter().map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32], color: [1.0, 0.0, 0.0] }).collect();
        glium::VertexBuffer::new(&display,
            &verts
        ).unwrap()
    };

    let normals = {
        #[derive(Copy, Clone)]
        pub struct Normal {
            normal: (f32, f32, f32)
        }

        implement_vertex!(Normal, normal);

        let normals: Vec<_> = normals.iter().map(|v| Normal { normal: (0., 0., 1.) }).collect();

        glium::VertexBuffer::new(&display, &normals).unwrap()
    };


    // building the index buffer
    let indices = {
        let mut buf = Vec::with_capacity(faces.len() * 3);
        for i in faces {
            buf.extend_from_slice(&i);
        }
        glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                               &buf).unwrap()
    };

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        out vec3 v_position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        uniform vec3 u_light;

        const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
        const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();

    let mut camera = support::camera::CameraState::new();
    let mut wireframe = false;
    loop {
        camera.update();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let scale = 0.1;
        let model = [
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        let light = [1.4, 0.4, -0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: if wireframe { glium::draw_parameters::PolygonMode::Line } else { glium::draw_parameters::PolygonMode::Fill },
            line_width: Some(3.),
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: camera.get_view(), perspective: camera.get_perspective(), u_light: light },
                    &params).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::F)) => wireframe = !wireframe,
                ev => camera.process_input(&ev)
            }
        }
    }
}

