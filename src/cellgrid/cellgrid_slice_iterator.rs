use super::*;

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
