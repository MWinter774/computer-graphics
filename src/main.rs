#![deny(clippy::all)]

use computer_graphics::engine;

fn main() {
    let mut e = engine::Engine::new();
    e.run();
}
