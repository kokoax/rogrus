use std;

use ncurses::{
    mv,
    printw,
};

use object;
use window;
use creature;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    pub win: window::Window,
    pub map: Vec<String>,
}

pub trait MapExt: window::WindowExt {
    fn new(win: &window::Window, map: &Vec<String>) -> Map;
    fn is_move_possible(&self, win: &window::Window, dir: creature::DIRECTION) -> bool;
}

impl MapExt for Map {
    fn new(win: &window::Window, map: &Vec<String>) -> Map {
        Map {
            win: win.clone(),
            map: map.clone(),
        }
    }

    fn is_move_possible(&self, win: &window::Window, dir: creature::DIRECTION) -> bool {
        let next_tile = match dir {
            creature::DIRECTION::UP    => self.map[(win.y-1) as usize].as_bytes()[win.x as usize],
            creature::DIRECTION::DOWN  => self.map[(win.y+1) as usize].as_bytes()[win.x as usize],
            creature::DIRECTION::LEFT  => self.map[win.y as usize].as_bytes()[(win.x-1) as usize],
            creature::DIRECTION::RIGHT => self.map[win.y as usize].as_bytes()[(win.x+1) as usize],
        };
        match std::str::from_utf8(&[next_tile]).unwrap() {
            "." => true,
            _   => false,
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

impl object::ObjectExt for Map {}

#[cfg(test)]
mod map_tests {
    use window;
    use map;
    use map::MapExt;
    use creature;

    #[test]
    fn test_new() {
        let win = window::Window::new(0,1,2,3);
        let map_vec = vec![String::from("###"), String::from("#.#"), String::from("###")];
        let m = map::Map::new(&win, &map_vec);

        assert_eq!(m.win, win);
        assert_eq!(m.map, map_vec);
    }

    #[test]
    fn test_equality() {
        let win = window::Window::new(0,1,2,3);
        let map_vec1 = vec![String::from("###"), String::from("#.#"), String::from("###")];
        let map_vec2 = vec![String::from("#.#"), String::from("#.#"), String::from("#.#")];
        let m1 = map::Map::new(&win, &map_vec1);
        let m2 = map::Map::new(&win, &map_vec2);
        let m3 = map::Map::new(&win, &map_vec1);

        assert_ne!(m1,m2);
        assert_eq!(m1,m3);
        assert_ne!(m2,m3);
    }

    #[test]
    fn test_is_move_possible() {
        let win = window::Window::new(1,1,1,1);
        let map_vec = vec![String::from("#.#"), String::from("..#"), String::from("###")];
        let m = map::Map::new(&win, &map_vec);

        assert_eq!(true,  m.is_move_possible(&win, creature::DIRECTION::UP));
        assert_eq!(false, m.is_move_possible(&win, creature::DIRECTION::DOWN));
        assert_eq!(true,  m.is_move_possible(&win, creature::DIRECTION::LEFT));
        assert_eq!(false, m.is_move_possible(&win, creature::DIRECTION::RIGHT));
    }

    #[test]
    fn test_is_move_possible_unbound_vec() {
        let win = window::Window::new(0,0,1,1);
        let map_vec = vec![String::from("#.#"), String::from("..#"), String::from("###")];
        let m = map::Map::new(&win, &map_vec);

        m.is_move_possible(&win, creature::DIRECTION::UP);
        m.is_move_possible(&win, creature::DIRECTION::LEFT);
        m.is_move_possible(&win, creature::DIRECTION::DOWN);
        assert_eq!(true, true);
    }
}
