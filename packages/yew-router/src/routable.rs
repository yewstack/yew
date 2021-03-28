use std::collections::HashMap;

pub trait Routable: std::fmt::Debug {
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self> where Self: Sized;

    fn to_route(&self) -> String;

    // from https://stackoverflow.com/a/33687996
    fn as_any(&self) -> &(dyn std::any::Any + 'static);
}
