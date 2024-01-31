#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum PostState {
    /// Awaiting approval from one of the mods.
    #[default]
    Awaiting = 0,
    /// Has been approved and is now public.
    Approved = 1,
    /// Has been rejected and the contents cannot be viewed.
    Rejected = 2,
    /// Marked for deletion.
    ForDeletion = 3,
}

impl From<u8> for PostState {
    fn from(value: u8) -> Self {
        if value > PostState::ForDeletion as u8 {
            Self::default()
        } else {
            unsafe {
                std::mem::transmute(value)
            }
        }
    }
}
