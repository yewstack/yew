use std::borrow::Cow;
use std::iter::FromIterator;
use std::rc::Rc;

use implicit_clone::ImplicitClone;
use indexmap::IndexSet;

use super::IntoPropValue;
use crate::virtual_dom::AttrValue;

/// A set of classes, cheap to clone.
///
/// The preferred way of creating this is using the [`classes!`][yew::classes!] macro.
#[derive(Debug, Clone, Default)]
pub struct Classes {
    set: Rc<IndexSet<AttrValue>>,
}

impl ImplicitClone for Classes {}

/// helper method to efficiently turn a set of classes into a space-separated
/// string. Abstracts differences between ToString and IntoPropValue. The
/// `rest` iterator is cloned to pre-compute the length of the String; it
/// should be cheap to clone.
fn build_attr_value(first: AttrValue, rest: impl Iterator<Item = AttrValue> + Clone) -> AttrValue {
    // The length of the string is known to be the length of all the
    // components, plus one space for each element in `rest`.
    let mut s = String::with_capacity(
        rest.clone()
            .map(|class| class.len())
            .chain([first.len(), rest.size_hint().0])
            .sum(),
    );

    s.push_str(first.as_str());
    // NOTE: this can be improved once Iterator::intersperse() becomes stable
    for class in rest {
        s.push(' ');
        s.push_str(class.as_str());
    }
    s.into()
}

impl Classes {
    /// Creates an empty set of classes. (Does not allocate.)
    #[inline]
    pub fn new() -> Self {
        Self {
            set: Rc::new(IndexSet::new()),
        }
    }

    /// Creates an empty set of classes with capacity for n elements. (Does not allocate if n is
    /// zero.)
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            set: Rc::new(IndexSet::with_capacity(n)),
        }
    }

    /// Adds a class to a set.
    ///
    /// If the provided class has already been added, this method will ignore it.
    pub fn push<T: Into<Self>>(&mut self, class: T) {
        let classes_to_add: Self = class.into();
        if self.is_empty() {
            *self = classes_to_add
        } else {
            Rc::make_mut(&mut self.set).extend(classes_to_add.set.iter().cloned())
        }
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
    /// you are absolutely certain that the string does not contain any whitespace and it is not
    /// empty. Using `push()`  is preferred.
    pub unsafe fn unchecked_push<T: Into<AttrValue>>(&mut self, class: T) {
        Rc::make_mut(&mut self.set).insert(class.into());
    }

    /// Check the set contains a class.
    #[inline]
    pub fn contains<T: AsRef<str>>(&self, class: T) -> bool {
        self.set.contains(class.as_ref())
    }

    /// Check the set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}

impl IntoPropValue<AttrValue> for Classes {
    #[inline]
    fn into_prop_value(self) -> AttrValue {
        let mut classes = self.set.iter().cloned();

        match classes.next() {
            None => AttrValue::Static(""),
            Some(class) if classes.len() == 0 => class,
            Some(first) => build_attr_value(first, classes),
        }
    }
}

impl IntoPropValue<Option<AttrValue>> for Classes {
    #[inline]
    fn into_prop_value(self) -> Option<AttrValue> {
        if self.is_empty() {
            None
        } else {
            Some(self.into_prop_value())
        }
    }
}

impl IntoPropValue<Classes> for &'static str {
    #[inline]
    fn into_prop_value(self) -> Classes {
        self.into()
    }
}

impl<T: Into<Classes>> Extend<T> for Classes {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|classes| self.push(classes))
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
    type IntoIter = indexmap::set::IntoIter<AttrValue>;
    type Item = AttrValue;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // NOTE: replace this by Rc::unwrap_or_clone() when it becomes stable
        Rc::try_unwrap(self.set)
            .unwrap_or_else(|rc| (*rc).clone())
            .into_iter()
    }
}

impl IntoIterator for &Classes {
    type IntoIter = indexmap::set::IntoIter<AttrValue>;
    type Item = AttrValue;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (*self.set).clone().into_iter()
    }
}

impl ToString for Classes {
    fn to_string(&self) -> String {
        let mut iter = self.set.iter().cloned();

        iter.next()
            .map(|first| build_attr_value(first, iter))
            .unwrap_or_default()
            .to_string()
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
        let set = t.split_whitespace().map(AttrValue::Static).collect();
        Self { set: Rc::new(set) }
    }
}

impl From<String> for Classes {
    fn from(t: String) -> Self {
        match t.contains(|c: char| c.is_whitespace()) {
            // If the string only contains a single class, we can just use it
            // directly (rather than cloning it into a new string). Need to make
            // sure it's not empty, though.
            false => match t.is_empty() {
                true => Self::new(),
                false => Self {
                    set: Rc::new(IndexSet::from_iter([AttrValue::from(t)])),
                },
            },
            true => Self::from(&t),
        }
    }
}

impl From<&String> for Classes {
    fn from(t: &String) -> Self {
        let set = t
            .split_whitespace()
            .map(ToOwned::to_owned)
            .map(AttrValue::from)
            .collect();
        Self { set: Rc::new(set) }
    }
}

impl From<&AttrValue> for Classes {
    fn from(t: &AttrValue) -> Self {
        let set = t
            .split_whitespace()
            .map(ToOwned::to_owned)
            .map(AttrValue::from)
            .collect();
        Self { set: Rc::new(set) }
    }
}

impl From<AttrValue> for Classes {
    fn from(t: AttrValue) -> Self {
        match t.contains(|c: char| c.is_whitespace()) {
            // If the string only contains a single class, we can just use it
            // directly (rather than cloning it into a new string). Need to make
            // sure it's not empty, though.
            false => match t.is_empty() {
                true => Self::new(),
                false => Self {
                    set: Rc::new(IndexSet::from_iter([t])),
                },
            },
            true => Self::from(&t),
        }
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
        t.iter().cloned().collect()
    }
}

impl PartialEq for Classes {
    fn eq(&self, other: &Self) -> bool {
        self.set.len() == other.set.len() && self.set.iter().eq(other.set.iter())
    }
}

impl Eq for Classes {}

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
        subject.extend(&other);
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

    #[test]
    fn ignores_empty_string() {
        let classes = String::from("");
        let subject = Classes::from(classes);
        assert!(subject.is_empty())
    }
}
