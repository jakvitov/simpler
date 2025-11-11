use std::env;
use std::fmt::{Display, Formatter};

pub mod basic_simplex_table_data;
pub mod simplex_error;
mod basic_simplex_solver;
mod basic_simplex_table_operations;
mod two_phase_simplex_solver;
mod revised_simplex_solver;

pub use basic_simplex_solver::solve_basic_simplex;
pub use two_phase_simplex_solver::solve_two_phase_simplex;

pub enum SimplexSoverAlgorithm {
    BASIC_SIMPLEX,
    TWO_PHASE_SIMPLEX,
    REVISED_SIMPLEX
}

impl Display for SimplexSoverAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimplexSoverAlgorithm::BASIC_SIMPLEX => f.write_str("Basic simplex algorithm"),
            SimplexSoverAlgorithm::TWO_PHASE_SIMPLEX => f.write_str("Two phase simplex algorithm"),
            SimplexSoverAlgorithm::REVISED_SIMPLEX => f.write_str("Revised simplex algorithm")
        }
    }
}
