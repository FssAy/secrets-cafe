#[cfg(debug_assertions)]
pub mod debug;

use command_engine::{*, shared::*};

macro_rules! get_arg {
    ($args:ident, $pos:literal, $err:expr) => {
        if let Some(arg) = $args.get($pos) {
            arg
        } else {
            return $err;
        }
    };
}

pub(crate) use get_arg;
