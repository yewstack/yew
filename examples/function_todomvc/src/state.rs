use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, ToString};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub entries: Vec<Entry>,
    pub filter: Filter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Entry {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, EnumIter, ToString, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn fits(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }

    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        }
    }
}
