use std::string::FromUtf8Error;

use glium::{Display, Program};

pub fn compile_shaders(d: &Display, v_bytes: &[u8], f_bytes: &[u8]) -> Result<Program, FromUtf8Error> {
    let v_src = String::from_utf8(v_bytes.to_vec())?;
    let f_src = String::from_utf8(f_bytes.to_vec())?;

    Ok(Program::from_source(d, &v_src, &f_src, None).unwrap())
}
