use super::*;

use rand::prelude::*;

pub struct CellGrid {
    pub(super) indexer: Indexer,
    pub(super) cells: Vec<CellType>,
}

impl CellGrid {
    pub fn new(width: usize, height: usize, randomize: bool) -> Result<CellGrid, BoundsError> {
        if width == 0 || height == 0 {
            return Err(BoundsError {
                kind: ErrorKind::SizeError,
            });
        };
        let indexer = Indexer::new(width, height, 0);
        let cell_count = indexer.cell_count();
        let mut result = CellGrid {
            indexer: indexer,
            cells: Vec::with_capacity(cell_count),
        };
        for _ in 0..cell_count {
            result.cells.push(if randomize { random() } else { 0 });
        }
        Ok(result)
    }

    pub fn get(&self, x: usize, y: usize) -> Result<bool, BoundsError> {
        let index = self.indexer.index(x, y)?;
        let cell = self.cells[index.cell];
        let bit = cell & index.bit_mask;
        Ok(bit != 0)
    }

    fn set(&mut self, x: usize, y: usize, bit: bool) -> Result<(), BoundsError> {
        let index = self.indexer.index(x, y)?;
        let mut cell = self.cells[index.cell];
        if bit {
            cell |= index.bit_mask;
        } else {
            cell &= !index.bit_mask;
        }
        self.cells[index.cell] = cell;
        Ok(())
    }

    pub fn neighbours(&self, x0: usize, y0: usize) -> Result<u8, BoundsError> {
        let mut result = 0u8;
        for dy in -1..=1 {
            let y = self.indexer.add_y(y0, dy);
            for dx in -1..=1 {
                let x = self.indexer.add_x(x0, dx);
                if !(dy == 0 && dx == 0) && self.get(x, y)? {
                    result += 1;
                }
            }
        }
        Ok(result)
    }

    pub fn set_random(&mut self, bit: bool) -> Result<(), BoundsError> {
        let (x, y) = self.indexer.random();
        self.set(x, y, bit)?;
        Ok(())
    }
}
