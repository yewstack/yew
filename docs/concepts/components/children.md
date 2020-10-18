---
title: Children
description: 
---

## General usage

_Most of the time,_ when allowing a component to have children, you don't care 
what type of children the component has. In such cases, the below example will
suffice.

```rust
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ListProps {
	#[prop_or_default]
	pub children: Children,
}

pub struct List {
	props: ListProps,
}

impl Component for List {
	type Properties = ListProps;
	// ...

	fn veiw(&self) -> Html {
		html! {
			<div class="list">
				{ for self.props.children.iter() }
			</div>
		}
	}
}
```

## Advanced usage

Here are two cases where you would want to break away from using `Children`
and instead use something else:

 - You want to restrict the types of components that can be used as children
   on this component.

 - You want to share state between a component and its children.

### Typed children

In cases where you only need to restrict the children to a single component,
you can use `yew::html::ChildrenWithProps<T>`.

```rust
use yew::prelude::*;
use yew::html::ChildrenWithProps;

// ...

#[derive(Properties, Clone)]
pub struct ListProps {
	#[prop_or_default]
	pub children: ChildrenWithProps<Item>,
}

pub struct List {
	props: ListProps,
}

impl Component for ListProps {
	type Properties = ListProps;
	// ...

	fn view(&self) -> Html {
		html! {
			<div class="list">
				{ for self.props.children.iter() }
			</div>
		}
	}
}
```

Of course, sometimes you might need to restrict the children to a few different
components. In these cases, you have to get a little more hands-on with yew.

<!--
I'm not sure where else to put this note, so feel free to make suggestions
in the review.
-->
:::note
The `derive_more` crate is used here for better ergonomics. If you don't want
to use it, replace `ItemPropVariants` with this:
```rust
#[derive(Clone)]
pub enum ItemPropVariants {
	MyFirstComponent(MyFirstComponentProps),
	MySecondComponent(MySecondComponentProps),
}

impl From<MyFirstComponentProps> for ItemPropVariants {
	fn from(props: MyFirstComponentProps) -> Self {
		Self::MyFirstComponent(props)		
	}
}

impl From<MySecondComponentProps> for ItemPropVariants {
	fn from(props: MySecondComponentProps) -> Self {
		Self::MySecondComponent(props)
	}
}
```
:::

```rust
use yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::virtual_dom::{ VChild, VComp };

// ...

#[derive(Clone, derive_more::From)]
pub enum ItemPropVariants {
	MyFirstComponent(MyFirstComponentProps),
	MySecondComponent(MySecondComponentProps),
}

#[derive(Clone)]
pub struct Item {
	props: ItemPropVariants,
}

impl<CHILD> From<VChild<CHILD>> for Item
where
	CHILD: Component,
	CHILD::Properties: Into<ItemPropVariants>,
{
	fn from(vchild: VChild<CHILD>) -> Self {
		Item {
			props: vchild.props.into(),
		}
	}
}

impl Into<Html> for Item {
	fn into(self) -> Html {
		match self.props {
			ItemPropVariants::MyFirstComponent(props) => {
				VComp::new::<MyFirstComponent>(props, NodeRef::default(), None).into()
			}
			ItemPropVariants::MySecondComponent(props) => {
				VComp::new::<MySecondComponent>(props, NodeRef::default(), None).into()
			}
		}
	}
}

#[derive(Properties, Clone)]
pub struct ListProps {
	#[prop_or_default]
	pub children: ChildrenRenderer<Item>,
}

pub struct List {
	props: ListProps,
}

impl Component for List {
	type Properties = ListProps;
	// ...

	fn view(&self) -> Html {
		html! {
			<div class="list">
				{ for self.props.children.iter() }
			</div>
		}
	}
}
```

### Sharing properties

This use case is mostly helpful for library developers, as you may want to
share state between parent and child components without causing difficulty for
users of your library.

```rust
// ... See the multiple typed children example ...

#[derive(Properties, Clone)]
pub struct ListProps {
	#[prop_or_default]
	pub children: ChildrenRenderer<Item>,
}

pub struct List {
	props: ListProps,
}

impl Component for List {
	type Properties = ListProps;
	// ...

	fn view(&self) -> Html {
		html! {
			<div class="list">
				{
					for self.props.children.iter().enumerate().map(|(i, mut item)| {
						item.props.value = format!("#{}", i);
						item
					})
				}
			</div>
		}
	}
}
```

