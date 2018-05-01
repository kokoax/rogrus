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
use rogrus::window::WindowExt;
use rogrus::map;
use rogrus::map::MapExt;
use rogrus::creature;
use rogrus::creature::CreatureExt;
use rogrus::message;
use rogrus::message::MessageExt;

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

    let mut p = creature::Creature::player(&window::Window::new(1,1,1,1));
    let m = map::Map::new(&window::Window::new(0,0,16,16), &vec![
                          String::from("########        "),
                          String::from("#......#        "),
                          String::from("#..##..#        "),
                          String::from("#..#####        "),
                          String::from("##.#####        "),
                          String::from("##....##        "),
                          String::from("##....##        "),
                          String::from("########        "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
                          String::from("                "),
    ]);
    let mut msg = message::Message::new(&window::Window::new(0,16,16,5));
    view(vec![&m,&msg,&p]);

    let mut done = false;
    let msg_frash = vec![
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    ];
    let mut msg_buf: Vec<String>;
    while !done {
        let key = getch();
        msg_buf = msg_frash.clone();
        msg_buf[0] = key.to_string();

        match key {
            107 => if m.is_move_possible(&p.as_window(), creature::DIRECTION::UP)    { p = p.mv(creature::DIRECTION::UP) },
            104 => if m.is_move_possible(&p.as_window(), creature::DIRECTION::LEFT)  { p = p.mv(creature::DIRECTION::LEFT) },
            106 => if m.is_move_possible(&p.as_window(), creature::DIRECTION::DOWN)  { p = p.mv(creature::DIRECTION::DOWN) },
            108 => if m.is_move_possible(&p.as_window(), creature::DIRECTION::RIGHT) { p = p.mv(creature::DIRECTION::RIGHT) },
            32  => msg_buf[1] = look(&m,&p),
            113 => done = true,
            _   => (),
        }

        msg = msg.update(&msg_buf).unwrap();
        view(vec![&m,&msg,&p]);
    }

    endwin();
}
