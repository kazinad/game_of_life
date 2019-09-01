use crate::cellgrid::{BoundsError, CellGrid};
use crate::frame_counter::FrameCounter;
use crate::screen::Screen;

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

    pub fn tick(
        &mut self,
        screen: &mut Screen,
        frame_counter: &FrameCounter,
    ) -> Result<(), BoundsError> {
        self.current.set_random(true)?;
        let current = &self.current;
        let curr = &self.current;
        let mut slices = self.next.split_mut(num_cpus::get());
        let result = crossbeam::scope(|scope| {
            let th = scope.spawn(move |_| -> Result<(), BoundsError> {
                screen.update(|buff| {
                    buff.push_str(frame_counter.as_string().as_str());
                    buff.push('\n');
                    for (_, _, alive) in curr.iter() {
                        buff.push(if alive { '®' } else { ' ' });
                    }
                });

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

            threads.push(th);

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
