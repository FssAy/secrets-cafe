/// Different Mod tiers and permissions.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum ModTier {
    /// No additional permissions.
    #[default]
    None = 0,
    /// Can verify posts and mark them for deletion.
    Verifier = 1,
    /// Can fully remove posts without additional supervision.
    Remover = 2,
    /// Has full permissions to do everything.
    Admin = 3,
    // if adding tier above the last one make sure to update Self::is_raw_in_bounds
    /// Same as admin, but also have access to unsafe features.
    Dev = 4,
}

impl ModTier {
    fn is_raw_in_bounds(value: u8) -> bool {
        if value <= ModTier::Dev as u8 {
            true
        } else {
            false
        }
    }
}

impl From<u8> for ModTier {
    fn from(value: u8) -> Self {
        if Self::is_raw_in_bounds(value) {
            unsafe {
                std::mem::transmute(value)
            }
        } else {
            Self::default()
        }
    }
}

impl TryFrom<&String> for ModTier {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let value = u8::from_str_radix(value, 10).map_err(|_| ())?;

        if Self::is_raw_in_bounds(value) {
            Ok(Self::from(value))
        } else {
            Err(())
        }
    }
}
