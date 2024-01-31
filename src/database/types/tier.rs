/// Different Mod tiers and permissions.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum ModTier {
    /// Can verify posts and mark them for deletion.
    #[default]
    Verifier = 0,
    /// Can fully remove posts without additional supervision.
    Remover = 1,
    /// Has full permissions to do everything.
    Admin = 2,
    /// Same as admin, but also have access to unsafe features.
    Dev = 3,
}

impl From<u8> for ModTier {
    fn from(value: u8) -> Self {
        if value > ModTier::Dev as u8 {
            Self::default()
        } else {
            unsafe {
                std::mem::transmute(value)
            }
        }
    }
}
