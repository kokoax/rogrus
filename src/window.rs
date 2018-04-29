use ncurses::{
    mv,
    printw,
};

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod window_tests {
    use window;

    #[test]
    fn test_new() {
        let w = window::Window::new(0,1,2,3);
        assert_eq!(w.x, 0);
        assert_eq!(w.y, 1);
        assert_eq!(w.w, 2);
        assert_eq!(w.h, 3);
    }

    #[test]
    fn test_equality() {
        let w1 = window::Window::new(0,0,0,0);
        let w2 = window::Window::new(0,0,1,1);
        let w3 = window::Window::new(0,0,0,0);

        assert_ne!(w1,w2);
        assert_eq!(w1,w3);
        assert_ne!(w2,w3);
    }
}
