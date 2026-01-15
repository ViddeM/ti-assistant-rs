use serde::{Deserialize, Serialize, de::Error};
use std::{
    fmt::{Debug, Display},
    io::Write,
    ops::Deref,
    str::{self, FromStr},
};

/// A game ID, which is always an 8 character hexadecimal string.
///
/// You can create a [GameId] by calling `.parse` on a string.
///
/// # Invariant
///
/// The inner `[u8; 8]` must always be a valid 8-character hexadecimal string. This is enforced
/// through the `From<u32>`, and the [FromStr] impls, which are the only valid ways to make GameIds.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameId([u8; 8]);

impl Deref for GameId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.0).expect("GameId must always be a hexadecimal str")
    }
}

#[cfg(feature = "server")]
use rand::random;

#[cfg(feature = "server")]
impl GameId {
    /// Generate a new randmom game ID.
    pub fn random() -> Self {
        let id: u32 = random();
        id.into()
    }
}

impl FromStr for GameId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: &[u8; 8] = s
            .as_bytes()
            .try_into()
            .map_err(|_| "GameId must be exactly 8 bytes long")?;

        if s.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err("GameId must be a hexadecimal string");
        }

        let _parsed = u32::from_str_radix(s, 16)
            .map_err(|_| "failed to parse game id, expected hex string of length 8")?;

        Ok(GameId(*id))
    }
}

impl From<u32> for GameId {
    fn from(id: u32) -> Self {
        let mut buf = [0u8; 8];
        write!(&mut &mut buf[..], "{id:08x}").expect("the buf is big enough");
        GameId(buf)
    }
}

impl Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Debug for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.deref(), f)
    }
}
