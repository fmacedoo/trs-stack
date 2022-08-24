use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
pub struct Todo {
    pub id: String,
    pub description: String,
}

impl Serialize for Todo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut result = serializer.serialize_struct("Todo", 2)?;
        result.serialize_field("id", &self.id)?;
        result.serialize_field("description", &self.description)?;
        result.end()
    }
}
