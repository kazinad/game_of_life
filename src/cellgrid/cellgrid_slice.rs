use super::*;

impl CellGrid {
    pub fn split_mut(&mut self, slices: usize) -> Vec<CellGridSlice> {
        let mut result = Vec::with_capacity(slices);
        let cells_len = self.cells.len();
        let mut chunk_size = cells_len / slices;
        if cells_len % slices > 0 {
            chunk_size += 1;
        };
        let mut offset = 0;
        let mut cells_bits = self.indexer.width * self.indexer.height;
        let chunk_bits = chunk_size * BITS_PER_CELLS;
        for group in self.cells.chunks_mut(chunk_size) {
            result.push(CellGridSlice {
                indexer: Indexer::new(self.indexer.width, self.indexer.height, offset),
                len: {
                    if cells_bits < chunk_bits {
                        cells_bits
                    } else {
                        chunk_bits
                    }
                },
                cells: group,
            });
            offset += chunk_size * BITS_PER_CELLS;
            if cells_bits > chunk_bits {
                cells_bits -= chunk_bits;
            }
        }
        result
    }
}

pub struct CellGridSlice<'a> {
    pub(super) indexer: Indexer,
    pub(super) len: usize,
    cells: &'a mut [CellType],
}

impl CellGridSlice<'_> {
    pub fn set(&mut self, x: usize, y: usize, bit: bool) -> Result<(), BoundsError> {
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
}
