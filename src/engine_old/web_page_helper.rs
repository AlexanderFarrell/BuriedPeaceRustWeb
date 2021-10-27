use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use crate::Closure;

#[inline]
pub fn window() -> web_sys::Window {
    web_sys::window().expect("Could not get window")
}

#[inline]
pub fn document() -> web_sys::Document {
    window().document().expect("Could not get document")
}

#[inline]
pub fn element_by_id(id: &'static str) -> Option<Element> {
    document().get_element_by_id(id)
}

#[inline]
pub fn request_animation_frame(f: &Closure<dyn FnMut()>){
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("Could not request animation frame. ");
}

