pub mod ant;
pub mod cell;
pub mod grid;

pub use ant::*;
pub use cell::*;
pub use grid::*;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
    Grid,
    Cell,
    Ant,
}
