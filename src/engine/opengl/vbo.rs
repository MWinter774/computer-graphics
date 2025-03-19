pub struct VBO {
    vbo_index: gl::types::GLuint,
}

impl VBO {
    pub fn new() -> Self {
        let mut vbo_index: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_index);
        }
        Self { vbo_index }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_index);
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo_index);
        }
    }
}
