//! Module for `neq_assign` utility function.

use crate::html::ShouldRender;

/// Blanket trait to provide a convenience method for assigning props in `changed` or updating values in `update`.
pub trait NeqAssign {
    /// If `self` and `new` aren't equal, assigns `new` to `self` and returns true, otherwise returns false.
    ///
    /// Short for "Not equal assign".
    ///
    /// # Example
    /// ```
    /// # use yew::{Component, ShouldRender, ComponentLink};
    /// # use yew::neq_assign::NeqAssign;
    /// # use yew::Properties;
    /// ##[derive(Properties, PartialEq)]
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
    ///  }
    /// ```
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}
