use crate::engine;

pub struct Engine {
    context: engine::EngineContext,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: engine::EngineContext::new(),
        }
    }

    pub fn run(&self) {}
}
