use crate::cellgrid::{BoundsError, CellGrid, CellGridSlice};
use core::cmp::max;
use core::slice::Iter;

const CHANGES_HISTORY_LENGTH: usize = 5;

pub struct Universe {
    current: CellGrid,
    next: CellGrid,
    limit: usize,
    changes_history: Vec<i32>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Result<Universe, BoundsError> {
        Ok(Universe {
            current: CellGrid::new(width, height, true)?,
            next: CellGrid::new(width, height, false)?,
            limit: width * height * 10 / 1000,
            changes_history: Vec::with_capacity(CHANGES_HISTORY_LENGTH),
        })
    }

    pub fn tick<T>(&mut self, update: T) -> Result<(), BoundsError>
    where
        T: FnOnce(&CellGrid) + std::marker::Send,
    {
        let current = &self.current;
        let current_for_update = &self.current;
        let mut slices = self.next.split_mut(num_cpus::get());

        let scope = crossbeam::scope(|scope| {
            let update_thread = scope.spawn(move |_| {
                update(current_for_update);
                Ok(SliceCalc(0usize, 0i32))
            });

            let mut threads: Vec<_> = slices
                .iter_mut()
                .map(|slice| scope.spawn(move |_| calc_slice(current, slice)))
                .collect();

            threads.push(update_thread);

            threads
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .collect::<Result<Vec<SliceCalc>, BoundsError>>()
        });

        std::mem::swap(&mut self.current, &mut self.next);

        match scope {
            Ok(slice_calcs) => {
                let SliceCalc(cells, changes) = slice_calcs?.iter().sum_slice_calcs();
                if self.is_stable(changes) {
                    for _ in cells..max(self.limit, cells + 1) {
                        self.current.set_random(true)?;
                    }
                }
                Ok(())
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    fn is_stable(&mut self, changes: i32) -> bool {
        self.changes_history.push(changes);
        while self.changes_history.len() >= CHANGES_HISTORY_LENGTH {
            self.changes_history.remove(0);
        }
        for i in 0..self.changes_history.len() {
            if self.changes_history.iter().skip(i).sum::<i32>() == 0i32 {
                return true;
            }
        }
        false
    }
}

struct SliceCalc(usize, i32);

trait SumSliceCalcs {
    fn sum_slice_calcs(self) -> SliceCalc;
}

impl SumSliceCalcs for Iter<'_, SliceCalc> {
    fn sum_slice_calcs(self) -> SliceCalc {
        self.fold(SliceCalc(0usize, 0i32), |acc, i| {
            SliceCalc(acc.0 + i.0, acc.1 + i.1)
        })
    }
}

fn calc_slice(current: &CellGrid, slice: &mut CellGridSlice) -> Result<SliceCalc, BoundsError> {
    let mut cells = 0usize;
    let mut changes = 0i32;
    for (x, y) in slice.iter() {
        let alive = current.get(x, y)?;
        let neighbours = current.neighbours(x, y)?;
        let survive = survive(alive, neighbours);
        slice.set(x, y, survive)?;
        if survive {
            cells += 1;
        }
        if survive != alive {
            if survive {
                changes += 1;
            } else {
                changes -= 1;
            }
        }
    }
    Ok(SliceCalc(cells, changes))
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
