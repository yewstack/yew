/// Transform properties to the expected type.
pub trait Transformer<FROM, TO> {
    /// Transforms one type to another.
    fn transform(from: FROM) -> TO;
}

mod vcomp {
    use crate::virtual_dom::VComp;

    use super::*;
    impl<T> Transformer<T, T> for VComp {
        fn transform(from: T) -> T {
            from
        }
    }

    impl<'a, T> Transformer<&'a T, T> for VComp
    where
        T: Clone,
    {
        fn transform(from: &'a T) -> T {
            from.clone()
        }
    }

    impl<'a> Transformer<&'a str, String> for VComp {
        fn transform(from: &'a str) -> String {
            from.to_owned()
        }
    }

    impl<T> Transformer<T, Option<T>> for VComp {
        fn transform(from: T) -> Option<T> {
            Some(from)
        }
    }

    impl<'a, T> Transformer<&'a T, Option<T>> for VComp
    where
        T: Clone,
    {
        fn transform(from: &T) -> Option<T> {
            Some(from.clone())
        }
    }

    impl<'a> Transformer<&'a str, Option<String>> for VComp {
        fn transform(from: &'a str) -> Option<String> {
            Some(from.to_owned())
        }
    }

    impl<'a> Transformer<Option<&'a str>, Option<String>> for VComp {
        fn transform(from: Option<&'a str>) -> Option<String> {
            from.map(|s| s.to_owned())
        }
    }
}

mod vtag {
    use super::*;
    use crate::{backend::DomBackend, virtual_dom::VTag};

    impl<T, REND: DomBackend> Transformer<T, T> for VTag<REND> {
        fn transform(from: T) -> T {
            from
        }
    }

    impl<'a, T, REND: DomBackend> Transformer<&'a T, T> for VTag<REND>
    where
        T: Clone,
    {
        fn transform(from: &'a T) -> T {
            from.clone()
        }
    }
}
