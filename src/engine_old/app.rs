use crate::Closure;
use crate::engine_old::engine::{Engine, GameApplication};
use crate::engine_old::engine;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use js_sys::Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use crate::engine_old::web_page_helper::request_animation_frame;

pub struct App<G: 'static + GameApplication> {
    game: Arc<Mutex<G>>,
    engine: Arc<Mutex<Engine>>,
}

impl<G: 'static + GameApplication> App<G> {
    pub fn new(game: G, engine: Engine) -> Self {
        Self {
            game: Arc::new(Mutex::new(game)),
            engine: Arc::new(Mutex::new(engine)),
        }
    }

    fn run(&self, engine: Arc<Mutex<Engine>>, game: Arc<Mutex<G>>) {
        {
            //engine_old.clone().lock().unwrap().start();
            game.clone().lock().unwrap().start(engine.clone());
        }


        let inner_data_ref = Rc::new(RefCell::new(None));
        let outer_data_ref = inner_data_ref.clone();
        let g = game.clone();
        let e = engine.clone();

        let mut running = true;

        *outer_data_ref.borrow_mut() = Some(Closure::wrap(Box::new(move || {

            let running: bool = {
                engine.clone().lock().unwrap().running.clone()
            };

            if running {
                let _ = inner_data_ref.borrow_mut().take();
                return;
            }

            g.lock().unwrap().update(e.clone());
            e.clone().lock().unwrap().update();
            e.clone().lock().unwrap().draw();


            request_animation_frame(inner_data_ref.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(outer_data_ref.borrow().as_ref().unwrap());
    }

    pub fn start(&self){
        let e = self.engine.clone();
        let g = self.game.clone();
        self.run(e, g);
    }

    pub fn stop(&self){
        self.engine.lock().unwrap().running = false;
    }
}
