use std::{
    fmt::Display,
    io::{Read, Write},
};
use strum::IntoEnumIterator;
use termion::{
    clear,
    color::{Bg, Black, Fg, White},
    cursor::{Restore, Save},
    event::{Event, Key},
    input::{Events},
    style::Reset,
};

struct Select<E> {
    selected: usize,
    selection: Vec<E>,
}

impl<E: Display> Display for Select<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.selection.iter().enumerate().peekable();
        while let Some((i, item)) = iter.next() {
            if i == self.selected {
                write!(f, "{}{}", Bg(White), Fg(Black))?;
            }
            write!(f, "{}{}", item, Reset)?;
            if iter.peek().is_some() {
                write!(f, " | ")?;
            }
        }
        Ok(())
    }
}

impl<E: IntoEnumIterator + PartialEq> From<E> for Select<E> {
    fn from(value: E) -> Self {
        Self {
            selected: E::iter().position(|e| &e == &value).unwrap(),
            selection: E::iter().collect::<Vec<_>>(),
        }
    }
}

pub fn select<E: Display + IntoEnumIterator + PartialEq + Clone, W: Write, R: Read>(
    stdout: &mut W,
    events: &mut Events<R>,
    selected: E,
) -> E {
    let mut select = Select::from(selected);
    let mut before = select.selected + 1;
    loop {
        if before != select.selected {
            write!(
                stdout,
                "{}{}{}{}",
                clear::CurrentLine,
                Save,
                select,
                Restore
            )
            .unwrap();
            stdout.flush().unwrap();
        }
        before = select.selected;
        select.selected = before
            .checked_add_signed(match events.next().and_then(|e| e.ok()) {
                Some(Event::Key(key)) => match key {
                    Key::Left if select.selected > 0 => -1,
                    Key::Right if select.selected < select.selection.len() - 1 => 1,
                    Key::Char('\n') => break,
                    _ => 0,
                },
                _ => 0,
            })
            .unwrap();
    }
    select.selection[select.selected].clone()
}
