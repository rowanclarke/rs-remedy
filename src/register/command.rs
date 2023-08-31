use super::{arg::*, Str};

use stabby::{stabby, vtable, Dyn, DynRef};
use take_mut::take;

pub(crate) type UserCommand = clap::Command;

#[stabby]
pub trait Command {
    extern "C" fn subcommand<'a>(&mut self, name: Str, then: WithRefCommandMut<'a>);
    extern "C" fn arg<'a>(&mut self, name: Str, then: WithRefArgMut<'a>);
    extern "C" fn author(&mut self, author: Str);
    extern "C" fn version(&mut self, version: Str);
    extern "C" fn about(&mut self, about: Str);
    extern "C" fn long_about(&mut self, long_about: Str);
    extern "C" fn before_help(&mut self, before_help: Str);
    extern "C" fn after_help(&mut self, after_help: Str);
    extern "C" fn before_long_help(&mut self, before_long_help: Str);
    extern "C" fn after_long_help(&mut self, after_long_help: Str);
}

impl Command for UserCommand {
    extern "C" fn subcommand<'a>(&mut self, name: Str, then: WithRefCommandMut<'a>) {
        let mut subcommand = UserCommand::new(name.as_str());
        then.call((&mut subcommand).into());
        take(self, |command: UserCommand| command.subcommand(subcommand));
    }

    extern "C" fn arg<'a>(&mut self, name: Str, then: WithRefArgMut<'a>) {
        let mut arg = UserArg::new(name.as_str());
        then.call((&mut arg).into());
        take(self, |command: Self| command.arg(arg));
    }

    arg_impl![
        author,
        version,
        about,
        long_about,
        before_help,
        after_help,
        before_long_help,
        after_long_help; str
    ];
}

pub type CommandMut<'a> = Dyn<'a, &'a mut (), vtable!(Command)>;
pub type WithRefCommandMut<'a> = DynRef<'a, vtable!(WithCommandMut)>;

#[stabby]
pub trait WithCommandMut {
    extern "C" fn call<'a>(&self, command: CommandMut<'a>);
}

impl<F: for<'a> Fn(CommandMut<'a>)> WithCommandMut for F {
    extern "C" fn call<'a>(&self, command: CommandMut<'a>) {
        self(command)
    }
}
