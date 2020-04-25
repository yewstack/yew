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
    ///#
    ///#     fn view(&self) -> VNode {
    ///#         unimplemented!()
    ///#     }
    ///  }
    /// ```
    fn neq_assign(&mut self, new: NEW) -> ShouldRender;
}

impl<T: BorrowMut<U>, U: PartialEq> NeqAssign<U> for T {
    fn neq_assign(&mut self, new: U) -> bool {
        if self.borrow() != &new {
            *self.borrow_mut() = new;
            true
        } else {
            false
        }
    }
}
