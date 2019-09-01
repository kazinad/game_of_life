pub struct CellGrid {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

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
    bit_mask: usize,
}

static BITS_PER_CELLS: usize = std::mem::size_of::<usize>() * 8;

use rand::prelude::*;

impl CellGrid {
    pub fn new(width: usize, height: usize, randomize: bool) -> Result<CellGrid, BoundsError> {
        if width == 0 || height == 0 {
            return Err(BoundsError {
                kind: ErrorKind::SizeError,
            });
        };
        let cells = cells(width, height);
        let mut result = CellGrid {
            width: width,
            height: height,
            cells: Vec::with_capacity(cells),
        };
        for _ in 0..cells {
            result.cells.push(if randomize { random() } else { 0 });
        }
        Ok(result)
    }

    fn index(&self, x: usize, y: usize) -> Result<BitIndex, BoundsError> {
        if x >= self.width || y >= self.height {
            return Err(BoundsError {
                kind: ErrorKind::PositionError,
            });
        }
        let bit_index = y * self.width + x;
        let shift = bit_index % BITS_PER_CELLS;
        let bit_mask = 1 << shift;
        Ok(BitIndex {
            cell: bit_index / BITS_PER_CELLS,
            bit_mask: bit_mask,
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Result<bool, BoundsError> {
        let index = self.index(x, y)?;
        let cell = self.cells[index.cell];
        let bit = cell & index.bit_mask;
        Ok(bit != 0)
    }

    pub fn set(&mut self, x: usize, y: usize, bit: bool) -> Result<(), BoundsError> {
        let index = self.index(x, y)?;
        let mut cell = self.cells[index.cell];
        if bit {
            cell |= index.bit_mask;
        } else {
            cell &= !index.bit_mask;
        }
        self.cells[index.cell] = cell;
        Ok(())
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Result<u8, BoundsError> {
        let mut result = 0u8;
        for dy in -1..=1 {
            let b = add(y, dy, self.height);
            for dx in -1..=1 {
                let a = add(x, dx, self.width);
                if !(dy == 0 && dx == 0) && self.get(a, b)? {
                    result += 1;
                }
            }
        }
        Ok(result)
    }

    pub fn set_random(&mut self, bit: bool) -> Result<(), BoundsError> {
        self.set(
            random::<usize>() % self.width,
            random::<usize>() % self.height,
            bit,
        )?;
        Ok(())
    }
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

fn cells(width: usize, height: usize) -> usize {
    let bits_count = width * height;
    let mut cells = bits_count / BITS_PER_CELLS;
    if bits_count % BITS_PER_CELLS > 0 {
        cells += 1
    }
    cells
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
        if self.x >= self.cell_grid.width {
            self.y += 1;
            self.x = 0;
        }
    }
}

impl Iterator for CellGridIterator<'_> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.cell_grid.width && self.y < self.cell_grid.height {
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

// -- slices

impl CellGrid {
    pub fn split_mut(&mut self, slices: usize) -> Vec<CellGridSlice> {
        let mut result = Vec::with_capacity(slices);
        let cells_len = self.cells.len();
        let mut chunk_size = cells_len / slices;
        if cells_len % slices > 0 {
            chunk_size += 1;
        };
        let mut offset = 0;
        let mut cells_bits = self.width * self.height;
        let chunk_bits = chunk_size * BITS_PER_CELLS;
        for group in self.cells.chunks_mut(chunk_size) {
            result.push(CellGridSlice {
                offset: offset,
                len: {
                    if cells_bits < chunk_bits {
                        cells_bits
                    } else {
                        chunk_bits
                    }
                },
                width: self.width,
                height: self.height,
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

#[derive(Debug)]
pub struct CellGridSlice<'a> {
    offset: usize,
    len: usize,
    width: usize,
    height: usize,
    cells: &'a mut [usize],
}

impl CellGridSlice<'_> {
    pub fn iter(&self) -> CellGridSliceIterator {
        CellGridSliceIterator {
            current: 0,
            offset: self.offset,
            len: self.len,
            width: self.width,
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
    pub fn set(&mut self, x: usize, y: usize, bit: bool) -> Result<(), BoundsError> {
        let index = self.index(x, y)?;
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

pub struct CellGridSliceIterator {
    current: usize,
    offset: usize,
    len: usize,
    width: usize,
}

impl Iterator for CellGridSliceIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.current;
        self.current += 1;
        if c < self.len {
            let c1 = c + self.offset;
            return Some((c1 % self.width, c1 / self.width));
        }
        None
    }
}
