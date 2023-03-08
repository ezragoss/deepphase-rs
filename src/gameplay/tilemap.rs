/** Handles logic imminent to the tile map data structure
*
*
*/

pub type Coord = (i8, i8);

pub enum Direction {
    North,
    South,
    East,
    West
}

pub trait CoordinateSystem {
    fn neighbors(&self) -> Vec<Coord>;
    fn neighbors_of(coord: &Coord) -> Vec<Coord>;
    fn neighbor(&self, direction: Direction) -> Coord;
    fn neighbor_of(coord: &Coord, direction: Direction) -> Coord;
    fn coord(&self) -> Coord;
}

enum TileState {
    TemporaryResistance(i8),  // Tile belongs to resistance until number of turns are over
    Resistance,
    Suppressor
}

pub struct Tile {
    state: TileState,
    coord: Coord
}

impl CoordinateSystem for Tile {
    /// Get all the neighbors in a clockwise ordering
    fn neighbors(&self) -> Vec<Coord> {
        Self::neighbors_of(&self.coord)
    }

    /// Get all the neighbors in a clockwise ordering
    fn neighbors_of(coord: &Coord) -> Vec<Coord> {
        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];
        let mut neighbors: Vec<Coord>= Vec::new();
        for direction in directions {
            neighbors.push(Self::neighbor_of(coord, direction));
        }
        return neighbors
    }

    /// Get the neighbor for the given direction
    ///
    /// ## Arguments:
    /// * `direction` - the direction of the neighbor from this instances coordinate
    fn neighbor(&self, direction: Direction) -> Coord {
        Self::neighbor_of(&self.coord, direction)
    }

    fn neighbor_of(coord: &Coord, direction: Direction) -> Coord {
        let (x, y) = coord.clone();
        match direction {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y)
        }
    }

    fn coord(&self) -> Coord {
        return self.coord;
    }
}

// -- TESTS -- //

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use crate::gameplay::tilemap::{TileState, Tile, Direction, CoordinateSystem, Coord};

    /// Tests whether the neighbor for the given direction returns expected result
    #[test]
    fn validate_neighbor() {
        let coord: Coord = (0, 0);
        let north_neighbor = Tile::neighbor_of(&coord, Direction::North);
        let east_neighbor = Tile::neighbor_of(&coord, Direction::East);
        let south_neighbor = Tile::neighbor_of(&coord, Direction::South);
        let west_neighbor = Tile::neighbor_of(&coord, Direction::West);
        assert_eq!(north_neighbor, (0, 1));
        assert_eq!(south_neighbor, (0, -1));
        assert_eq!(east_neighbor, (1, 0));
        assert_eq!(west_neighbor, (-1, 0));
    }

    /// Tests that neighbors return in a clockwise ordering
    #[test]
    fn validate_neighbors_order() {
        let coord: Coord = (0, 0);
        let neighbors = Tile::neighbors_of(&coord);
        let direction_order = [
            Direction::North, Direction::East, Direction::South, Direction::West
        ];
        for (neighbor, direction) in zip(neighbors, direction_order)  {
            assert_eq!(neighbor, Tile::neighbor_of(&coord, direction))
        }
    }
}