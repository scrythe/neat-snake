use pyo3::prelude::*;
mod snake;

#[pymodule]
mod neat_snake {
    #[pymodule_export]
    use super::snake::Game;
    #[pymodule_export]
    use super::snake::GameState;
}
