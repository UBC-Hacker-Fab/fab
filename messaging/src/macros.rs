macro_rules! messages {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            $item
        )*
    };
}
pub(crate) use messages;

macro_rules! protocol {
    ($vis:vis $frame:ident { $($rest:tt)* }) => {
        paste::paste! {
            protocol!($vis $frame : [<$frame Trait>] { $($rest)* });
        }
    };
    ($vis:vis $frame:ident : $traitname:ident { $($name:ident),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
        $vis enum $frame {
            $( $name($name), )*
        }

        impl $frame {
            $vis fn name(&self) -> &'static str {
                match self { $( $frame::$name(_) => <$name as $traitname>::NAME, )* }
            }

            $vis fn index(&self) -> usize {
                match self { $( $frame::$name(_) => <$name as $traitname>::INDEX, )* }
            }

            $vis const COUNT: usize = [$( stringify!($name) ),*].len();
        }

        $vis trait $traitname :
            ::serde::Serialize
            + ::serde::de::DeserializeOwned
            + Into<$frame>
            + ::core::convert::TryFrom<$frame, Error = $frame>
            + ::core::fmt::Debug
            + Clone
            + Send
            + 'static
        {
            const NAME: &'static str;
            const INDEX: usize;
        }

        // Assign INDEX.
        protocol!(@indices $traitname, 0usize; $($name)*);

        $(
            impl From<$name> for $frame { fn from(v: $name) -> $frame { $frame::$name(v) } }

            impl ::core::convert::TryFrom<$frame> for $name {
                type Error = $frame;
                fn try_from(f: $frame) -> ::core::result::Result<Self, $frame> {
                    match f { $frame::$name(v) => Ok(v), _ => Err(f) }
                }
            }
        )*
    };

    // internal: recursively assign 0, 1, 2, ... to each name in order
    (@indices $traitname:ident, $i:expr; ) => {};
    (@indices $traitname:ident, $i:expr; $head:ident $($tail:ident)*) => {
        impl $traitname for $head {
            const NAME: &'static str = stringify!($head);
            const INDEX: usize = $i;
        }
        protocol!(@indices $traitname, $i + 1usize; $($tail)*);
    };
}
pub(crate) use protocol;

#[cfg(test)]
mod tests;
