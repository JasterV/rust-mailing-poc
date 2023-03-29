use async_imap::types::Flag as InternalFlag;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Deserialize)]
pub enum Flag {
    Seen,
    Answered,
    Flagged,
    Deleted,
    Draft,
    Recent,
    #[serde(rename(deserialize = "*"))]
    MayCreate,
    #[serde(rename(deserialize = "custom"))]
    Custom(String),
}

impl From<Flag> for InternalFlag<'_> {
    fn from(value: Flag) -> Self {
        match value {
            Flag::Seen => InternalFlag::Seen,
            Flag::Deleted => InternalFlag::Deleted,
            Flag::Draft => InternalFlag::Draft,
            Flag::Answered => InternalFlag::Answered,
            Flag::Flagged => InternalFlag::Flagged,
            Flag::Recent => InternalFlag::Recent,
            Flag::MayCreate => InternalFlag::MayCreate,
            Flag::Custom(custom) => InternalFlag::Custom(custom.into()),
        }
    }
}

impl From<InternalFlag<'_>> for Flag {
    fn from(value: InternalFlag<'_>) -> Self {
        match value {
            InternalFlag::Seen => Flag::Seen,
            InternalFlag::Deleted => Flag::Deleted,
            InternalFlag::Draft => Flag::Draft,
            InternalFlag::Answered => Flag::Answered,
            InternalFlag::Flagged => Flag::Flagged,
            InternalFlag::Recent => Flag::Recent,
            InternalFlag::MayCreate => Flag::MayCreate,
            InternalFlag::Custom(custom) => Flag::Custom(custom.into()),
        }
    }
}

impl From<Flag> for String {
    fn from(flag: Flag) -> Self {
        match flag {
            Flag::Seen => "\\Seen".into(),
            Flag::Answered => "\\Answered".into(),
            Flag::Flagged => "\\Flagged".into(),
            Flag::Deleted => "\\Deleted".into(),
            Flag::Draft => "\\Draft".into(),
            Flag::Recent => "\\Recent".into(),
            Flag::MayCreate => "\\*".into(),
            Flag::Custom(custom) => custom,
        }
    }
}

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = String::from(self.clone());
        serializer.serialize_str(&serialized)
    }
}
