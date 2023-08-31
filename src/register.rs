macro_rules! arg_impl {
    ($($x:ident),*; str) => {
        $(extern "C" fn $x(&mut self, $x: Str) {
            take(self, |this: Self| this.$x($x.as_str()));
        })*
    };
    ($($x:ident),*; $ty:ty) => {
        $(extern "C" fn $x(&mut self, $x: $ty) {
            take(self, |this: Self| this.$x($x));
        })*
    };
}

pub mod arg;
pub mod command;

type Str = stabby::str::Str<'static>;
