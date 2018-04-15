extern crate ncurses;

use ncurses::{
    printw,
    getch,
    endwin,
    initscr,
    mv,
    curs_set,
    noecho,
    CURSOR_VISIBILITY,
};
use std::vec::Vec;
use std::string::String;

#[derive(Clone)]
struct Window {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

trait WindowExt {
    fn view(&self);
    fn clear(&self) {
        let w: Window = self.as_window();
        for x in (w.x)..(w.x+w.w) {
            for y in (w.y)..(w.y+w.h) {
                mv(y,x);
                printw(" ");
            }
        }
    }
    fn as_window(&self) -> Window;
}

#[derive(Clone)]
struct Map {
    win: Window,
    map: Vec<String>,
}

#[derive(Clone)]
struct Creature {
    win: Window,
    c: String,
}

enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
struct Message {
    win: Window,
    buf: Vec<String>,
}

impl Window {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Window {
        Window {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}
impl Map {
    fn new() -> Map {
        Map {
            win: Window::new(0,0,8,8),
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
impl WindowExt for Map {
    fn view(&self) {
        for y in (self.win.y)..(self.win.y+self.win.h) {
            mv(y,self.win.x);
            printw(&self.map[(y-self.win.y) as usize]);
        }
    }
    fn as_window(&self) -> Window {
        self.win.clone()
    }
}

impl Creature {
    fn player() -> Creature {
        Creature::new(String::from("@"))
    }
    fn new(c: String) -> Creature {
        Creature {
            win: Window::new(1,1,1,1),
            c: c
        }
    }
    fn mv(&mut self, m: &Map, d: DIRECTION) {
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
impl WindowExt for Creature {
    fn view(&self) {
        mv(self.win.y,self.win.x);
        printw(&self.c);
    }
    fn as_window(&self) -> Window {
        self.win.clone()
    }
}

impl Message {
    fn new() -> Message {
        Message {
            win: Window::new(0,8,8,3),
            buf: vec![
                String::from("This"),
                String::from("is"),
                String::from("message"),
            ],
        }
    }
    fn update(&mut self, buf: Vec<String>) {
        self.buf = buf.clone();
    }
}
impl WindowExt for Message {
    fn view(&self) {
        for y in (self.win.y)..(self.win.y+self.win.h) {
            mv(y,self.win.x);
            printw(&self.buf[(y-self.win.y) as usize]);
        }
    }
    fn as_window(&self) -> Window {
        self.win.clone()
    }
}

fn view(obj: Vec<&WindowExt>) {
    for o in obj {
        o.clear();
        o.view();
    }
}

fn look(m: &Map, p: &Creature) -> String{
    let clst = m.map[p.win.y as usize].as_bytes();
    match clst[p.win.x as usize] {
        46 => String::from("is common tile"),
        35 => String::from("is common wall"),
        _ => String::from(""),
    }
}

fn main() {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();

    let mut p = Creature::player();
    let m = Map::new();
    let mut msg = Message::new();
    view(vec![&m,&msg,&p]);

    let mut done = false;
    let mut msg_frash = vec![
        String::from(""),
        String::from(""),
        String::from(""),
    ];
    let mut msg_buf = msg_frash.clone();
    while !done {
        let key = getch();
        let key_s = key.to_string();
        msg_buf = msg_frash.clone();
        msg_buf[0] = key_s.to_string();

        match key_s.as_str() {
            "119" => p.mv(&m, DIRECTION::UP),
            "97"  => p.mv(&m, DIRECTION::LEFT),
            "115" => p.mv(&m, DIRECTION::DOWN),
            "100" => p.mv(&m, DIRECTION::RIGHT),
            "32"  => msg_buf[1] = look(&m,&p),
            "113" => break,
            _   => (),
        }

        msg.update(msg_buf.clone());
        view(vec![&m,&msg,&p]);
    }

    endwin();
}
