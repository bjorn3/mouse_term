extern crate termion;

use std::io;
use std::io::prelude::*;
use termion::raw::IntoRawMode;
use termion::input::{TermRead, MouseTerminal};
use termion::event::{Event, Key, MouseEvent, MouseButton};
use termion::cursor;
use termion::color::*;

pub fn read_line(prompt: &str) -> io::Result<String> {
    fn print<W: Write>(stdout: &mut W, prompt: &str, buf: &str, loc: usize) {
        write!(stdout, "\r{}{}{}", ::termion::clear::CurrentLine, prompt, cursor::Hide).unwrap();
        for (i, chr) in buf.chars().enumerate() {
            if i == loc {
                write!(stdout, "{}{}", Fg(Black), Bg(White)).unwrap();
            }
            write!(stdout, "{}", chr).unwrap();
            if i == loc {
                write!(stdout, "{}{}", Fg(Reset), Bg(Reset)).unwrap();
            }
        }
        stdout.flush().unwrap();
    }

    let mut log_file = ::std::fs::OpenOptions::new().write(true).open("abc.txt").unwrap();
    let mut buf = " ".to_string(); // Space to show cursor even when at the end
    let mut loc = 0;
    let stdin = io::stdin();
    let mut stdout: MouseTerminal<_> = io::stdout().into_raw_mode().unwrap().into();
    print(&mut stdout, prompt, &buf, loc);

    for event in stdin.events() {
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
                Key::Left => {
                    if loc > 0 {
                        loc -= 1;
                    }
                }
                Key::Right => {
                    if loc < buf.len() - 1 {
                        loc += 1;
                    }
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
                                loc = buf.len() - 1;
                            }
                        }
                    }
                    MouseEvent::Press(_, _, _) => {}
                    MouseEvent::Release(_, _) => {}
                    MouseEvent::Hold(_, _) => {}
                }
            }
            _ => {
                ::std::mem::drop(stdout);
                panic!();
            }
        }
        print(&mut stdout, prompt, &buf, loc);
        stdout.flush().unwrap();
        if let Event::Key(Key::Char('\n')) = event {
            write!(stdout, "\n\r{}", cursor::Show).unwrap();
            buf.pop(); // Remove trailing space
            return Ok(buf);
        }
    }
    unreachable!();
}
