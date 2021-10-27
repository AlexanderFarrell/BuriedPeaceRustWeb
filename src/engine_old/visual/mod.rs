mod shader;
pub mod material;
pub mod mesh;
pub mod visible_object;

use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use crate::engine::web_page_helper::element_by_id;
use wasm_bindgen::JsCast;
use crate::{JsValue, log};
use crate::engine::data::id_map::IdMap;
use std::sync::{Arc, Mutex};
use crate::engine::visual::visible_object::VisibleObject;

pub struct Visual {
    pub gl: WebGlRenderingContext,
    pub clear_color: (f32, f32, f32, f32),
    pub clear_depth: f32,
    pub visible_objects: IdMap<Arc<Mutex<VisibleObject>>>,
}
//Loc on Graph

impl Visual {
    pub fn new() -> Result<Self, JsValue> {
        let canvas_element = window().unwrap().document().unwrap().get_element_by_id("game_canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas_element.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        Ok(Visual {
            gl,
            clear_color: (0.5, 0.3, 0.7, 1.0),
            clear_depth: 1.0,
            visible_objects: IdMap::new(),
        })
    }

    pub fn add(&mut self, visible_object: &Arc<Mutex<VisibleObject>>) -> i32 {
        self.visible_objects.add(visible_object.clone())
    }

    pub fn remove(&mut self, id: i32) {
        self.visible_objects.remove(&id)
    }

    pub fn draw(&self) {
        self.gl.clear_color(self.clear_color.0,
                       self.clear_color.1,
                       self.clear_color.2,
                       self.clear_color.3);
        self.gl.clear_depth(1.0);
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        for (_, visible_object) in self.visible_objects.iter() {
            visible_object.lock().unwrap().draw(&self.gl);
        }
    }
}