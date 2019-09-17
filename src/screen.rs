pub struct Screen {
    buff: String,
}

static CLEAR_SCREEN: &str = "\x1B[3J";
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
        print!("{}", self.buff);
    }

    pub fn get_size() -> (usize, usize) {
        if let Some((w, h)) = term_size::dimensions_stdout() {
            (w, h)
        } else {
            (40, 20)
        }
    }
}
