use ncurses;

use window;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    pub win: window::Window,
    pub buf: Vec<String>,
}

impl Message {
    pub fn new(win: &window::Window) -> Message {
        Message {
            win: win.clone(),
            buf: vec![String::from(""); win.y as usize],
        }
    }

    pub fn new_with_buf(win: &window::Window, buf: &Vec<String>) -> Result<Message, String> {
        if (buf.len() as i32) == win.h {
            return Ok(Message {
                win: win.clone(),
                buf: buf.clone(),
            });
        } else {
            return Err(String::from("Error: Not match window height and buffer length"));
        }
    }

    pub fn update(&self, buf: &Vec<String>) -> Result<Message, String> {
        return Message::new_with_buf(&self.win, &buf);
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
        return self.win.clone();
    }
}

#[cfg(test)]
mod message_tests {
    use window;
    use message;

    #[test]
    fn test_new() {
        let win = window::Window::new(1,1,1,1);
        let m = message::Message::new(&win);

        assert_eq!(m.win, win);
        assert_eq!(m.buf, vec![String::from("")]);
    }

    #[test]
    fn test_new_with_buf() {
        let win = window::Window::new(1,1,1,1);
        let buf = vec![String::from("###")];
        let m = message::Message::new_with_buf(&win, &buf).unwrap();

        assert_eq!(m.win, win.clone());
        assert_eq!(m.buf, buf.clone());
    }

    #[test]
    fn test_equality() {
        let win1 = window::Window::new(1,1,1,1);
        let win2 = window::Window::new(1,1,0,0);
        let m1 = message::Message::new(&win1);
        let m2 = message::Message::new(&win2);
        let m3 = message::Message::new(&win1);

        assert_ne!(m1, m2);
        assert_eq!(m1, m3);
        assert_ne!(m2, m3);
    }

    #[test]
    fn test_update() {
        let win = window::Window::new(1,1,3,1);
        let m1 = message::Message::new(&win);
        let m2 = message::Message::new_with_buf(&win, &vec![String::from("###")]).unwrap();
        let updated_m = m1.clone().update(&vec![String::from("###")]).unwrap();

        assert_ne!(updated_m, m1);
        assert_eq!(updated_m, m2);
    }
}
