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

    pub fn tick<T>(&mut self, update: T) -> Result<(), BoundsError>
    where
        T: FnOnce(&CellGrid) + std::marker::Send,
    {
        self.current.set_random(true)?;
        let current = &self.current;
        let curr = &self.current;
        let mut slices = self.next.split_mut(num_cpus::get());
        let result = crossbeam::scope(|scope| {
            let update_thread = scope.spawn(move |_| -> Result<(), BoundsError> {
                update(curr);
                Ok(())
            });

            let mut threads: Vec<_> = slices
                .iter_mut()
                .map(|slice| {
                    scope.spawn(move |_| -> Result<(), BoundsError> {
                        for (x, y) in slice.iter() {
                            let alive = current.get(x, y)?;
                            let neighbours = current.neighbours(x, y)?;
                            slice.set(x, y, survive(alive, neighbours))?;
                        }
                        Ok(())
                    })
                })
                .collect();

            threads.push(update_thread);

            threads
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .collect::<Result<(), BoundsError>>()
        });
        std::mem::swap(&mut self.current, &mut self.next);
        match result {
            Ok(ok) => ok,
            Err(e) => panic!("{:?}", e),
        }
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
