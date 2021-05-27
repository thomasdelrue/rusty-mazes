mod cell;
mod grid;

use grid::Grid;

fn main() {
    let _grid = Grid::new(10, 10);

    println!("Grid: {}", _grid);

    let _cell = _grid.cell_at(1, 2);

    println!("Cell: {:?}", _cell);
}
