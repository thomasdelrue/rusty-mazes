use std::collections::{HashMap, hash_map};
use std::fmt;
use std::fmt::Debug;


pub struct Cell {
    row: u8,
    column: u8,
    links: HashMap<(u8, u8), bool>,
    pub north: Option<(u8, u8)>
}

impl Cell {
    pub fn new(row: u8, column: u8) -> Cell {
        Cell {
            row,
            column,
            links: HashMap::new(),
            north: None,
        }
    }

    pub fn pos(&self) -> (u8, u8) {
        (self.row, self.column)
    }

    pub fn link(&mut self, other: &mut Cell, bidi: bool) {
        self.links.insert(other.pos(), true);
        if bidi {
            other.links.insert(self.pos(), true);
        }
    }

    pub fn unlink(&mut self, other: &mut Cell, bidi: bool) {
        self.links.remove(&other.pos());
        if bidi {
            other.links.remove(&self.pos());
        }
    }

    pub fn links(&self) -> hash_map::Keys<(u8, u8), bool> {
        self.links.keys()
    }

    pub fn is_linked_to(&self, other: &Cell) -> bool {
        if let Some(&linked) = self.links.get(&other.pos()) {
            linked
        } else {
            false
        }
    }

    // TODO: north, east, west, south, neighbours. here or in Grid?
}

impl Debug for Cell {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(format, "{},{} -> {:?}", self.row, self.column, self.north)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos() {
        let cell = Cell::new(1, 2);

        assert_eq!(cell.pos(), (1, 2));
    }

    #[test]
    fn test_bidi_link() {
        let mut cell = Cell::new(1, 2);
        let mut other = Cell::new(3, 4);
        cell.link(&mut other, true);

        assert_eq!(*cell.links.get(&(3, 4)).unwrap(), true);
        assert_eq!(*other.links.get(&(1, 2)).unwrap(), true);
    }

    #[test]
    fn test_is_linked_to() {
        let mut cell = Cell::new(0, 0);
        let mut other = Cell::new(5, 5);
        let third = Cell::new(3, 3);
        cell.link(&mut other, false);

        assert!(cell.is_linked_to(&other));
        assert!(! cell.is_linked_to(&third));
    }

    #[test]
    fn test_links() {
        let mut cell = Cell::new(1, 2);
        let mut other = Cell::new(3, 4);
        let mut third = Cell::new(5, 6);
        cell.link(&mut other, false);
        cell.link(&mut third, false);

        assert_eq!(cell.links.len(), 2);
        assert_eq!(*cell.links.get(&(3, 4)).unwrap(), true);
        assert_eq!(*cell.links.get(&(5, 6)).unwrap(), true);
    }
}