pub struct Screen {
    buff: String,
}

static CLEAR_SCREEN: &str = "\x1B[2J";
static MOVE_CURSOR_TOP_LEFT: &str = "\x1B[H";

impl Screen {
    pub fn new() -> Screen {
        print!("{}", CLEAR_SCREEN);
        Screen {
            buff: String::new(),
        }
    }

    pub fn update<F>(&mut self, fill_buff: F)
    where
        F: FnOnce(&mut String),
    {
        self.buff.clear();
        fill_buff(&mut self.buff);
        self.buff.push_str(MOVE_CURSOR_TOP_LEFT);
    }

    pub fn print(&self) {
        print!("{}", self.buff);
    }

    pub fn get_size() -> (usize, usize) {
        let (w, h) = crossterm::terminal().size().unwrap();
        (w as usize, h as usize)
    }
}
