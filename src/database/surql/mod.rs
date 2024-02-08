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
    create_post,
    create_mod,
    get_mod_login_info,
    update_mod_tier,
    update_mod_phash,
    get_post,
    get_post_random,
    get_post_unverified,
    get_position_current,
    relate_mod_verified,
    relate_mod_rejected,
}
