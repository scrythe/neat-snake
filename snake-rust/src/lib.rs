use pyo3::prelude::*;
mod snake;

#[pymodule]
mod snake_rust {
    #[pymodule_export]
    use super::snake::Game;
    #[pymodule_export]
    use super::snake::GameState;
}
