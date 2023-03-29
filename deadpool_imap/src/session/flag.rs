use async_imap::types::Flag as InternalFlag;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Flag {
    #[serde(rename(serialize = "\\Seen"))]
    Seen,
    #[serde(rename(serialize = "\\Answered"))]
    Answered,
    #[serde(rename(serialize = "\\Flagged"))]
    Flagged,
    #[serde(rename(serialize = "\\Deleted"))]
    Deleted,
    #[serde(rename(serialize = "\\Draft"))]
    Draft,
    #[serde(rename(serialize = "\\Recent"))]
    Recent,
    #[serde(rename(serialize = "\\*"))]
    MayCreate,
    #[serde(rename(serialize = "{0}"))]
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
            Flag::Custom(custom) => custom.into(),
        }
    }
}
