extern crate termion;

use std::io;
use std::io::prelude::*;
use termion::raw::IntoRawMode;
use termion::input::{TermRead, MouseTerminal};
use termion::event::{Event, Key, MouseEvent, MouseButton};
use termion::cursor;
use termion::color::*;

pub fn read_line(prompt: &str) -> io::Result<String> {
    fn print<W: Write>(stdout: &mut W, prompt: &str, buf: &str, loc: usize, msg: &str) -> io::Result<()> {
        use termion::clear;
        write!(stdout, "\r{}{}{}", clear::CurrentLine, prompt, cursor::Hide)?;
        for (i, chr) in buf.chars()
            .chain(Some(' ')) // Space to show cursor even when at the end
            .enumerate() {
            if i == loc {
                write!(stdout, "{}{}", Fg(Black), Bg(White))?;
            }
            write!(stdout, "{}", chr)?;
            if i == loc {
                write!(stdout, "{}{}", Fg(Reset), Bg(Reset))?;
            }
        }
        write!(stdout, "\t\t{}{}{}", Fg(Yellow), msg, Fg(Reset))?;
        stdout.flush()
    }

    let mut log_file = ::std::fs::OpenOptions::new().write(true).open("abc.txt").unwrap();
    let mut buf = String::new();
    let mut loc = 0;
    let mut msg;
    let stdin = io::stdin();
    let mut stdout: MouseTerminal<_> = io::stdout().into_raw_mode().unwrap().into();
    print(&mut stdout, prompt, &buf, loc, "").unwrap();

    for event in stdin.events() {
        msg = "";
        let event = event?;
        match event {
            Event::Key(key) => match key {
                Key::Char(c) => {
                    if c != '\n'{
                        buf.insert(loc, c);
                        loc += 1;
                    }
                }
                Key::Backspace => {
                    if loc > 0 {
                        buf.remove(loc - 1);
                        loc -= 1;
                    }
                }
                Key::Delete => {
                    if loc != buf.len() {
                        buf.remove(loc);
                    }
                }
                Key::Home => {
                    loc = 0;
                }
                Key::End => {
                    loc = buf.len();
                }
                Key::Left => {
                    if loc > 0 {
                        loc -= 1;
                    }
                }
                Key::Right => {
                    if loc < buf.len() {
                        loc += 1;
                    }
                }
                Key::Insert => {
                    msg = "Insert key is not supported";
                }
                _ => {
                    ::std::mem::drop(stdout);
                    panic!();
                }
            },
            Event::Mouse(mouse) => {
                match mouse {
                    MouseEvent::Press(MouseButton::Left, x, y) => {
                        write!(log_file, "{} {}\n", x, y).unwrap();
                        if (x as usize) < prompt.len() {
                            loc = 0;
                        } else {
                            loc = x as usize - prompt.len() - 1;
                            if loc > buf.len() {
                                loc = buf.len();
                            }
                        }
                    }
                    MouseEvent::Press(_, _, _) => {
                        msg = "Mouse button is not supported";
                    }
                    MouseEvent::Release(_, _) => {}
                    MouseEvent::Hold(_, _) => {}
                }
            }
            _ => {
                ::std::mem::drop(stdout);
                panic!();
            }
        }
        print(&mut stdout, prompt, &buf, loc, msg).unwrap();
        stdout.flush().unwrap();
        if let Event::Key(Key::Char('\n')) = event {
            write!(stdout, "\r\n{}", cursor::Show).unwrap();
            return Ok(buf);
        }
    }
    unreachable!();
}
