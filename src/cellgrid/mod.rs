mod cellgird;
mod cellgird_slice;
mod cellgrid_iterator;
mod cellgrid_slice_iterator;
mod indexer;

pub use crate::cellgrid::cellgird::CellGrid;
use crate::cellgrid::cellgird_slice::*;
pub use crate::cellgrid::indexer::BoundsError;
use crate::cellgrid::indexer::*;

type CellType = usize;
