mod cellgrid;
use cellgrid::BoundsError;
use std::io::{Error, ErrorKind, Result};

impl From<BoundsError> for Error {
    fn from(be: BoundsError) -> Self {
        Error::new(
            ErrorKind::Other,
            format!("Bounds error occured: {:?}", be.kind),
        )
    }
}

mod frame_counter;
mod screen;
mod universe;
use frame_counter::FrameCounter;
use screen::Screen;
use std::sync::mpsc;
use universe::Universe;

fn main() -> Result<()> {
    let mut universe = {
        let (width, height) = Screen::get_size();
        Universe::new(width, height - 1)?
    };
    let mut screen = Screen::new();
    let mut frame_counter = FrameCounter::new();
    let mut cells = 0usize;
    loop {
        universe.update(|current| {
            screen.update(|buff| {
                buff.push_str(frame_counter.as_string().as_str());
                buff.push_str(format!(", {}‰ ", current.thousandths_set(cells)).as_str());
                cells = 0;
                buff.push('\n');
                for (_, _, alive) in current.iter() {
                    buff.push(if alive {
                        cells += 1;
                        '®'
                    } else {
                        ' '
                    });
                }
            });
        });

        let universe = &mut universe;
        let frame_counter = &mut frame_counter;
        let screen = &screen;

        rayon::scope(move |scope| {
            let (tx, rx) = mpsc::channel();
            scope.spawn(move |_| {
                while rx.try_recv().is_err() {
                    universe.tick().unwrap();
                    frame_counter.step();
                }
            });
            screen.print();
            tx.send(false).unwrap();
        });
    }
}
