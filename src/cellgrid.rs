use rand::prelude::*;

// -- indexer

#[derive(Debug)]
pub enum ErrorKind {
    SizeError,
    PositionError,
}

#[derive(Debug)]
pub struct BoundsError {
    pub kind: ErrorKind,
}

struct BitIndex {
    cell: usize,
    bit_mask: CellType,
}

struct Indexer {
    width: usize,
    height: usize,
    offset: usize,
}

impl Indexer {
    fn new(width: usize, height: usize, offset: usize) -> Indexer {
        Indexer {
            width: width,
            height: height,
            offset: offset,
        }
    }
    fn index(&self, x: usize, y: usize) -> Result<BitIndex, BoundsError> {
        if x >= self.width || y >= self.height {
            return Err(BoundsError {
                kind: ErrorKind::PositionError,
            });
        }
        let bit_index = y * self.width + x - self.offset;
        let shift = bit_index % BITS_PER_CELLS;
        let bit_mask = 1 << shift;
        Ok(BitIndex {
            cell: bit_index / BITS_PER_CELLS,
            bit_mask: bit_mask,
        })
    }

    fn pos(&self, index: usize) -> (usize, usize) {
        let c1 = index + self.offset;
        (c1 % self.width, c1 / self.width)
    }

    fn add(n: usize, dn: isize, max: usize) -> usize {
        let udn = (dn.abs() as usize) % max;
        if dn < 0 {
            if udn > n {
                return max - udn;
            }
            return n - udn;
        };
        return (n + udn) % max;
    }

    fn add_x(&self, x: usize, dx: isize) -> usize {
        Indexer::add(x, dx, self.width)
    }

    fn add_y(&self, y: usize, dy: isize) -> usize {
        Indexer::add(y, dy, self.height)
    }

    fn random(&self) -> (usize, usize) {
        (
            random::<usize>() % self.width,
            random::<usize>() % self.height,
        )
    }

    fn cell_count(&self) -> usize {
        let bits_count = self.width * self.height;
        let mut cell_count = bits_count / BITS_PER_CELLS;
        if bits_count % BITS_PER_CELLS > 0 {
            cell_count += 1
        }
        cell_count
    }
}

// -- CellGrid

type CellType = usize;

static BITS_PER_CELLS: usize = std::mem::size_of::<CellType>() * 8;

pub struct CellGrid {
    indexer: Indexer,
    cells: Vec<CellType>,
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

// -- iterator

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

// -- slice

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
    indexer: Indexer,
    len: usize,
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

// slice iterator

impl CellGridSlice<'_> {
    pub fn iter(&self) -> CellGridSliceIterator {
        CellGridSliceIterator {
            indexer: Indexer::new(self.indexer.width, self.indexer.height, self.indexer.offset),
            current: 0,
            len: self.len,
        }
    }
}

pub struct CellGridSliceIterator {
    indexer: Indexer,
    current: usize,
    len: usize,
}

impl Iterator for CellGridSliceIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.current;
        self.current += 1;
        if c < self.len {
            return Some(self.indexer.pos(c));
        }
        None
    }
}
