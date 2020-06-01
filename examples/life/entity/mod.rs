pub mod cell;
pub mod grid;

pub use cell::*;
pub use grid::*;

/// The type used for the entities IDs.
pub type Id = i32;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub enum Kind {
    Grid,
    Cell,
}
