use crate::engine::visual::shader::Shader;
use std::collections::HashMap;

use web_sys::WebGlRenderingContext as GL;
use crate::WebGlProgram;

pub enum AttrSize {
    Scaler = 1,
    Vec2 = 2,
    Vec3 = 3,
    Vec4 = 4,
    Mat4 = 16,
}

pub enum AttrType {
    Int,
    Float,
}

pub enum AttrData {
    Integer(Vec<i32>),
    Float(Vec<f32>),
}

pub struct UniformInfo {
    index: usize,
    size:  AttrSize,
    data:  Vec<AttrEle>,
}

pub enum AttrEle {
    Int(i32),
    Float(f32),
}

#[derive(Copy, Clone)]
pub enum Uniform {
    Int(i32),
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
}

pub struct Material {
    shader: Shader,
    uniforms: Vec<(Uniform, usize)>,
    uniforms_named: HashMap<&'static str, usize>
}

impl Material {
    pub fn new(gl: &GL,
               sources: Vec<(u32, &str)>,
               uniform_dna: Vec<(&'static str, Uniform)>) -> Result<Self, String> {
        let names: Vec<&'static str> = uniform_dna.iter().map(|u| u.0).collect();
        let shader: Shader = Shader::create(gl, sources, names.as_slice())?;

        let mut named = HashMap::new();

        let uniforms = uniform_dna.iter().map(|u| {
            let index = shader.uniforms.get(u.0).clone().unwrap(); //TODO: Fix this to give error
            named.insert(u.0, index.clone());
            (u.1, index.clone())
        }).collect();

        Ok(Self {
            shader,
            uniforms,
            uniforms_named: named,
        })
    }

    pub fn bind(&self, gl: &GL){
        gl.use_program(Some(self.shader.program()));
        for (uniform, loc) in self.uniforms.iter() {
            match uniform {
                Uniform::Int(u) => {
                    self.shader.uniform_1i(gl, u.clone(), loc.clone())
                }
                Uniform::Float(u) => {
                    self.shader.uniform_1f(gl, u.clone(), loc.clone())
                }
                Uniform::Vec2(u) => {
                    self.shader.uniform2fv_with_f32_array(gl, u, loc.clone())
                }
                Uniform::Vec3(u) => {
                    self.shader.uniform3fv_with_f32_array(gl, u, loc.clone())
                }
                Uniform::Vec4(u) => {
                    self.shader.uniform4fv_with_f32_array(gl, u, loc.clone())
                }
                Uniform::Mat4(u) => {
                    self.shader.uniform_mat(gl, u, loc.clone())
                }
            }
        }
    }
}

