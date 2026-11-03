#[macro_export]
macro_rules! message_traits {
    ($($item:item)*) => {
        $(
        #[derive(Debug, Clone, PartialEq, crate::serde::Serialize, crate::serde::Deserialize)]
            $item
        )*
    };
}

#[macro_export]
macro_rules! message {
    ($vis:vis $message:ident { $($rest:tt)* }) => {
        paste::paste! {
            message!($vis $message : [<$message Trait>] { $($rest)* });
        }
    };
    ($vis:vis $message:ident : $traitname:ident { $($name:ident),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, crate::serde::Serialize, crate::serde::Deserialize)]
        $vis enum $message {
            $( $name($name), )*
        }

        impl $message {
            $vis fn name(&self) -> &'static str {
                match self { $( $message::$name(_) => <$name as $traitname>::NAME, )* }
            }

            $vis fn index(&self) -> usize {
                match self { $( $message::$name(_) => <$name as $traitname>::INDEX, )* }
            }

            $vis const COUNT: usize = [$( stringify!($name) ),*].len();
        }

        $vis trait $traitname :
            ::serde::Serialize
            + ::serde::de::DeserializeOwned
            + Into<$message>
            + ::core::convert::TryFrom<$message, Error = $message>
            + ::core::fmt::Debug
            + Clone
            + Send
            + 'static
        {
            const NAME: &'static str;
            const INDEX: usize;
        }

        // Assign INDEX.
        message!(@indices $traitname, 0usize; $($name)*);

        $(
            impl From<$name> for $message { fn from(v: $name) -> $message { $message::$name(v) } }

            impl ::core::convert::TryFrom<$message> for $name {
                type Error = $message;
                fn try_from(f: $message) -> ::core::result::Result<Self, $message> {
                    match f { $message::$name(v) => Ok(v), _ => Err(f) }
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
        message!(@indices $traitname, $i + 1usize; $($tail)*);
    };
}

#[cfg(test)]
mod tests;
