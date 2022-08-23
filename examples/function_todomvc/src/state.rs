use std::rc::Rc;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub entries: Vec<Entry>,
    pub filter: Filter,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
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

pub enum Action {
    Add(String),
    Edit((usize, String)),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    Toggle(usize),
    ClearCompleted,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(description) => {
                let mut entries = self.entries.clone();
                entries.push(Entry {
                    id: entries.last().map(|entry| entry.id + 1).unwrap_or(1),
                    description,
                    completed: false,
                });
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::Remove(id) => {
                let mut entries = self.entries.clone();
                entries.retain(|entry| entry.id != id);
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::Toggle(id) => {
                let mut entries = self.entries.clone();
                let entry = entries.iter_mut().find(|entry| entry.id == id);
                if let Some(entry) = entry {
                    entry.completed = !entry.completed;
                }
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::Edit((id, description)) => {
                let mut entries = self.entries.clone();

                if description.is_empty() {
                    entries.retain(|entry| entry.id != id)
                }

                let entry = entries.iter_mut().find(|entry| entry.id == id);
                if let Some(entry) = entry {
                    entry.description = description;
                }
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::ToggleAll => {
                let mut entries = self.entries.clone();
                for entry in &mut entries {
                    if self.filter.fits(entry) {
                        entry.completed = !entry.completed;
                    }
                }
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::ClearCompleted => {
                let mut entries = self.entries.clone();
                entries.retain(|e| Filter::Active.fits(e));
                State {
                    entries,
                    filter: self.filter,
                }
                .into()
            }
            Action::SetFilter(filter) => State {
                filter,
                entries: self.entries.clone(),
            }
            .into(),
        }
    }
}
