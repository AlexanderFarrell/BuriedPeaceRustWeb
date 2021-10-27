use crate::{WebGlBuffer};

use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

type AttribBuffer = (u32, u32);

pub struct AttribInfo {
    pub len_elements: i32,
    pub ele_type: u32,
    pub normalize: bool,
    pub stride: i32,
    pub offset: i32,
}

impl Default for AttribInfo {
    fn default() -> Self {
        Self {
            len_elements: 4,
            ele_type: GL::FLOAT,
            normalize: false,
            stride: 0,
            offset: 0
        }
    }
}

pub enum Indices {
    None,
    Small(Vec<u16>),
    Large(Vec<u32>),
}

pub struct Mesh {
    vertex_attr_info: Vec<AttribInfo>,

    pub vertices: Vec<f32>,
    pub indices: Indices,

    vertex_buffer: Option<WebGlBuffer>,
    index_buffer: Option<WebGlBuffer>,

    ele_count: i32,
}

impl Mesh {
    pub fn new(
        attrib_info: Vec<AttribInfo>,
        indices: Indices) -> Self {
        Self {
            vertex_attr_info: attrib_info,

            vertex_buffer: None,
            index_buffer: None,

            vertices: vec![],
            indices,

            ele_count: 0
        }
    }

    pub fn vertex_attr_info(&self) -> &Vec<AttribInfo> {
        &(self.vertex_attr_info)
    }

    pub fn buffer(&mut self, gl: &GL, access: u32){
        self.ele_count = (self.vertices.len() / 3) as i32;
        self.vertex_buffer = Some(create_vertex_buffer(
            gl,
            self.vertices.as_slice(),
            GL::STATIC_DRAW));

        match &self.indices {
            Indices::None => {
                self.ele_count = (self.vertices.len() as i32 / self.vertex_attr_info.iter().map(|info| info.len_elements).sum::<i32>() as i32) as i32;
                self.index_buffer = None
            }
            Indices::Small(indices) => {
                self.ele_count = indices.len() as i32;
                self.index_buffer =
                    Some(create_index_buffer_small(&gl, indices.as_slice(), access));

            }
            Indices::Large(indices) => {
                self.ele_count = indices.len() as i32;
                self.index_buffer =
                    Some(create_index_buffer_large(&gl, indices.as_slice(), access));
            }
        }
    }

    pub fn draw(&self, gl: &GL){
        //gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.bind_buffer(GL::ARRAY_BUFFER, self.vertex_buffer.as_ref());

        for (index, attribute) in self.vertex_attr_info.iter().enumerate() {
            gl.vertex_attrib_pointer_with_i32(
                index as u32,
                attribute.len_elements,
                attribute.ele_type,
                attribute.normalize,
                attribute.stride,
                attribute.offset,
            );
            gl.enable_vertex_attrib_array(index as u32);
        }

        match &self.indices {
            Indices::None => {
                gl.draw_arrays(GL::TRIANGLES, 0, self.ele_count);
                //gl.draw_arrays(GL::TRIANGLES, 0, (self.vertices / 2) as i32);
                todo!()
            }
            Indices::Small(_) => {
                gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());
                gl.draw_elements_with_i32(GL::TRIANGLES,
                                          self.ele_count,
                                          GL::UNSIGNED_SHORT,
                                          0);
            }
            Indices::Large(_) => {
                gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());
                gl.draw_elements_with_i32(GL::TRIANGLES, self.ele_count, GL::UNSIGNED_INT, 0);
            }
        }
    }
}

fn create_vertex_buffer(gl: &GL, elements: &[f32], access: u32) -> WebGlBuffer {
    let memory = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let vert_loc = elements.as_ptr() as u32 / std::mem::size_of::<f32>() as u32;
    let vert_array = js_sys::Float32Array::new
        (&memory).subarray(
        vert_loc,
        vert_loc + elements.len() as u32,
    );
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, access);
    buffer
}

fn create_index_buffer_small(gl: &GL, elements: &[u16], access: u32) -> WebGlBuffer {
    let memory = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let loc = elements.as_ptr() as u32 / std::mem::size_of::<u16>() as u32;
    let array = js_sys::Uint16Array::new
        (&memory).subarray(
        loc,
        loc + elements.len() as u32,
    );
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, access);
    buffer
}

fn create_index_buffer_large(gl: &GL, elements: &[u32], access: u32) -> WebGlBuffer {
    let memory = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let loc = elements.as_ptr() as u32 / std::mem::size_of::<u32>() as u32;
    let array = js_sys::Uint32Array::new
        (&memory).subarray(
        loc,
        loc + elements.len() as u32,
    );
    let buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &array, access);
    buffer
}