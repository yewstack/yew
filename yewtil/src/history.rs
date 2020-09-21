use std::collections::VecDeque;
use std::ops::Deref;

// TODO when const generics lands, it would be useful to add a usize type parameter over the max number of elements.

// It would also be interesting to see if a diffing history implementation could be built over types
// that can represent themselves as diffs - where reversible transitions can be recorded instead of values
// and the transitions can be rolled back.
// That would probably have worse performance in exchange for smaller size.

/// Wrapper that keeps track of prior values that have been assigned to it.
///
/// It keeps values that have been `set` for it around for the duration of its lifetime,
/// or until they are dropped by calling `reset` or `forget`.
///
/// Prior values can be iterated over as well.
///
/// # Example
///
/// ```
/// # use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};
/// use yewtil::History;
///
/// pub enum Msg {
///     SetText(String),
///     Reset,
///     Forget,
/// }
///
/// pub struct Model {
///     link: ComponentLink<Self>,
///     text: History<String>,
/// }
/// impl Component for Model {
///     type Message = Msg;
/// #    type Properties = ();
///
///     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
///         Self {
///             link,
///             text: History::new("Hello World!".to_string()),
///         }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         match msg {
///             Msg::SetText(text) => self.text.neq_set(text),
///             Msg::Reset => self.text.reset(),
///             Msg::Forget => {
///                 self.text.forget();
///                 false
///             }
///         }
///     }
/// #
/// #    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
/// #        unimplemented!()
/// #    }
///
///     fn view(&self) -> Html {
///         html! {
///             <>
///                 <span>{ &*self.text }</span>
///                 <div>
///                     <input
///                         type="text"
///                         value=&*self.text
///                         oninput=self.link.callback(|data: InputData| Msg::SetText(data.value))
///                     />
///                     <button onclick=self.link.callback(|_| Msg::Reset)>{ "Reset to the oldest value" }</button>
///                     <button onclick=self.link.callback(|_| Msg::Forget)>{ "Forget prior values" }</button>
///                 </div>
///                 <div>
///                     <span>{ "History" }</span>
///                     { for self.text.iter() }
///                 </div>
///             </>
///         }
///     }
/// }
/// ```
pub struct History<T>(VecDeque<T>);

impl<T> History<T> {
    /// Creates a new history wrapper.
    pub fn new(value: T) -> Self {
        let mut vec = VecDeque::new();
        vec.push_front(value);
        Self(vec)
    }

    /// Set the value represented by the `History` struct.
    ///
    /// This pushes the new value into the front of a list,
    /// where the front-most value represents the most recent value.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// history.set(1);
    ///
    /// assert_eq!(*history, 1);
    /// assert_eq!(history.count(), 2);
    /// ```
    pub fn set(&mut self, value: T) {
        self.0.push_front(value)
    }

    /// Replaces the current value without creating a history entry.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// history.replace(1);
    ///
    /// assert_eq!(*history, 1);
    /// assert_eq!(history.count(), 1);
    /// ```
    pub fn replace(&mut self, value: T) {
        self.0[0] = value;
    }

    /// Removes all prior values.
    ///
    /// This effectively sets a new "checkpoint" that can be restored by calling `reset`.
    ///
    /// The returned bool indicates if any entries were removed.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// history.set(1);
    /// history.set(2);
    ///
    /// history.forget();
    /// assert_eq!(*history, 2);
    /// assert_eq!(history.count(), 1);
    /// ```
    pub fn forget(&mut self) -> bool {
        if self.dirty() {
            self.0.drain(1..);
            true
        } else {
            false
        }
    }

    /// Remove all elements except the last one, making the oldest entry the "current value".
    ///
    /// The returned bool indicates if any entries were removed.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// history.set(1);
    /// history.set(2);
    ///
    /// history.reset();
    /// assert_eq!(*history, 0);
    /// assert_eq!(history.count(), 1);
    /// ```
    pub fn reset(&mut self) -> bool {
        if self.dirty() {
            self.0.drain(..self.0.len() - 1);
            true
        } else {
            false
        }
    }

    /// Returns true if there is more than one entry in the history.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// history.set(1);
    /// assert!(history.dirty());
    /// ```
    pub fn dirty(&mut self) -> bool {
        self.count() > 1
    }

    /// Returns the number of entries in the history.
    ///
    /// This will never be less than 1, as the first entry of the backing VecDeque is always occupied by the
    /// "current value" in the `History` struct.
    ///
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// assert_eq!(history.count(), 1);
    ///
    /// history.set(1);
    /// assert_eq!(history.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Produces an iterator over references to history items ordered from newest to oldest.
    pub fn iter(&self) -> std::collections::vec_deque::Iter<T> {
        self.0.iter()
    }

    /// Gets the current value.
    pub fn into_inner(mut self) -> T {
        self.0
            .pop_front()
            .expect("History should have at least one item")
    }
}

impl<T: PartialEq> History<T> {
    /// Will only `set` the value if the provided value is different than the current value.
    ///
    /// It returns true to indicate if the history's current value was updated to be the provided value.
    /// # Example
    /// ```
    ///# use yewtil::History;
    /// let mut history = History::new(0);
    /// let did_set = history.neq_set(0);
    /// assert!(!did_set);
    ///
    /// let did_set = history.neq_set(1);
    /// assert!(did_set);
    /// ```
    pub fn neq_set(&mut self, value: T) -> bool {
        if self.0[0] != value {
            self.set(value);
            true
        } else {
            false
        }
    }
}

impl<T> IntoIterator for History<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> AsRef<T> for History<T> {
    fn as_ref(&self) -> &T {
        // Get the first element
        &self.0[0]
    }
}

impl<T> Deref for History<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
