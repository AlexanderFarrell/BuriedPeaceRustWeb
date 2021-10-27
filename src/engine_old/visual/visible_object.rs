use crate::engine_old::visual::mesh::Mesh;
use std::sync::{Arc, Mutex};
use crate::engine_old::visual::material::Material;
use web_sys::WebGlRenderingContext as GL;

pub struct VisibleObject {
    pub mesh: Arc<Mutex<Mesh>>,
    pub material: Arc<Mutex<Material>>,
}

impl VisibleObject {
    pub fn new(mesh: Arc<Mutex<Mesh>>, material: Arc<Mutex<Material>>) -> Self {
        VisibleObject {
            mesh,
            material,
        }
    }

    pub fn draw(&self, gl: &GL) {
        let mat = self.material.lock().unwrap();
        let mesh = self.mesh.lock().unwrap();

        mat.bind(gl);
        mesh.draw(gl);
    }
}