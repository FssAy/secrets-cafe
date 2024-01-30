macro_rules! surql {
    ( $( $name:ident $(,)* )+ ) => {
        paste::item! {
            $(
                pub static [<$name:upper>] : &str = include_str!(
                    stringify!([<$name:lower>].surql)
                );
            )+
        }
    };
}

surql! {
    build,
}
