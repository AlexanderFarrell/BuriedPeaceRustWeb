use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::Window;
use crate::engine::visual::Visual;

pub struct Engine {
    pub(crate) running: bool,
    pub visual: Visual,
}

impl Engine {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self {
            running: false,
            visual: Visual::new()?,
        })
    }

    pub fn update(&self){

    }

    pub fn draw(&self){
        self.visual.draw();
    }
}

pub trait GameApplication {
    fn start(&self, engine: Arc<Mutex<Engine>>);
    fn end(&self, engine: Arc<Mutex<Engine>>);
    fn update(&self, engine: Arc<Mutex<Engine>>);
}