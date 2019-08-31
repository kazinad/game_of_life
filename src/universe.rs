use crate::cellgrid::{BoundsError, CellGrid};

pub struct Universe {
    current: CellGrid,
    next: CellGrid,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Result<Universe, BoundsError> {
        Ok(Universe {
            current: CellGrid::new(width, height, true)?,
            next: CellGrid::new(width, height, false)?,
        })
    }

    pub fn current(&mut self) -> &CellGrid {
        &self.current
    }

    pub fn tick(&mut self) -> Result<(), BoundsError> {
        self.current.set_random(true)?;
        for (x, y, alive, neighbours) in self.current.iter_with_neighbours() {
            self.next.set(x, y, survive(alive, neighbours))?;
        }
        std::mem::swap(&mut self.current, &mut self.next);
        Ok(())
    }
}

fn survive(alive: bool, neighbours: u8) -> bool {
    match alive {
        true => match neighbours {
            2..=3 => true,
            _ => false,
        },
        false => match neighbours {
            3 => true,
            _ => false,
        },
    }
}
