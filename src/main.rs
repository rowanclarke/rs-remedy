mod args;
mod display;
mod select;

use args::{Args, Command, DeckAction, DeckAddAction, DeckCommand, SessionAction, SessionCommand};
use clap::Parser;
use core::array;
use display::DisplayCard;
use remediate::{
    deck::{ArchivedDeck, Deck},
    schedule::{sm2, Review},
    session,
    workspace::{fs::LocalWorkspace, Workspace},
};
use select::select;
use std::{
    env,
    fmt::Display,
    io::{stdin, stdout, Write},
};
use strum::{Display, EnumIter, IntoEnumIterator};
use termion::{
    clear,
    cursor::{Down, Goto, Hide, Restore, Show},
    input::TermRead,
    raw::IntoRawMode,
};

type Session = session::Session<<LocalWorkspace as Workspace>::Component, sm2::Data>;

fn main() {
    let args = Args::parse();

    let workspace = LocalWorkspace::new(env::current_dir().unwrap().to_str().unwrap().into());

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut events = stdin().events();

    match args.command {
        Command::Deck(DeckAction {
            command: DeckCommand::Add(DeckAddAction { paths }),
        }) => {
            for path in paths {
                let location = workspace.components(path);
                Deck::parse(&workspace, &location).save(&workspace, &location);
            }
        }
        Command::Session(SessionAction {
            command: SessionCommand::Initialize,
        }) => Session::new(&workspace).save(&workspace),
        Command::Session(SessionAction {
            command: SessionCommand::Learn,
        }) => {
            type Score = sm2::Score;

            #[derive(Display, EnumIter, PartialEq, Clone)]
            enum ShowCard {
                Quit,
                Show,
            }

            #[derive(PartialEq, Clone)]
            enum ReviewCard {
                Quit,
                Review(Score),
            }

            impl IntoEnumIterator for ReviewCard {
                type Iterator = array::IntoIter<Self, 7>;

                fn iter() -> Self::Iterator {
                    [
                        Self::Quit,
                        Self::Review(Score::Awful),
                        Self::Review(Score::Poor),
                        Self::Review(Score::Okay),
                        Self::Review(Score::Good),
                        Self::Review(Score::Solid),
                        Self::Review(Score::Perfect),
                    ]
                    .into_iter()
                }
            }

            impl Display for ReviewCard {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{}",
                        match self {
                            Self::Quit => "Quit",
                            Self::Review(score) => match score {
                                Score::Awful => "Awful",
                                Score::Poor => "Poor",
                                Score::Okay => "Okay",
                                Score::Good => "Good",
                                Score::Solid => "Solid",
                                Score::Perfect => "Perfect",
                            },
                        }
                    )
                }
            }

            write!(stdout, "{}", Hide).unwrap();
            let mut session = Session::load(&workspace);
            session.for_each(|entry| {
                let (id, group) = entry.id();
                let card = ArchivedDeck::load(&workspace, &entry.location()).get_card(id.as_ref());
                let mut display_card = DisplayCard::new(&card, group);
                display_card.hide();
                write!(
                    stdout,
                    "{}{}{}{}{}",
                    clear::All,
                    Goto(1, 1),
                    display_card,
                    Restore,
                    Down(1)
                )
                .unwrap();
                stdout.flush().unwrap();
                match select(&mut stdout, &mut events, ShowCard::Show) {
                    ShowCard::Quit => return true,
                    ShowCard::Show => (),
                }
                display_card.show();
                write!(
                    stdout,
                    "{}{}{}{}{}",
                    clear::All,
                    Goto(1, 1),
                    display_card,
                    Restore,
                    Down(1)
                )
                .unwrap();
                stdout.flush().unwrap();
                match select(&mut stdout, &mut events, ReviewCard::Review(Score::Good)) {
                    ReviewCard::Quit => return true,
                    ReviewCard::Review(score) => entry.data_mut().review(score),
                }
                false
            });
            session.save(&workspace);
            write!(stdout, "{}{}{}", clear::All, Goto(1, 1), Show).unwrap();
        }
    }
}
