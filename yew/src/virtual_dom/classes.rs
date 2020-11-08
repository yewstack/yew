use indexmap::IndexSet;
use std::{
    borrow::{Borrow, Cow},
    iter::FromIterator,
};

/// A set of classes.
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
    pub fn unchecked_push<T: Into<Cow<'static, str>>>(&mut self, class: T) {
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
}

#[cfg(test)]
mod macro_tests {
    extern crate self as yew;

    use crate::virtual_dom::{AnyScope, VDiff, VNode, VTag};
    use crate::{classes, html, NodeRef};
    use std::any::TypeId;
    use std::rc::Rc;
    #[cfg(feature = "std_web")]
    use stdweb::web::{document, IElement};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};

    #[cfg(feature = "wasm_test")]
    wasm_bindgen_test_configure!(run_in_browser);

    fn test_scope() -> AnyScope {
        AnyScope {
            type_id: TypeId::of::<()>(),
            parent: None,
            state: Rc::new(()),
        }
    }

    #[test]
    fn classes_from_local_variables() {
        let a = html! {
            <div class=classes!("class-1", "class-2")></div>
        };

        let class_2 = "class-2";
        let b = html! {
            <div class=classes!("class-1", class_2)></div>
        };

        let class_2_fmt = format!("class-{}", 2);
        let c = html! {
            <div class=classes!("class-1", class_2_fmt)></div>
        };

        assert_eq!(a, b);
        assert_eq!(a, c);
    }

    /// Returns the class attribute as str reference, or "" if the attribute is not set.
    fn get_class_str(vtag: &VTag) -> &str {
        vtag.attributes
            .iter()
            .find(|(k, _)| k == &"class")
            .map(|(_, v)| AsRef::as_ref(v))
            .unwrap_or("")
    }

    /// Note: Compares to "" if the class attribute is not set.
    fn assert_class(vnode: VNode, class: &str) {
        if let VNode::VTag(ref vtag) = vnode {
            assert_eq!(get_class_str(vtag), class);
        } else {
            panic!("expected VTag");
        }
    }

    #[test]
    fn supports_multiple_non_unique_classes_tuple() {
        let a = html! {
            <div class=classes!("class-1", "class-1 class-2")></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_string() {
        let a = html! {
            <div class=classes!("class-1 class-2 class-3")></div>
        };
        let b = html! {
            <div class=classes!("class-2 class-3 class-1")></div>
        };

        assert_ne!(a, b);

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_slice() {
        let classes = classes!(&["class-1", "class-2"][..]);
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_one_value_slice() {
        let classes = classes!(&["class-1 class-2", "class-1"][..]);
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_vec() {
        let mut classes = vec!["class-1"];
        classes.push("class-2");
        let classes = classes!(classes);
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn supports_multiple_classes_one_value_vec() {
        let classes = vec!["class-1 class-2", "class-1"];
        let classes = classes!(classes);
        let a = html! {
            <div class=classes></div>
        };

        if let VNode::VTag(vtag) = a {
            assert!(get_class_str(&vtag).contains("class-1"));
            assert!(get_class_str(&vtag).contains("class-2"));
            assert!(!get_class_str(&vtag).contains("class-3"));
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn filter_empty_string_classes() {
        let a = html! { <div class=classes!(vec![""])></div> };
        let b = html! { <div class=classes!("", "")></div> };
        let c = html! { <div class=""></div> };
        let d_arr = [""];
        let d = html! { <div class=classes!(&d_arr[..])></div> };

        macro_rules! get_class {
            ($vtag:expr) => {
                $vtag
                    .attributes
                    .iter()
                    .find_map(|(k, v)| if k == "class" { Some(v) } else { None })
            };
        }

        if let VNode::VTag(vtag) = a {
            assert_eq!(get_class!(vtag), None);
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = b {
            assert_eq!(get_class!(vtag), None);
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = c {
            assert_eq!(get_class!(vtag), None);
        } else {
            panic!("vtag expected");
        }

        if let VNode::VTag(vtag) = d {
            assert_eq!(get_class!(vtag), None);
        } else {
            panic!("vtag expected");
        }
    }

    #[test]
    fn keeps_order_of_classes() {
        let a = html! {
            <div class=classes!(vec!["class-1", "class-2", "class-3"])></div>
        };

        if let VNode::VTag(vtag) = a {
            assert_eq!(get_class_str(&vtag), "class-1 class-2 class-3");
        }
    }

    #[test]
    fn tuple_different_types() {
        // check if tuples containing different types are compiling
        assert_class(
            html! { <div class=classes!("class-1", "class-2".to_string(), vec!["class-3", "class-4"])></div> },
            "class-1 class-2 class-3 class-4",
        );
        assert_class(
            html! { <div class=classes!("class-1", Some("class-2"), "class-3", Some("class-4".to_string()))></div> },
            "class-1 class-2 class-3 class-4",
        );
        // check different string references
        let str = "some-class";
        let string = str.to_string();
        let string_ref = &string;
        assert_class(html! { <p class=classes!(str) /> }, "some-class");
        assert_class(html! { <p class=classes!(string.clone()) /> }, "some-class");
        assert_class(html! { <p class=classes!(&Some(str)) /> }, "some-class");
        assert_class(html! { <p class=classes!(string_ref) /> }, "some-class");
        assert_class(html! { <p class=classes!(Some(str)) /> }, "some-class");
        assert_class(
            html! { <p class=classes!(Some(string.clone())) /> },
            "some-class",
        );
        assert_class(
            html! { <p class=classes!(Some(string_ref)) /> },
            "some-class",
        );
        assert_class(
            html! { <p class=classes!(&Some(string.clone())) /> },
            "some-class",
        );
        assert_class(
            html! { <p class=classes!(&Some(string_ref)) /> },
            "some-class",
        );
        // check with None
        assert_class(html! { <p class=classes!(&Option::<&str>::None) /> }, "");
        assert_class(html! { <p class=classes!(Option::<String>::None) /> }, "");
        // check with variables
        let some: Option<&'static str> = Some("some");
        let none: Option<&'static str> = None;
        assert_class(html! { <p class=classes!(some) /> }, "some");
        assert_class(html! { <p class=classes!(none) /> }, "");
        // check with variables of different type
        let some: Option<bool> = Some(false);
        let none: Option<bool> = None;
        assert_class(
            html! { <p class=classes!(some.map(|i| i.to_string())) /> },
            "false",
        );
        assert_class(
            html! { <p class=classes!(none.map(|i| i.to_string())) /> },
            "",
        );
    }

    #[test]
    #[cfg(any(feature = "std_web", feature = "web_sys"))]
    fn swap_order_of_classes() {
        #[cfg(feature = "std_web")]
        let document = document();
        #[cfg(feature = "web_sys")]
        let document = web_sys::window().unwrap().document().unwrap();

        let scope = test_scope();
        let parent = document.create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document.body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document.body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class=classes!("class-1", "class-2", "class-3")></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);

        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        let expected = "class-1 class-2 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );

        let ancestor = vtag;
        let elem = html! { <div class=classes!("class-3", "class-2", "class-1")></div> };
        let mut vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        let expected = "class-3 class-2 class-1";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );
    }

    #[test]
    #[cfg(any(feature = "std_web", feature = "web_sys"))]
    fn add_class_to_the_middle() {
        #[cfg(feature = "std_web")]
        let document = document();
        #[cfg(feature = "web_sys")]
        let document = web_sys::window().unwrap().document().unwrap();

        let scope = test_scope();
        let parent = document.create_element("div").unwrap();

        #[cfg(feature = "std_web")]
        document.body().unwrap().append_child(&parent);
        #[cfg(feature = "web_sys")]
        document.body().unwrap().append_child(&parent).unwrap();

        let mut elem = html! { <div class=classes!("class-1", "class-3")></div> };
        elem.apply(&scope, &parent, NodeRef::default(), None);

        let vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };

        let expected = "class-1 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );

        let ancestor = vtag;
        let elem = html! { <div class=classes!("class-1", "class-2", "class-3")></div> };
        let mut vtag = if let VNode::VTag(vtag) = elem {
            vtag
        } else {
            panic!("should be vtag")
        };
        vtag.apply(
            &scope,
            &parent,
            NodeRef::default(),
            Some(VNode::VTag(ancestor)),
        );

        let expected = "class-1 class-2 class-3";
        assert_eq!(get_class_str(&vtag), expected);
        assert_eq!(
            vtag.reference
                .as_ref()
                .unwrap()
                .get_attribute("class")
                .unwrap(),
            expected
        );
    }
}
