mod cellgrid;
use cellgrid::{BoundsError, CellGrid};
use std::io::{Error, ErrorKind, Result};

impl From<BoundsError> for Error {
    fn from(be: BoundsError) -> Self {
        Error::new(
            ErrorKind::Other,
            format!("Bounds error occured: {:?}", be.kind),
        )
    }
}

pub fn update_screen(buff: &mut String, top: String, cell_grid: &CellGrid) -> Result<()> {
    buff.clear();
    buff.push_str(&top);
    buff.push_str("\n");
    for (_, _, alive) in cell_grid.iter() {
        buff.push(if alive { '®' } else { ' ' });
    }
    Ok(())
}

fn get_size() -> (usize, usize) {
    if let Some((w, h)) = term_size::dimensions_stdout() {
        (w, h)
    } else {
        (40, 20)
    }
}

mod frame_counter;
mod universe;
use frame_counter::FrameCounter;
use universe::Universe;

static CLEAR_SCREEN: &str = "\x1B[3J";
static MOVE_CURSOR_TOP_LEFT: &str = "\x1B[H";

fn main() -> Result<()> {
    print!("{}", CLEAR_SCREEN);
    let size = get_size();
    let mut universe = Universe::new(size.0, size.1 - 1)?;
    let mut screen = String::new();
    let mut frame_counter = FrameCounter::new();
    loop {
        update_screen(&mut screen, frame_counter.as_string(), universe.current())?;
        print!("{}{}", screen, MOVE_CURSOR_TOP_LEFT);
        universe.tick()?;
        frame_counter.step();
    }
}
