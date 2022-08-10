#[macro_export]
macro_rules! def_rule {
    ($name:ident => $($id:ident : $type:ty),+) => {
        pub struct $name {
            $($id: std::boxed::Box<$type>,)*
        }
    };
    ($name:ident => $($untyped:ident)|* $([$variant:ident as $type:ty])|* ) => {
        pub enum $name {
            $($variant($type)),*
            $($untyped($untyped)),*
        }
    };
}

pub(crate) use def_rule;

#[macro_export]
macro_rules! def_visitor {
    // Had to adf func, because can't use macros on certain parts of the macros
    // but now it works, had to add more stuff.
    ($($name:ident : $func:ident),+) => {

        pub trait SyntaxVisitor<T> {
            $(fn $func(&mut self, arg: &$name) -> T;)+
        }

        pub trait Visitable<T> {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T;
        }

        $(impl<T> Visitable<T> for $name {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T {
                visitor.$func(&self)
            }
        })+
    }
}

pub(crate) use def_visitor;
