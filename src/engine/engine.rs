use crate::engine;

pub struct Engine {
    context: engine::EngineContext,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: engine::EngineContext::new("Engine", 800, 600),
        }
    }

    pub fn run(&mut self) {
        let mut window_should_close = false;

        self.context.initialize();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }

        while !window_should_close {
            // Swap the buffers of the opengl window and updates event pipeline
            let frame_data = self.context.next_frame();

            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            if frame_data
                .input_system
                .keyboard
                .get_key_press_state(glfw::Key::Escape)
                == glfw::Action::Press
            {
                window_should_close = true;
            }
        }
        self.context.set_should_terminate(true);
    }
}
