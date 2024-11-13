// src/main.rs
use pyo3::prelude::*;
use ndarray::Array2;

fn main() {
    println!("Welcome to the Movie Recommendation System");
}

#[pyfunction]
fn process_user_input(user_input: String) -> PyResult<String> {
    // Placeholder function to process user input
    Ok(format!("Processing input: {}", user_input))
}
