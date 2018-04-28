use ncurses::{
    mv,
    printw,
};

use window;

#[derive(Clone)]
pub struct Map {
    pub win: window::Window,
    pub map: Vec<String>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            win: window::Window::new(0,0,8,8),
            map: vec![
                String::from("########"),
                String::from("#......#"),
                String::from("#..##..#"),
                String::from("#..#####"),
                String::from("##.#####"),
                String::from("##....##"),
                String::from("##....##"),
                String::from("########"),
            ],
        }
    }
}
impl window::WindowExt for Map {
    fn view(&self) {
        for y in (self.win.y)..(self.win.y+self.win.h) {
            mv(y,self.win.x);
            printw(&self.map[(y-self.win.y) as usize]);
        }
    }

    fn as_window(&self) -> window::Window {
        self.win.clone()
    }
}
