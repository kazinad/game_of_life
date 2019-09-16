use super::*;

use rand::prelude::*;

pub(super) static BITS_PER_CELLS: usize = std::mem::size_of::<CellType>() * 8;

pub(super) struct Indexer {
    pub(super) width: usize,
    pub(super) height: usize,
    pub(super) offset: usize,
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

pub(super) struct BitIndex {
    pub(super) cell: usize,
    pub(super) bit_mask: CellType,
}

impl Indexer {
    pub(super) fn new(width: usize, height: usize, offset: usize) -> Indexer {
        Indexer {
            width: width,
            height: height,
            offset: offset,
        }
    }
    pub(super) fn index(&self, x: usize, y: usize) -> Result<BitIndex, BoundsError> {
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

    pub(super) fn pos(&self, index: usize) -> (usize, usize) {
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

    pub(super) fn add_x(&self, x: usize, dx: isize) -> usize {
        Indexer::add(x, dx, self.width)
    }

    pub(super) fn add_y(&self, y: usize, dy: isize) -> usize {
        Indexer::add(y, dy, self.height)
    }

    pub(super) fn random(&self) -> (usize, usize) {
        (
            random::<usize>() % self.width,
            random::<usize>() % self.height,
        )
    }

    pub(super) fn cell_count(&self) -> usize {
        let bits_count = self.width * self.height;
        let mut cell_count = bits_count / BITS_PER_CELLS;
        if bits_count % BITS_PER_CELLS > 0 {
            cell_count += 1
        }
        cell_count
    }
}
