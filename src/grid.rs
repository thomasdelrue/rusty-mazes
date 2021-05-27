use crate::cell::Cell;
use rand::Rng;
use std::fmt;
use std::fmt::Display;


pub struct Grid {
    pub rows: u8,
    pub columns: u8,
    _grid: Vec<Vec<Cell>>
}

impl Grid {
    
    pub fn new(rows: u8, columns: u8) -> Grid {
        let _grid = Grid::prepare_grid(rows, columns);
        let mut _grid = Grid {
            rows,
            columns,
            _grid,
        };
        _grid.configure_cells();
        _grid
    }

    pub fn cell_at(&self, row: u8, col: u8) -> &Cell {
        &self._grid[row as usize][col as usize]
    }

    pub fn random_cell(&self) -> &Cell {
        let mut rng = rand::thread_rng();
        let (row, col) = (rng.gen_range(0..self.rows), rng.gen_range(0..self.columns));
        self.cell_at(row, col)
    }

    pub fn size(&self) -> usize {
        (self.rows * self.columns) as usize
    }    

    fn prepare_grid(rows: u8, columns: u8) -> Vec<Vec<Cell>> {
        let mut _grid = Vec::with_capacity(rows as usize);

        for row in 0..rows {
            let mut _row = Vec::with_capacity(columns as usize);
            for col in 0..columns {
                _row.push(Cell::new(row, col));
            }
            _grid.push(_row)
        }

        _grid
    }

    // TODO: north, south, west, east...
    fn configure_cells(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                self._grid[row as usize][col as usize].north = Some((6, 6));
            }
        }
    }

    // TODO: write iterators, iter_rows, iter_cells...
    
}

impl Display for Grid {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(format, "{:?}", self._grid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_cell() {
        let grid = Grid::new(5, 5);

        println!("Random cell: {:?}", grid.random_cell());
    }
}