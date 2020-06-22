use super::*;

/// The neighbor tiles of a specific entity.
#[derive(Debug)]
pub struct NeighborHood<'a, 'e, I: Eq + Hash + Debug, K, C, T, E> {
    tiles: Vec<TileView<'a, 'e, I, K, C, T, E>>,
    dimension: Dimension,
}

impl<'a, 'e, I: Eq + Hash + Debug, K, C, T, E>
    NeighborHood<'a, 'e, I, K, C, T, E>
{
    /// Gets the dimension of this neighborhood.
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Gets the list of tiles included in this NeighborHood.
    pub fn tiles(&self) -> &Vec<TileView<'a, 'e, I, K, C, T, E>> {
        &self.tiles
    }

    /// Gets the tile located at the given offset from the center of this
    /// NeighborHood. The NeighborHood is seen as a Torus from this method,
    /// therefore, out of dimension offsets will be translated considering that
    /// the NeighborHood edges are joined.
    pub fn tile(&self, offset: Offset) -> &TileView<'a, 'e, I, K, C, T, E> {
        debug_assert!(!self.tiles.is_empty());
        let mut center = self.dimension.center();
        let index = center
            .translate(offset, self.dimension)
            .one_dimensional(self.dimension);
        &self.tiles[index]
    }

    /// Gets the tile located in the center of this NeighborHood.
    pub fn center(&self) -> &TileView<'a, 'e, I, K, C, T, E> {
        self.tile(Offset::origin())
    }

    /// Gets a list of tiles that surround the tile T of this NeighborHood,
    /// located at a given Offset from the center tile, and according to the
    /// given Scope, that represents the distance from the tile T.
    /// The tiles are returned in arbitrary order. Returns None if any of the
    /// border tiles is out of the NeighborHood dimension for the given Scope.
    pub fn border(
        &self,
        offset: Offset,
        scope: Scope,
    ) -> Option<Vec<&TileView<'a, 'e, I, K, C, T, E>>> {
        // the location of the tile T relative to the center
        let loc = self.dimension.center() + offset;

        // iterate over the 4 corners surrounding the tile T to check if
        // the whole border of the tile T is contained within this NeighborHood
        // according to the given scope
        for &delta in &Offset::corners(scope) {
            if !self.dimension.contains(loc + delta) {
                return None;
            }
        }

        let mut tiles =
            Vec::with_capacity(Dimension::perimeter_with_scope(scope));
        for mut delta in Offset::border(scope) {
            let loc = *delta.translate(offset, self.dimension);
            tiles.push(self.tile(loc))
        }

        debug_assert_eq!(tiles.capacity(), tiles.len());
        Some(tiles)
    }
}

impl<'a, 'e, I: Eq + Hash + Debug, K, C, T, E>
    From<Vec<TileView<'a, 'e, I, K, C, T, E>>>
    for NeighborHood<'a, 'e, I, K, C, T, E>
{
    /// Constructs a new NeighborHood from a list of tiles that can encode a
    /// squared grid.
    fn from(tiles: Vec<TileView<'a, 'e, I, K, C, T, E>>) -> Self {
        debug_assert!(!tiles.is_empty());
        let length = tiles.len() as f64;
        // NeighborHoods can only be constructed if they represent squares
        debug_assert!(math::is_perfect_square(length));
        let side = length.sqrt() as i32;
        Self {
            tiles,
            dimension: Dimension { x: side, y: side },
        }
    }
}
