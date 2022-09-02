//! A procedural style setup for a test runner

use std::fmt;

use web_sys::Element;
use yew::{AppHandle, Html, Renderer};

use crate::scaffold::{TestScaffold, TestScaffoldProps};

/// The test runner controls a piece of the DOM on which your components are mounted.
///
/// Use the [`TestCase`] trait to access the testing functionality.
pub struct TestRunner {
    test_app: Option<AppHandle<TestScaffold>>,
    parent: Element,
}

impl fmt::Debug for TestRunner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestRunner").finish_non_exhaustive()
    }
}

impl Drop for TestRunner {
    fn drop(&mut self) {
        if let Some(app) = self.test_app.take() {
            app.destroy();
        }
    }
}

/// Helper trait for things that can be more-or-less treated as a [`TestRunner`].
///
/// Automatically gets an implementation of [`TestCase`].
pub trait TestContext {
    /// Get the underlying runner
    fn as_runner(&self) -> &TestRunner;
    /// Get the underlying runner, mutably
    fn as_runner_mut(&mut self) -> &mut TestRunner;
}

impl TestContext for TestRunner {
    fn as_runner(&self) -> &TestRunner {
        self
    }

    fn as_runner_mut(&mut self) -> &mut TestRunner {
        self
    }
}

impl TestRunner {
    /// Create a new context in which to run tests in the document body.
    pub fn new() -> Self {
        let document = gloo::utils::document();
        let parent = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&parent).unwrap();
        Self::new_in(parent)
    }

    /// Create a new context in which to run tests, under the passed parent in the DOM.
    pub fn new_in(parent: Element) -> Self {
        Self {
            test_app: None,
            parent,
        }
    }

    fn ensure_initialized(&mut self) -> &mut AppHandle<TestScaffold> {
        self.test_app.get_or_insert_with(|| {
            let handle = Renderer::with_root(self.parent.clone()).render();
            yew::scheduler::__unstable_start_now();
            handle
        })
    }

    fn reconcile(&mut self, test_case: Html) {
        self.ensure_initialized()
            .update(TestScaffoldProps { test_case })
    }

    fn apply(&mut self, html: Html) {
        self.reconcile(html.clone()); // Apply the layout twice to catch bad re-apply
        yew::scheduler::__unstable_start_now();
        self.reconcile(html);
        yew::scheduler::__unstable_start_now();
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Access to the dom state after rendering a specific html.
///
/// Test properties and inspect the dom while you have one of these in sope.
/// Borrows the context, so that you don't accidentally override the dom state with another test.
pub struct TestableState<'s> {
    context: &'s mut dyn TestContext,
}

impl<'s> fmt::Debug for TestableState<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestableState").finish_non_exhaustive()
    }
}

impl<'s> TestableState<'s> {
    /// Retrieve the DOM element under which the test has been mounted.
    ///
    /// Allows directly interacting with DOM API.
    pub fn dom_root(&self) -> Element {
        self.context.as_runner().parent.clone()
    }

    /// Test against an exactly given inner html that is supposedly rendered.
    #[track_caller]
    pub fn assert_inner_html(&self, expected: &'static str) {
        let runner = self.context.as_runner();
        let inner_html = runner.parent.inner_html();
        assert_eq!(inner_html, expected, "inner html should match");
    }
}

#[cfg(feature = "hydration")]
mod feat_ssr_hydrate {
    pub(super) use std::future::Future;
    use std::ops::{Deref, DerefMut};
    pub(super) use std::pin::Pin;

    use super::*;

    /// Access to the dom state, just before hydration occurs.
    pub struct HydratableState<'s> {
        pub(super) state: TestableState<'s>,
        pub(super) test_case: Html,
    }

    impl fmt::Debug for HydratableState<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HydratableState").finish_non_exhaustive()
        }
    }

    impl<'s> Deref for HydratableState<'s> {
        type Target = TestableState<'s>;

        fn deref(&self) -> &Self::Target {
            &self.state
        }
    }

    impl<'s> DerefMut for HydratableState<'s> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.state
        }
    }

    impl<'s> HydratableState<'s> {
        /// Hydrate the application now, and wait for it to render
        pub fn hydrate(self) -> TestableState<'s> {
            let runner = self.state.context.as_runner_mut();
            // Hydrate the component into place
            runner.test_app = Some(
                Renderer::with_root_and_props(
                    runner.parent.clone(),
                    TestScaffoldProps {
                        test_case: self.test_case,
                    },
                )
                .hydrate(),
            );
            // Flush lifecycles, then return
            yew::scheduler::__unstable_start_now();
            TestableState {
                context: self.state.context,
            }
        }
    }
}
#[cfg(feature = "hydration")]
pub use feat_ssr_hydrate::*;

