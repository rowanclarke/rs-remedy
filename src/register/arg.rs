use super::Str;

use clap::{ArgAction, ValueHint};
use remedy_macros::into_enum;
use stabby::{slice::Slice, stabby, vtable, Dyn, DynRef};
use take_mut::take;

pub(crate) type UserArg = clap::Arg;

#[stabby]
pub trait Arg {
    extern "C" fn short(&mut self, short: u32);
    extern "C" fn long(&mut self, long: Str);
    extern "C" fn required(&mut self, yes: bool);
    extern "C" fn requires(&mut self, id: Str);
    extern "C" fn exclusive(&mut self, yes: bool);
    extern "C" fn global(&mut self, yes: bool);
    extern "C" fn action(&mut self, action: Action);
    extern "C" fn num_args(&mut self, from: usize, to: usize);
    extern "C" fn value_name(&mut self, name: Str);
    extern "C" fn value_names(&mut self, names: Slice<'static, Str>);
    extern "C" fn value_hint(&mut self, hint: Hint);
    extern "C" fn default_value(&mut self, default_value: Str);
    extern "C" fn default_values(&mut self, default_values: Slice<'static, Str>);
    extern "C" fn help(&mut self, help: Str);
    extern "C" fn long_help(&mut self, help: Str);
}

#[repr(usize)]
#[into_enum(ArgAction)]
pub enum Action {
    Set,
    Append,
    SetTrue,
    SetFalse,
    Count,
}

#[repr(usize)]
#[into_enum(ValueHint)]
pub enum Hint {
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
}

impl Arg for UserArg {
    arg_impl![long, requires, value_name, default_value, help, long_help; str];
    arg_impl![required, exclusive, global; bool];

    extern "C" fn short(&mut self, short: u32) {
        take(self, |this: Self| {
            this.short(unsafe { char::from_u32_unchecked(short) })
        })
    }

    extern "C" fn action(&mut self, action: Action) {
        take(self, |this: Self| this.action(ArgAction::from(action)))
    }

    extern "C" fn num_args(&mut self, from: usize, to: usize) {
        take(self, |this: Self| this.num_args(from..to))
    }

    extern "C" fn value_names(&mut self, names: Slice<'static, Str>) {
        take(self, |this: Self| {
            this.value_names(names.iter().map(|s| s.as_str()))
        })
    }

    extern "C" fn value_hint(&mut self, hint: Hint) {
        take(self, |this: Self| this.value_hint(ValueHint::from(hint)))
    }

    extern "C" fn default_values(&mut self, default_values: Slice<'static, Str>) {
        take(self, |this: Self| {
            this.default_values(default_values.iter().map(|s| s.as_str()))
        })
    }
}

pub type ArgMut<'a> = Dyn<'a, &'a mut (), vtable!(Arg)>;
pub type WithRefArgMut<'a> = DynRef<'a, vtable!(WithArgMut)>;

#[stabby]
pub trait WithArgMut {
    extern "C" fn call<'a>(&self, command: ArgMut<'a>);
}

impl<F: for<'a> Fn(ArgMut<'a>)> WithArgMut for F {
    extern "C" fn call<'a>(&self, arg: ArgMut<'a>) {
        self(arg)
    }
}
