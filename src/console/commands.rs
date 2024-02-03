#[cfg(debug_assertions)]
pub mod debug;

mod database;
mod reload;

use command_engine::{*, shared::*};
pub use database::*;
pub use reload::*;

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