/// Common methods to [`TestRunner`] and [`TestStep`]. Automatically implemented for any type
/// implementing [`TestContext`].
pub trait TestCase {
    /// Wait for dom events and lifecycle events to be committed to the dom.
    ///
    /// It's a good idea to call this after dispatching events or other manual mutations.
    fn process_events(&mut self) -> TestableState<'_>;
    /// Pretend that `test_case` was rendered by the server and prepare the dom to hydrate it.
    ///
    /// Call [`HydratableState::hydrate`] to actually actually hydrate the test case. This is done
    /// in two steps so that you can inspect the html as it would be returned from a server.
    #[cfg(feature = "hydration")]
    fn prepare_hydrate(
        &mut self,
        test_case: Html,
    ) -> Pin<Box<dyn '_ + Future<Output = HydratableState<'_>>>>;
    /// Render some [`html!`](yew::html!).
    ///
    /// # Example
    ///
    /// ```no_run
    /// let test_runner = TestRunner::new();
    /// test_runner
    ///     .render(html! {
    ///         <button>{"Click me!"}</button>
    ///     })
    ///     .assert_inner_html(r#"<button>Click me!</button>"#);
    /// ```
    fn render(&mut self, html: Html) -> TestableState<'_>;
    /// Alias for dropping the [`TestCase`].
    fn finish(self);
}

impl<TC: TestContext> TestCase for TC {
    fn process_events(&mut self) -> TestableState<'_> {
        yew::scheduler::__unstable_start_now();
        TestableState { context: self }
    }

    #[cfg(feature = "hydration")]
    fn prepare_hydrate(
        &mut self,
        html: Html,
    ) -> Pin<Box<dyn '_ + Future<Output = HydratableState<'_>>>> {
        use yew::LocalServerRenderer;

        async fn prepare_hydrate_impl(
            self_: &mut dyn TestContext,
            test_case: Html,
        ) -> HydratableState<'_> {
            // First, render the component with ssr rendering
            let rendered = LocalServerRenderer::<TestScaffold>::with_props(TestScaffoldProps {
                test_case: test_case.clone(),
            });
            let rendered = rendered.render().await;
            // Clear the parent, and replace it with ssr rendering result
            let runner = self_.as_runner_mut();
            runner.reconcile(Html::default());
            runner.parent.set_inner_html(&rendered);
            HydratableState {
                state: TestableState { context: self_ },
                test_case,
            }
        }
        Box::pin(prepare_hydrate_impl(self, html))
    }

    fn render(&mut self, test_case: Html) -> TestableState<'_> {
        self.as_runner_mut().apply(test_case);
        TestableState { context: self }
    }

    fn finish(self) {}
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    extern crate self as yew_test_runner;

    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    use yew::html;
    use yew_test_runner::procedural::{TestCase, TestRunner};

    #[test]
    fn render_functionality() {
        let mut test_runner = TestRunner::new();
        test_runner
            .render(html! {
                <button>{"Click me!"}</button>
            })
            .assert_inner_html(r#"<button>Click me!</button>"#);
    }

    #[test]
    async fn ssr_functionality() {
        let mut test_runner = TestRunner::new();
        test_runner
            .render(html! {
                <i>{"Some content before"}</i>
            })
            .assert_inner_html(r#"<i>Some content before</i>"#);
        test_runner
            .prepare_hydrate(html! {
                <button>{"Click me!"}</button>
            })
            .await
            .hydrate()
            .assert_inner_html(r#"<button>Click me!</button>"#);
        test_runner
            .render(html! {
                <i>{"Some other content"}</i>
            })
            .assert_inner_html(r#"<i>Some other content</i>"#);
    }
}
