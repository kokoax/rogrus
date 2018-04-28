use ncurses::{
    mv,
    printw,
};

#[derive(Clone)]
pub struct Window {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub trait WindowExt {
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

impl Window {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Window {
        Window {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}
