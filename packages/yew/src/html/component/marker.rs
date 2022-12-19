//! Primitive Components & Properties Types

use crate::function_component;
use crate::html::{BaseComponent, ChildrenProps, Html};

/// A Component to represent a component that does not exist in current implementation.
///
/// During Hydration, Yew expected the Virtual DOM hierarchy to match the the layout used in
/// server-side rendering. However, sometimes it is possible / reasonable to omit certain components
/// from one side of the implementation. This component is used to represent a component as if a
/// component "existed" in the place it is defined.
///
/// # Warning
///
/// The Real DOM hierarchy must also match the server-side rendered artifact. This component is
/// only usable when the original component does not introduce any additional elements. (e.g.:
/// Context Providers)
///
/// A generic parameter is provided to help identify the component to be substituted.
/// The type of the generic parameter is not required to be the same component that was in the other
/// implementation. However, this behaviour may change in the future if more debug assertions were
/// to be introduced. It is recommended that the generic parameter represents the component in the
/// other implementation.
///
/// # Example
///
/// ```
/// use yew::prelude::*;
/// # use yew::html::ChildrenProps;
/// #
/// # #[function_component]
/// # fn Comp(props: &ChildrenProps) -> Html {
/// #     Html::default()
/// # }
/// #
/// # #[function_component]
/// # fn Provider(props: &ChildrenProps) -> Html {
/// #     let children = props.children.clone();
/// #
/// #     html! { <>{children}</> }
/// # }
/// # type Provider1 = Provider;
/// # type Provider2 = Provider;
/// # type Provider3 = Provider;
/// # type Provider4 = Provider;
///
/// #[function_component]
/// fn ServerApp() -> Html {
///     // The Server Side Rendering Application has 3 Providers.
///     html! {
///         <Provider1>
///             <Provider2>
///                 <Provider3>
///                     <Comp />
///                 </Provider3>
///             </Provider2>
///         </Provider1>
///     }
/// }
///
/// #[function_component]
/// fn App() -> Html {
///     // The Client Side Rendering Application has 4 Providers.
///     html! {
///         <Provider1>
///             <Provider2>
///                 <Provider3>
///
///                     // This provider does not exist on the server-side
///                     // Hydration will fail due to Virtual DOM layout mismatch.
///                     <Provider4>
///                         <Comp />
///                     </Provider4>
///
///                 </Provider3>
///             </Provider2>
///         </Provider1>
///     }
/// }
/// ```
///
/// To mitigate this, we can use a `PhantomComponent`:
///
/// ```
/// use yew::prelude::*;
/// # use yew::html::{PhantomComponent, ChildrenProps};
/// #
/// # #[function_component]
/// # fn Comp(props: &ChildrenProps) -> Html {
/// #     Html::default()
/// # }
/// #
/// # #[function_component]
/// # fn Provider(props: &ChildrenProps) -> Html {
/// #     let children = props.children.clone();
/// #
/// #     html! { <>{children}</> }
/// # }
/// # type Provider1 = Provider;
/// # type Provider2 = Provider;
/// # type Provider3 = Provider;
/// # type Provider4 = Provider;
///
/// #[function_component]
/// fn ServerApp() -> Html {
///     html! {
///         <Provider1>
///             <Provider2>
///                 <Provider3>
///                     // We add a PhantomComponent for Provider4,
///                     // it acts if a Provider4 component presents in this position.
///                     <PhantomComponent<Provider4>>
///                         <Comp />
///                     </PhantomComponent<Provider4>>
///                 </Provider3>
///             </Provider2>
///         </Provider1>
///     }
/// }
///
/// #[function_component]
/// fn App() -> Html {
///     html! {
///         <Provider1>
///             <Provider2>
///                 <Provider3>
///
///                     // Hydration will succeed as the PhantomComponent in the server-side
///                     // implementation will represent a Provider4 component in this position.
///                     <Provider4>
///                         <Comp />
///                     </Provider4>
///
///                 </Provider3>
///             </Provider2>
///         </Provider1>
///     }
/// }
/// ```
#[function_component]
pub fn PhantomComponent<T>(props: &ChildrenProps) -> Html
where
    T: BaseComponent,
{
    props.children.clone()
}
