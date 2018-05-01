use ncurses;

use object;
use window;

#[derive(Eq, PartialEq)]
pub enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait CreatureExt: window::WindowExt {
    fn player(win: &window::Window) -> Creature;
    fn new(c: String, win: &window::Window) -> Creature;
    fn mv(&self, d: DIRECTION) -> Creature;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Creature {
    pub win: window::Window,
    pub c: String,
}

impl object::ObjectExt for Creature {}

impl window::WindowExt for Creature {
    fn view(&self) {
        ncurses::mv(self.win.y,self.win.x);
        ncurses::printw(&self.c);
    }

    fn as_window(&self) -> window::Window {
        self.win.clone()
    }
}

impl CreatureExt for Creature {
    fn player(win: &window::Window) -> Creature {
        Creature::new(String::from("@"), &win.clone())
    }

    fn new(c: String, win: &window::Window) -> Creature {
        Creature {
            win: win.clone(),
            c: c,
        }
    }

    fn mv(&self, d: DIRECTION) -> Creature {
        let mut nc = self.clone();
        match d {
            DIRECTION::UP    => nc.win.y -= 1,
            DIRECTION::LEFT  => nc.win.x -= 1,
            DIRECTION::DOWN  => nc.win.y += 1,
            DIRECTION::RIGHT => nc.win.x += 1,
        }
        return nc;
    }
}

#[cfg(test)]
mod creature_tests {
    use window;
    use creature;
    use creature::CreatureExt;

    #[test]
    fn test_new() {
        let w = window::Window::new(1,1,1,1);
        let c = creature::Creature::new(String::from("i"), &w.clone());
        assert_eq!(c.win, w.clone());
        assert_eq!(c.c, "i");
    }

    #[test]
    fn test_player() {
        let win = window::Window::new(1,1,1,1);
        let p = creature::Creature::player(&win.clone());
        let c = creature::Creature::new(String::from("@"), &win.clone());
        assert_eq!(p, c);
    }

    #[test]
    fn test_equality() {
        let w = window::Window::new(1,1,1,1);
        let c1 = creature::Creature::new(String::from("i"), &w.clone());
        let c2 = creature::Creature::new(String::from("@"), &w.clone());
        let p  = creature::Creature::new(String::from("@"), &w.clone());

        assert_ne!(c1, c2);
        assert_eq!(p, c2);
        assert_ne!(p, c1);
    }

    #[test]
    fn test_mv() {
        let w    = window::Window::new(1,1,1,1);
        let c = creature::Creature::new(String::from("i"), &w.clone());
        let mut moved_c = creature::Creature::new(String::from("i"), &w.clone());
        let w_up    = window::Window::new(1,0,1,1);
        let w_down  = window::Window::new(1,2,1,1);
        let w_left  = window::Window::new(0,1,1,1);
        let w_right = window::Window::new(2,1,1,1);

        moved_c.win = w_up.clone();
        assert_eq!(c.mv(creature::DIRECTION::UP), moved_c);

        moved_c.win = w_down.clone();
        assert_eq!(c.mv(creature::DIRECTION::DOWN), moved_c);

        moved_c.win = w_left.clone();
        assert_eq!(c.mv(creature::DIRECTION::LEFT), moved_c);

        moved_c.win = w_right.clone();
        assert_eq!(c.mv(creature::DIRECTION::RIGHT), moved_c);
    }
}
