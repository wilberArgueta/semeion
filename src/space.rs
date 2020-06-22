use std::ops::{Add, Sub};

/// A Point in 2D space.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// Represents the dimension of a grid as the integer number of columns and rows.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dimension {
    pub x: i32,
    pub y: i32,
}

/// The size of a Shape represented as number of pixels.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Represents the location of an entity within the environment as pair of
/// coordinate that identify the environment grid tile.
pub type Location = Point<i32>;

/// Represents an offset from an Entity location within the environment.
pub type Offset = Point<i32>;

/// Represents the location of an entity within the environment expressed in
/// pixel coordinates.
pub type PixelCoordinate = Point<f32>;

/// The scope of an Entity, defined as the maximum distance between the tile
/// where the Entity is located, and the farthest possible tile the Entity can
/// see or influence.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Scope(usize);

impl PixelCoordinate {
    /// Gets the origin coordinates in (0.0, 0.0).
    pub const fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Location {
    /// Gets the origin coordinates in (0, 0).
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Converts the Point into a point expressed as pixel coordinates, according
    /// to the length of each grid square side.
    pub fn to_pixel_coords(self, side: f32) -> PixelCoordinate {
        PixelCoordinate {
            x: self.x as f32 * side,
            y: self.y as f32 * side,
        }
    }

    /// Maps a 2-dimensional coordinate to a 1-dimensional index.
    pub fn one_dimensional(self, dimension: Dimension) -> usize {
        debug_assert!(!self.x.is_negative());
        debug_assert!(!self.y.is_negative());
        let pos = self.y.saturating_mul(dimension.x).saturating_add(self.x);
        debug_assert!(!pos.is_negative());
        debug_assert!(pos < dimension.x.saturating_mul(dimension.y));
        pos as usize
    }

    /// Translates the current location by the given offset, while keeping the
    /// final location within a Torus with the given dimension. Returns a reference
    /// to the final location.
    pub fn translate(
        &mut self,
        offset: Offset,
        dimension: Dimension,
    ) -> &mut Self {
        self.x = self.x.saturating_add(offset.x).rem_euclid(dimension.x);
        self.y = self.y.saturating_add(offset.y).rem_euclid(dimension.y);
        self
    }
}

impl From<(i32, i32)> for Location {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Offset {
    /// Gets a list of offsets from a central location in a grid, to all the tiles
    /// located in its border, according to the given distance between the tile
    /// in the center and the border (Scope), in arbitrary order. Returns a
    /// single Offset equal to the origin (0, 0) if the given Scope is equal to
    /// 0.
    pub fn border(scope: Scope) -> Vec<Offset> {
        let delta = scope.magnitude() as i32;
        if delta == 0 {
            return vec![Offset::origin()];
        }

        let mut offsets =
            Vec::with_capacity(Dimension::perimeter_with_scope(scope));
        // top and bottom rows of the border
        for &y in &[-delta, delta] {
            for x in -delta..=delta {
                offsets.push(Offset { x, y });
            }
        }
        // left and right columns of the border (without corners)
        for y in 1i32.saturating_sub(delta)..=delta.saturating_sub(1) {
            for &x in &[-delta, delta] {
                offsets.push(Offset { x, y });
            }
        }
        offsets
    }

    /// Gets a list of offsets from a central location in  a grid, to all the 4
    /// tiles located in the corners of its border, according to the given
    /// distance between the tile in the center and the border (Scope), in
    /// arbitrary order.
    pub fn corners(scope: Scope) -> [Offset; 4] {
        let delta = scope.magnitude() as i32;
        [
            (-delta, -delta).into(),
            (-delta, delta).into(),
            (delta, -delta).into(),
            (delta, delta).into(),
        ]
    }
}

impl Size {
    /// Converts the Size to a Dimension according to the given side length.
    pub fn to_dimension(self, side: f32) -> Dimension {
        Dimension {
            x: (self.width / side) as i32,
            y: (self.height / side) as i32,
        }
    }
}

impl Dimension {
    /// Gets the number of tiles in a grid of given Dimension, equal to the
    /// number of row by the number of columns.
    pub fn len(self) -> usize {
        debug_assert!(!self.x.is_negative());
        debug_assert!(!self.y.is_negative());
        self.x.saturating_mul(self.y) as usize
    }

    /// Returns true only if the number of tiles in the grid is 0.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Returns true only if the components of this Dimension are equal in magnitude,
    /// that is, `self.x == self.y`, and therefore this Dimension represents a square.
    pub fn is_square(self) -> bool {
        self.x == self.y
    }

    /// Gets the Location of the center of this Dimension.
    pub fn center(self) -> Location {
        Location {
            x: self.x / 2,
            y: self.y / 2,
        }
    }

    /// Returns true only if the given Location is within the Dimension.
    pub fn contains(self, location: Location) -> bool {
        debug_assert!(self.x >= 0 && self.y >= 0);
        !(location.x < 0
            || location.x >= self.x
            || location.y < 0
            || location.y >= self.y)
    }

    /// Gets the length of the side of a squared grid (where the number of rows
    /// is equal to the number of columns), given a specific scope (maximum
    /// distance from the center tile of the grid to the farthest).
    pub(crate) fn side_with_scope(scope: Scope) -> usize {
        1 + scope.magnitude().saturating_sub(1) * 2
    }

    /// Gets the perimeter of a squared grid (where the number of rows is equal
    /// to the number of columns), given a specific scope (maximum distance from
    /// the center tile of the grid to the farthest).
    pub(crate) fn perimeter_with_scope(scope: Scope) -> usize {
        match scope.magnitude() {
            0 => 1,
            _ => Self::side_with_scope(scope) * 4 + 4,
        }
    }

    /// Gets the number of elements in a squared grid (where the number of rows
    /// is equal to the number of columns), given a specific scope (maximum
    /// distance from the center tile of the grid to the farthest).
    pub(crate) fn len_with_scope(scope: Scope) -> usize {
        match scope.magnitude() {
            0 => 1,
            _ => {
                Self::len_with_scope((scope.magnitude() - 1).into())
                    + Self::perimeter_with_scope(scope)
            }
        }
    }
}

impl From<usize> for Scope {
    fn from(magnitude: usize) -> Self {
        Self(magnitude)
    }
}

impl From<Scope> for usize {
    fn from(scope: Scope) -> Self {
        scope.0
    }
}

impl Scope {
    /// Constructs a new Scope of the given magnitude.
    pub fn with_magnitude(magnitude: usize) -> Self {
        Self(magnitude)
    }

    /// Constructs a new Scope with no magnitude.
    pub fn empty() -> Self {
        Self::with_magnitude(0)
    }

    /// Gets the magnitude of this Scope, that is its value.
    pub fn magnitude(self) -> usize {
        self.0
    }
}

impl Add for Point<i32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point<i32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
