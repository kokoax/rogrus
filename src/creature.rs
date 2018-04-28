use ncurses;

use window;
use map;

pub enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Creature {
    pub win: window::Window,
    pub c: String,
}

impl Creature {
    pub fn player() -> Creature {
        Creature::new(String::from("@"))
    }

    pub fn new(c: String) -> Creature {
        Creature {
            win: window::Window::new(1,1,1,1),
            c: c
        }
    }

    pub fn mv(&mut self, m: &map::Map, d: DIRECTION) {
        let mut nc = self.clone();
        match d {
            DIRECTION::UP    => nc.win.y -= 1,
            DIRECTION::LEFT  => nc.win.x -= 1,
            DIRECTION::DOWN  => nc.win.y += 1,
            DIRECTION::RIGHT => nc.win.x += 1,
        }
        if m.map[nc.win.y as usize].as_bytes()[nc.win.x as usize] == 46 {
            self.win.x = nc.win.x;
            self.win.y = nc.win.y;
        }
    }
}
impl window::WindowExt for Creature {
    fn view(&self) {
        ncurses::mv(self.win.y,self.win.x);
        ncurses::printw(&self.c);
    }

    fn as_window(&self) -> window::Window {
        self.win.clone()
    }
}
