extern crate rogrus;

use std::vec::Vec;
use std::string::String;

use rogrus::ncurses::{
    getch,
    endwin,
    initscr,
    curs_set,
    noecho,
    CURSOR_VISIBILITY,
};

use rogrus::window;
use rogrus::map;
use rogrus::creature;
use rogrus::message;

fn view(obj: Vec<&window::WindowExt>) {
    for o in obj {
        o.clear();
        o.view();
    }
}

fn look(m: &map::Map, p: &creature::Creature) -> String{
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

    let mut p = creature::Creature::player();
    let m = map::Map::new();
    let mut msg = message::Message::new();
    view(vec![&m,&msg,&p]);

    let mut done = false;
    let msg_frash = vec![
        String::from(""),
        String::from(""),
        String::from(""),
    ];
    let mut msg_buf: Vec<String>;
    while !done {
        let key = getch();
        let key_s = key.to_string();
        msg_buf = msg_frash.clone();
        msg_buf[0] = key_s.to_string();

        match key_s.as_str() {
            "119" => p.mv(&m, creature::DIRECTION::UP),
            "97"  => p.mv(&m, creature::DIRECTION::LEFT),
            "115" => p.mv(&m, creature::DIRECTION::DOWN),
            "100" => p.mv(&m, creature::DIRECTION::RIGHT),
            "32"  => msg_buf[1] = look(&m,&p),
            "113" => done = true,
            _   => (),
        }

        msg.update(msg_buf.clone());
        view(vec![&m,&msg,&p]);
    }

    endwin();
}
