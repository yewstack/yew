use std::collections::HashMap;

/// Parameters from a query parameters.
#[derive(Debug, Clone)]
pub struct Params {
    data: HashMap<String, String>,
}

impl Params {
    /// Get a parameter by its key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|it| it.as_str())
    }
}

impl From<HashMap<String, String>> for Params {
    fn from(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}

impl IntoIterator for Params {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
