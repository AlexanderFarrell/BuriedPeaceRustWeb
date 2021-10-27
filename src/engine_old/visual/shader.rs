use crate::{WebGlProgram, WebGlRenderingContext as GL, WebGlShader, WebGlUniformLocation};
use std::collections::HashMap;



pub struct Shader {
    program: WebGlProgram,
    uniform_locations: Vec<WebGlUniformLocation>,
    pub uniforms: HashMap<&'static str, usize>,
}

impl Shader {
    pub fn create(gl: &GL, sources: Vec<(u32, &str)>, uniform_names: &[&'static str]) -> Result<Shader, String> {
        let program = create_program(
            &gl,
            sources
        )?;
        let names_loc = get_name_loc(&uniform_names);
        let locs = get_named_locs(&gl, uniform_names, &program)?;
        Ok(Shader {
            program,
            uniform_locations: locs,
            uniforms: names_loc,
        })
    }

    // pub fn create_vert_frag(gl: &GL,
    //                         vert_source: &str,
    //                         frag_source: &str,
    //                         uniform_names: &[&str]) -> Result<WebGlProgram, &'static str> {
    //     let names_loc = get_name_loc(&uniform_names);
    //     let locs = get_named_locs(&gl, uniform_names, );
    //     Ok(Shader {
    //         program: create_program(
    //             &gl,
    //             vec![
    //                 (GL::VERTEX_SHADER, vert_source),
    //                 (GL::FRAGMENT_SHADER, frag_source)
    //             ],
    //         )?,
    //         uniform_locations: locs,
    //         uniforms: names_loc,
    //     })
    // }

    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn uniform_mat(&self, gl: &GL, data: &[f32; 16], loc: usize){
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniform_locations[loc]),
            false,
            data,
        )
    }

    pub fn uniform_1i(&self, gl: &GL, data: i32, loc: usize){
        gl.uniform1i(
            Some(&self.uniform_locations[loc]),
            data,
        )
    }

    pub fn uniform_1f(&self, gl: &GL, data: f32, loc: usize){
        gl.uniform1f(
            Some(&self.uniform_locations[loc]),
            data,
        )
    }

    pub fn uniform1fv_with_f32_array(&self, gl: &GL, data: &[f32; 1], loc: usize){
        gl.uniform1f(
            Some(&self.uniform_locations[loc]),
            data[0],
        )
    }

    pub fn uniform2fv_with_f32_array(&self, gl: &GL, data: &[f32; 2], loc: usize){
        gl.uniform2fv_with_f32_array(
            Some(&self.uniform_locations[loc]),
            data,
        )
    }

    pub fn uniform3fv_with_f32_array(&self, gl: &GL, data: &[f32; 3], loc: usize){
        gl.uniform3fv_with_f32_array(
            Some(&self.uniform_locations[loc]),
            data,
        )
    }

    pub fn uniform4fv_with_f32_array(&self, gl: &GL, data: &[f32; 4], loc: usize){
        gl.uniform4fv_with_f32_array(
            Some(&self.uniform_locations[loc]),
            data,
        )
    }
}

fn get_named_locs(gl: &GL, uniform_names: &[&str], program: &WebGlProgram) -> Result<Vec<WebGlUniformLocation>, String> {
    // Ok(uniform_names
    //     .iter()
    //     .map(|name| {
    //         get_uniform_loc(&gl, name, &program)
    //     })
    //     .collect::<Result<WebGlUniformLocation, String>>()?)
    let mut s = vec![];
    for uniform_name in uniform_names {
        s.push(get_uniform_loc(&gl, uniform_name, &program)?);
    }
    Ok(s)
}

fn create_program(
    gl: &GL,
    dna: Vec<(u32, &str)>,
) -> Result<WebGlProgram, String>{
    let program = gl
        .create_program()
        .unwrap();
        // .ok_or_else(|| String::from("Error creating program from WASM"));

    let mut shaders = Vec::new();
    for s in dna {
        shaders.push(create_shader(
            &gl,
            s.0,
            s.1,
        )?)
    }

    // let shaders: Vec<WebGlShader> =
    //     dna.iter().map(|i| create_shader(
    //         &gl,
    //         i.0,
    //         i.1,
    //     )?).collect::<Result<WebGlShader, String>>()?;

    for shader in shaders.iter() {
        gl.attach_shader(&program, &shader);
    }

    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Error linking shader program AND error getting program info log for the error message.")))
    }
}

fn get_uniform_loc(gl: &GL, name: &str, program: &WebGlProgram) -> Result<WebGlUniformLocation, String> {
    gl.get_uniform_location(&program, name)
        .ok_or_else(|| String::from(format!("Error getting uniform location: {}", name)))
}

fn get_name_loc(dnas: &[&'static str]) -> HashMap<&'static str, usize> {
    let mut h: HashMap<&'static str, usize> = HashMap::new();
    for (index, dna) in dnas.iter().enumerate() {
        h.insert(dna, index);
    }
    h
}

// fn create_vert_frag_program(
//     gl: &GL,
//     vert_source: &str,
//     frag_source: &str,
// ) -> Result<WebGlProgram, &'static str>{
//     let program = gl
//         .create_program()
//         .ok_or_else(|| "Error creating program from WASM");
//
//     let vert = create_shader(
//         &gl,
//         GL::VERTEX_SHADER,
//         vert_source,
//     ).unwrap();
//
//     let frag = create_shader(
//         &gl,
//         GL::FRAGMENT_SHADER,
//         frag_source,
//     ).unwrap();
//
//     gl.attach_shader(&program, &vert_shader);
//     gl.attach_shader(&program, &frag_shader);
//     gl.link_program(&program);
//
//     if gl.get_program_parameter(&program, GL::LINK_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(program)
//     } else {
//         Err(gl.get_program_info_log(&program)
//             .unwrap_or_else(|| "Error linking shader program AND error getting program info log for the error message."))
//     }
// }

fn create_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader from WASM"))
        .unwrap();

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Error compiling shader AND Error getting the shader log.")))
    }
}