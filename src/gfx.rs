use crate::game::Game;

use glium::texture::UnsignedTexture3d;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use na::vector;
use std::fs;
use std::path::Path;

use crate::uniforms::AsGPUResource;

pub const SHADER_PATH_NAME: &str = "src/shaders";

pub struct DenseCartesianUniforms {
    pub sdf: UnsignedTexture3d,
    pub voxels: UnsignedTexture3d,
}

pub struct DenseCartesianRenderer {
    pub uniforms: DenseCartesianUniforms,
}

impl DenseCartesianRenderer {
    fn update_uniforms(&mut self, facade: &dyn glium::backend::Facade, game: &Game) {
        self.uniforms.sdf = game.world.sdf.as_gpu_resource(facade);
        self.uniforms.voxels = game.world.voxels.as_gpu_resource(facade);
    }

    pub fn draw(
        &mut self,
        display: &glium::Display,
        vertex_buffer: &glium::VertexBuffer<attrib::Vertex>,
        index_buffer: &glium::IndexBuffer<u16>,
        program: &glium::Program,
        game: &Game,
    ) {
        let ref cam = game.camera;

        // up vector in world coordinates
        let up = vector![0.0, 1.0, 0.0];

        // camera rotation matrix
        let cam_rot = na::Rotation3::face_towards(&cam.dir, &up);

        let cam_pos = {
            let cv = cam.pos.data.0;
            (cv[0][0], cv[0][1], cv[0][2])
        };

        let fov_h: f32 = 45.0;
        let near = 1.0 / (fov_h / 2.0).tan();
        let aspect_ratio: f32 = {
            let (w, h) = display.get_framebuffer_dimensions();
            (w as f32) / (h as f32)
        };

        self.update_uniforms(display, game);

        let ref uniforms = uniform! {
            cam_pos: cam_pos,
            cam_rot: cam_rot.matrix().data.0,
            near: near,
            time: game.time_elapsed.as_secs_f32(),
            aspect_ratio: aspect_ratio,
            sdf_data: &self.uniforms.sdf,
            voxels: &self.uniforms.voxels,
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                vertex_buffer,
                index_buffer,
                program,
                uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}

/// Loads a shader by file path
pub fn load_shader(
    display: &glium::Display,
    name: &str,
) -> Result<glium::Program, glium::ProgramCreationError> {
    let shader_path = Path::new(SHADER_PATH_NAME);

    let vert_fname = format!("{}.vert", name);
    let vert_path = shader_path.join(vert_fname);
    let vert_src = fs::read_to_string(&vert_path)
        .expect(&format!("Unable to read vertex shader: {:?}", &vert_path));

    let frag_fname = format!("{}.frag", name);
    let frag_path = shader_path.join(frag_fname);
    let frag_src = fs::read_to_string(&frag_path)
        .expect(&format!("Unable to read fragment shader: {:?}", &frag_path));

    glium::Program::from_source(display, &vert_src, &frag_src, None)
}

pub mod attrib {
    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 2],
        pub color: [f32; 3],
    }
    implement_vertex!(Vertex, position, color);
}
