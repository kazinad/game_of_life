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

    pub fn iter(&self) -> CellGridIterator {
        CellGridIterator {
            x: 0,
            y: 0,
            cell_grid: self,
        }
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
            let pos = (self.x, self.y);
            self.step();

            match self.cell_grid.get(pos.0, pos.1) {
                Ok(b) => return Some((pos.0, pos.1, b)),
                Err(_) => return None,
            }
        }
        None
    }
}
