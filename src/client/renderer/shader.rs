use std::{fs, io};

use glium::{Display, Program};



pub fn read_shader(d: &Display, v_filename: &str, f_filename: &str) -> io::Result<Program> {

    let v_src = fs::read_to_string(v_filename)?;
    let f_src = fs::read_to_string(f_filename)?;

    Ok(
        Program::from_source(d, &v_src, &f_src, None).unwrap()
    )
    
}