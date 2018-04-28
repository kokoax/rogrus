use ncurses;

use window;

#[derive(Clone)]
pub struct Message {
    pub win: window::Window,
    pub buf: Vec<String>,
}

impl Message {
    pub fn new() -> Message {
        Message {
            win: window::Window::new(0,8,8,3),
            buf: vec![
                String::from("This"),
                String::from("is"),
                String::from("message"),
            ],
        }
    }

    pub fn update(&mut self, buf: Vec<String>) {
        self.buf = buf.clone();
    }
}
impl window::WindowExt for Message {
    fn view(&self) {
        for y in (self.win.y)..(self.win.y+self.win.h) {
            ncurses::mv(y,self.win.x);
            ncurses::printw(&self.buf[(y-self.win.y) as usize]);
        }
    }
    fn as_window(&self) -> window::Window {
        self.win.clone()
    }
}
