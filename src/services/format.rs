use serde::{Serialize, Deserialize};
use serde_json;

pub type Storable = Option<String>;

pub type Restorable = Result<String, String>;

pub struct Nothing;

impl Into<Storable> for Nothing {
    fn into(self) -> Storable {
        None
    }
}

impl From<Restorable> for Nothing {
    fn from(_: Restorable) -> Nothing {
        Nothing
    }
}

pub struct Json<T>(pub T);

impl<'a, T> Into<Storable> for Json<&'a T>
where
    T: Serialize
{
    fn into(self) -> Storable {
        serde_json::to_string(&self.0).ok()
    }
}

impl<T> From<Restorable> for Json<Result<T, ()>>
where
    T: for <'de> Deserialize<'de>
{
    fn from(value: Restorable) -> Self {
        match value {
            Ok(data) => {
                Json(serde_json::from_str(&data).map_err(drop))
            }
            Err(_reason) => {
                Json(Err(()))
            }
        }
    }
}

