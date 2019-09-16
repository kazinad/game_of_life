mod cellgrid;
mod cellgrid_iterator;
mod cellgrid_slice;
mod cellgrid_slice_iterator;
mod indexer;

pub use crate::cellgrid::cellgrid::CellGrid;
use crate::cellgrid::cellgrid_slice::*;
pub use crate::cellgrid::indexer::BoundsError;
use crate::cellgrid::indexer::*;

type CellType = usize;
