use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::database::types::U8Visitor;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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

impl Serialize for PostState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for PostState {
    fn deserialize<D>(deserializer: D) -> Result<PostState, D::Error>
        where
            D: Deserializer<'de>,
    {
        let inner = deserializer.deserialize_u8(U8Visitor)?;
        Ok(PostState::from(inner))
    }
}
