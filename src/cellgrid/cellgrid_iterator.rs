use super::*;

impl CellGrid {
    pub fn iter(&self) -> CellGridIterator {
        CellGridIterator {
            x: 0,
            y: 0,
            cell_grid: self,
        }
    }
}

pub struct CellGridIterator<'a> {
    x: usize,
    y: usize,
    cell_grid: &'a CellGrid,
}

impl CellGridIterator<'_> {
    fn step(&mut self) {
        self.x += 1;
        if self.x >= self.cell_grid.indexer.width {
            self.y += 1;
            self.x = 0;
        }
    }
}

impl Iterator for CellGridIterator<'_> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.cell_grid.indexer.width && self.y < self.cell_grid.indexer.height {
            let (x, y) = (self.x, self.y);
            self.step();
            match self.cell_grid.get(x, y) {
                Ok(alive) => return Some((x, y, alive)),
                Err(_) => return None,
            }
        }
        None
    }
}
