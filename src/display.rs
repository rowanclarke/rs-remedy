use remediate::deck::{Card, Content, Group, Text};
use std::fmt::{self, Display};
use termion::{
    cursor::{Down, Restore, Save},
    style::{self, Bold},
};

pub enum DisplayCardStatus {
    Show,
    Hide,
}

pub struct DisplayCard<'a> {
    card: &'a Card,
    group: Group,
    status: DisplayCardStatus,
}

impl<'a> DisplayCard<'a> {
    pub fn new(card: &'a Card, group: Group) -> Self {
        Self {
            card,
            group,
            status: DisplayCardStatus::Hide,
        }
    }

    pub fn show(&mut self) {
        self.status = DisplayCardStatus::Show;
    }

    pub fn hide(&mut self) {
        self.status = DisplayCardStatus::Hide;
    }
}

struct DisplayText {
    text: Text,
    depth: usize,
}

impl DisplayText {
    fn new(text: Text, depth: usize) -> Self {
        Self { text, depth }
    }
}

impl Display for DisplayText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = self.text.as_ref().split('\n').peekable();
        while let Some(line) = lines.next() {
            write!(f, "{}", line)?;
            if lines.peek().is_some() {
                write!(
                    f,
                    "{}{}{}{}",
                    Restore,
                    Down(1),
                    Save,
                    "  ".repeat(self.depth + 1)
                )?;
            }
        }
        Ok(())
    }
}

impl<'a> Display for DisplayCard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cards = self.card.rems().iter().peekable();
        while let Some((depth, rem)) = cards.next() {
            write!(f, "{}{}", Save, "  ".repeat(*depth))?;
            for content in rem {
                match content {
                    Content::Closure(group, text) if group == &self.group => match &self.status {
                        DisplayCardStatus::Show => write!(
                            f,
                            "{}({})[{}]{}",
                            Bold,
                            group.1,
                            DisplayText::new(text.clone(), *depth),
                            style::Reset
                        )?,
                        DisplayCardStatus::Hide => {
                            write!(f, "{}({})[...]{}", Bold, group.1, style::Reset)?
                        }
                    },
                    Content::Text(text) | Content::Closure(_, text) => {
                        write!(f, "{}", DisplayText::new(text.clone(), *depth))?
                    }
                }
            }
            if cards.peek().is_some() {
                write!(f, "{}{}", Restore, Down(1))?;
            }
        }
        Ok(())
    }
}
