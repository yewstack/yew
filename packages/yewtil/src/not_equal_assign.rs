//! Module for `neq_assign` utility function.

use std::borrow::BorrowMut;
use yew::html::ShouldRender;

/// Blanket trait to provide a convenience method for assigning props in `changed` or updating values in `update`.
pub trait NeqAssign<NEW> {
    /// If `self` and `new` aren't equal, assigns `new` to `self` and returns true, otherwise returns false.
    ///
    /// Short for "Not equal assign".
    ///
    /// # Example
    /// ```
    /// # use yew::{Component, ShouldRender, ComponentLink};
    /// # use yewtil::NeqAssign;
    /// # use yew::Properties;
    ///# use yew::virtual_dom::VNode;
    /// ##[derive(Clone, Properties, PartialEq)]
    ///  struct Props {
    ///     field1: String,
    ///     field2: usize
    ///  }
    ///  struct Model {
    ///     props: Props
    ///  }
    ///  impl Component for Model {
    /// #    type Message = ();
    ///     type Properties = Props;
    ///     // ...
    /// #
    /// #    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    /// #        unimplemented!()
    /// #    }
    /// #    fn update(&mut self, msg: ()) -> ShouldRender {
    /// #        unimplemented!()
    /// #    }
    /// #
    ///     fn change(&mut self, props: Self::Properties) -> ShouldRender{
    ///         self.props.neq_assign(props)
    ///     }
    ///#
    ///#     fn view(&self) -> VNode {
    ///#         unimplemented!()
    ///#     }
    ///  }
    ///
    /// let mut foo = 1;
    ///
    /// assert_eq!(foo.neq_assign(42), true);
    /// assert_eq!(foo, 42);
    ///
    /// assert_eq!(foo.neq_assign(42), false);
    /// ```
    fn neq_assign(&mut self, new: NEW) -> ShouldRender;
}

impl<T: BorrowMut<U>, U: PartialEq> NeqAssign<U> for T {
    fn neq_assign(&mut self, new: U) -> bool {
        self.neq_assign_by(new, |x, y| x == y)
    }
}
/// Blanket trait to provide a convenience method for assigning props in `changed` or updating values in `update`.
///
/// Like `neq_assign`, but for cases where `self` doesn't impl `PartialEq` or a nonstandard equality comparison is needed.
///
/// Useful for `Result<T, E: !PartialEq>`.
pub trait NeqAssignBy<NEW> {
    /// ```
    /// # use yewtil::{NeqAssign, NeqAssignBy};
    /// ##[derive(Clone, Debug)]
    /// struct NonComparableError;
    ///
    /// fn eq_by_ok<T, E>(a: &Result<T, E>, b: &Result<T, E>) -> bool
    /// where
    ///     T: PartialEq,
    /// {
    ///     match (a, b) {
    ///         (Ok(_), Err(_))
    ///         | (Err(_), Ok(_))
    ///         | (Err(_), Err(_)) => false,
    ///         (Ok(a), Ok(b)) => a == b,
    ///     }
    /// }
    ///
    /// let mut foo: Result<u32, NonComparableError> = Ok(1);
    ///
    /// // Won't compile
    /// // assert_eq!(foo.neq_assign(Ok(42)), true)
    ///
    /// assert_eq!(foo.neq_assign_by(Ok(42), eq_by_ok), true);
    /// assert_eq!(foo.clone().unwrap(), 42);
    ///
    /// assert_eq!(foo.neq_assign_by(Err(NonComparableError), eq_by_ok), true);
    /// assert!(foo.is_err());
    ///
    /// // The tradeoff: all assignments of an `Err` value will count as updates, even if they are
    /// // "the same" for all practical intents and purposes.
    /// assert_eq!(foo.neq_assign_by(Err(NonComparableError), eq_by_ok), true);
    /// assert_eq!(foo.neq_assign_by(Err(NonComparableError), eq_by_ok), true);
    /// assert_eq!(foo.neq_assign_by(Err(NonComparableError), eq_by_ok), true);
    /// ```
    ///
    fn neq_assign_by<F>(&mut self, new: NEW, eq: F) -> ShouldRender
    where
        F: FnOnce(&NEW, &NEW) -> bool;
}

impl<T: BorrowMut<U>, U> NeqAssignBy<U> for T {
    fn neq_assign_by<F>(&mut self, new: U, eq: F) -> ShouldRender
    where
        F: FnOnce(&U, &U) -> bool,
    {
        if !eq(self.borrow(), &new) {
            *self.borrow_mut() = new;
            true
        } else {
            false
        }
    }
}
