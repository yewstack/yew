use indexmap::IndexSet;
use std::{
    borrow::{Borrow, Cow},
    iter::FromIterator,
};

/// A set of classes.
///
/// The preferred way of creating this is using the [`classes!`][yew::classes!] macro.
#[derive(Debug, Clone, Default)]
pub struct Classes {
    set: IndexSet<Cow<'static, str>>,
}

impl Classes {
    /// Creates an empty set of classes. (Does not allocate.)
    pub fn new() -> Self {
        Self {
            set: IndexSet::new(),
        }
    }

    /// Creates an empty set of classes with capacity for n elements. (Does not allocate if n is
    /// zero.)
    pub fn with_capacity(n: usize) -> Self {
        Self {
            set: IndexSet::with_capacity(n),
        }
    }

    /// Adds a class to a set.
    ///
    /// If the provided class has already been added, this method will ignore it.
    pub fn push<T: Into<Self>>(&mut self, class: T) {
        let classes_to_add: Self = class.into();
        self.set.extend(classes_to_add.set);
    }

    /// Adds a class to a set.
    ///
    /// If the provided class has already been added, this method will ignore it.
    ///
    /// This method won't check if there are multiple classes in the input string.
    ///
    /// # Safety
    ///
    /// This function will not split the string into multiple classes. Please do not use it unless
    /// you are absolutely certain that the string does not contain any whitespace. Using `push()`
    /// is preferred.
    pub unsafe fn unchecked_push<T: Into<Cow<'static, str>>>(&mut self, class: T) {
        self.set.insert(class.into());
    }

    /// Check the set contains a class.
    pub fn contains<T: AsRef<str>>(&self, class: T) -> bool {
        self.set.contains(class.as_ref())
    }

    /// Check the set is empty.
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl<T: Into<Classes>> Extend<T> for Classes {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let classes = iter
            .into_iter()
            .map(Into::into)
            .flat_map(|classes| classes.set);
        self.set.extend(classes);
    }
}

impl<T: Into<Classes>> FromIterator<T> for Classes {
    fn from_iter<IT: IntoIterator<Item = T>>(iter: IT) -> Self {
        let mut classes = Self::new();
        classes.extend(iter);
        classes
    }
}

impl IntoIterator for Classes {
    type Item = Cow<'static, str>;
    type IntoIter = indexmap::set::IntoIter<Cow<'static, str>>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl ToString for Classes {
    fn to_string(&self) -> String {
        self.set
            .iter()
            .map(Borrow::borrow)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl From<Cow<'static, str>> for Classes {
    fn from(t: Cow<'static, str>) -> Self {
        match t {
            Cow::Borrowed(x) => Self::from(x),
            Cow::Owned(x) => Self::from(x),
        }
    }
}

impl From<&'static str> for Classes {
    fn from(t: &'static str) -> Self {
        let set = t.split_whitespace().map(Cow::Borrowed).collect();
        Self { set }
    }
}

impl From<String> for Classes {
    fn from(t: String) -> Self {
        Self::from(&t)
    }
}

impl From<&String> for Classes {
    fn from(t: &String) -> Self {
        let set = t
            .split_whitespace()
            .map(ToOwned::to_owned)
            .map(Cow::Owned)
            .collect();
        Self { set }
    }
}

impl<T: Into<Classes>> From<Option<T>> for Classes {
    fn from(t: Option<T>) -> Self {
        t.map(|x| x.into()).unwrap_or_default()
    }
}

impl<T: Into<Classes> + Clone> From<&Option<T>> for Classes {
    fn from(t: &Option<T>) -> Self {
        Self::from(t.clone())
    }
}

impl<T: Into<Classes>> From<Vec<T>> for Classes {
    fn from(t: Vec<T>) -> Self {
        Self::from_iter(t)
    }
}

impl<T: Into<Classes> + Clone> From<&[T]> for Classes {
    fn from(t: &[T]) -> Self {
        Self::from_iter(t.iter().cloned())
    }
}

impl PartialEq for Classes {
    fn eq(&self, other: &Self) -> bool {
        self.set.len() == other.set.len() && self.set.iter().eq(other.set.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestClass;

    impl TestClass {
        fn as_class(&self) -> &'static str {
            "test-class"
        }
    }

    impl From<TestClass> for Classes {
        fn from(x: TestClass) -> Self {
            Classes::from(x.as_class())
        }
    }

    #[test]
    fn it_is_initially_empty() {
        let subject = Classes::new();
        assert!(subject.is_empty());
    }

    #[test]
    fn it_pushes_value() {
        let mut subject = Classes::new();
        subject.push("foo");
        assert!(!subject.is_empty());
        assert!(subject.contains("foo"));
    }

    #[test]
    fn it_adds_values_via_extend() {
        let mut other = Classes::new();
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_contains_both_values() {
        let mut other = Classes::new();
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        subject.push("foo");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn it_splits_class_with_spaces() {
        let mut subject = Classes::new();
        subject.push("foo bar");
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn push_and_contains_can_be_used_with_other_objects() {
        let mut subject = Classes::new();
        subject.push(TestClass);
        let other_class: Option<TestClass> = None;
        subject.push(other_class);
        assert!(subject.contains(TestClass.as_class()));
    }

    #[test]
    fn can_be_extended_with_another_class() {
        let mut other = Classes::new();
        other.push("foo");
        other.push("bar");
        let mut subject = Classes::new();
        subject.extend(other);
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }

    #[test]
    fn can_be_collected() {
        let classes = vec!["foo", "bar"];
        let subject = classes.into_iter().collect::<Classes>();
        assert!(subject.contains("foo"));
        assert!(subject.contains("bar"));
    }
}
