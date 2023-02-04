use serde::Serialize;

pub mod commands;
pub mod environments;
pub mod tokens;

pub trait Tag {
    fn value(&self) -> &str;
}

impl Serialize for dyn Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.value())
    }
}