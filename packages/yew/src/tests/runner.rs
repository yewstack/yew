//! Testflow based testing of Yew components.
//!
//! This has to be run in the browser, and have the `csr` feature enabled.

use std::borrow::Cow;
use std::fmt;
use std::panic::Location;
use std::pin::Pin;

use futures::Future;
use web_sys::{Element, Text};

use crate::dom_bundle::{BSubtree, Bundle};
use crate::html::{AnyScope, Scope};
use crate::virtual_dom::VChild;
use crate::{scheduler, BaseComponent, Html, NodeRef};

#[derive(Debug)]
struct ReplayableLayout {
    _name: String,
    _html: Html,
    _expected: Cow<'static, str>,
}

/// The test runner controls a piece of the DOM on which your components are mounted.
///
/// You can then define sub-steps and test various properties of the result of rendering.
pub struct TestRunner {
    // Information needed for running the test
    scope: AnyScope,
    parent: Element,
    root: BSubtree,
    _end_node: Text,
    end_position: NodeRef,
    location: &'static Location<'static>,
    // Changing over the course of the test
    bundle: Bundle,
    // Collect a database of fully-specified layouts we can re-test again later
    full_layouts: Vec<ReplayableLayout>,
    unnamed_test_count: usize,
}

impl fmt::Debug for TestRunner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestRunner").finish_non_exhaustive()
    }
}

trait TestContext {
    // Get the underlying runner
    fn as_runner(&self) -> &TestRunner;
    // Get the underlying runner, mutably
    fn as_runner_mut(&mut self) -> &mut TestRunner;
    // Get the name of the current context
    fn name(&self) -> &str;
}

impl TestContext for TestRunner {
    fn as_runner(&self) -> &TestRunner {
        self
    }

    fn as_runner_mut(&mut self) -> &mut TestRunner {
        self
    }

    fn name(&self) -> &str {
        ""
    }
}

/// A substep of a flow based test.
///
/// You can recursively create more substeps, or render and test properties.
/// Borrows from the [`TestRunner`], since you can't run multiples tests on the same piece of DOM
/// concurrently.
pub struct TestStep<'s> {
    name: String,
    context: &'s mut dyn TestContext,
}

impl<'s> fmt::Debug for TestStep<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestStep")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl<'s> TestContext for TestStep<'s> {
    fn as_runner(&self) -> &TestRunner {
        self.context.as_runner()
    }

    fn as_runner_mut(&mut self) -> &mut TestRunner {
        self.context.as_runner_mut()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl TestRunner {
    /// Create a new context in which to run tests in the document body.
    #[track_caller]
    pub fn new() -> Self {
        let document = gloo_utils::document();
        let parent = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&parent).unwrap();
        Self::new_in(parent)
    }

    /// Create a new context in which to run tests, under the passed parent in the DOM.
    #[track_caller]
    pub fn new_in(parent: Element) -> Self {
        let scope: AnyScope = AnyScope::test();
        let root = BSubtree::create_root(&parent);

        let document = gloo_utils::document();
        let end_node = document.create_text_node("END");
        parent.append_child(&end_node).unwrap();
        let bundle = Bundle::new();

        Self {
            scope,
            parent,
            root,
            end_position: NodeRef::new(end_node.clone().into()),
            _end_node: end_node,
            location: Location::caller(),
            bundle,
            full_layouts: vec![],
            unnamed_test_count: 0,
        }
    }

    fn reconcile(&mut self, html: Html) {
        self.bundle.reconcile(
            &self.root,
            &self.scope,
            &self.parent,
            self.end_position.clone(),
            html,
        );
    }

    async fn apply(&mut self, html: Html) {
        self.reconcile(html.clone()); // Apply the layout twice to catch bad re-apply
        scheduler::start_now();
        self.reconcile(html);
        scheduler::start_now();
    }

    fn push_replayable_test(&mut self, replayable: ReplayableLayout) {
        self.full_layouts.push(replayable);
    }

    /// Re-apply "simple" test cases that have been passed in the various stages of the runner.
    /// This is a simple way to cross-test the interaction between deterministic layouts by trying
    /// different orders and how they get reconciled.
    #[track_caller]
    pub async fn run_replayable_tests(&mut self) {
        let layouts = std::mem::take(&mut self.full_layouts);

        for test in layouts.iter() {
            self.apply(test._html.clone()).await;
            assert_eq!(
                self.parent.inner_html(),
                format!("{}END", test._expected),
                "Sequential apply failed for layout '{}'",
                test._name,
            );
        }

        for test in layouts.into_iter().rev() {
            self.apply(test._html.clone()).await;
            assert_eq!(
                self.parent.inner_html(),
                format!("{}END", test._expected),
                "Sequential detach failed for layout '{}'",
                test._name,
            );
        }

        self.reconcile(Html::default());
        scheduler::start_now();
        assert_eq!(
            self.parent.inner_html(),
            "END",
            "Sequential detach failed for last layout",
        );
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
    name: String,
    generating_html: Option<Html>,
    full_layout: Option<&'static str>,
}

impl<'s> fmt::Debug for TestableState<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestableState")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<'s> TestableState<'s> {
    /// Retrieve the parent element under which the test has been mounted.
    ///
    /// Allows directly interacting with dom API, but marks this as a non-"simple" test case.
    pub fn parent(&mut self) -> Element {
        self.generating_html = None;
        self.context.as_runner().parent.clone()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn bundle(&mut self) -> &Bundle {
        self.generating_html = None;
        &self.context.as_runner().bundle
    }

    /// Test against an exactly given inner html that is supposedly render.
    ///
    /// Marks this test case as a "simple" test that can be replayed.
    #[track_caller]
    pub fn assert_inner_html(&mut self, expected: &'static str) {
        self.full_layout = Some(expected);
        let runner = self.context.as_runner();
        let inner_html = runner.parent.inner_html();
        assert_eq!(
            inner_html,
            format!("{}END", expected),
            "Independent attach failed for layout '{}'",
            self.name,
        );
    }
}

pub trait TestCase {
    fn step(&mut self, name: &'static str) -> TestStep<'_>;
    fn render<'s>(
        &'s mut self,
        html: Html,
    ) -> Pin<Box<dyn 's + Future<Output = TestableState<'_>>>>;
    fn render_app<'s, T: BaseComponent>(
        &'s mut self,
        html: VChild<T>,
    ) -> Pin<Box<dyn 's + Future<Output = (TestableState<'_>, Scope<T>)>>>;
    fn finish(self);
}

impl<TC: TestContext> TestCase for TC {
    fn step(&mut self, name: &'static str) -> TestStep<'_> {
        let name = if self.name().is_empty() {
            name.to_owned()
        } else {
            format!("{}.{}", self.name(), name)
        };
        TestStep {
            name,
            context: self,
        }
    }

    fn render<'s>(
        &'s mut self,
        html: Html,
    ) -> Pin<Box<dyn 's + Future<Output = TestableState<'_>>>> {
        async fn render_impl<TC: TestContext>(self_: &mut TC, html: Html) -> TestableState<'_> {
            let name = self_.name().to_owned();
            let runner = self_.as_runner_mut();
            runner.apply(html.clone()).await;
            TestableState {
                context: self_,
                name,
                generating_html: Some(html),
                full_layout: None,
            }
        }
        Box::pin(render_impl(self, html))
    }

    fn render_app<'s, T: BaseComponent>(
        &'s mut self,
        html: VChild<T>,
    ) -> Pin<Box<dyn 's + Future<Output = (TestableState<'s>, Scope<T>)>>> {
        async fn render_app_impl<TC: TestContext, T: BaseComponent>(
            self_: &mut TC,
            html: VChild<T>,
        ) -> (TestableState<'_>, Scope<T>) {
            let name = self_.name().to_owned();
            let runner = self_.as_runner_mut();
            let scope = runner.bundle.reconcile_vchild(
                &runner.root,
                &runner.scope,
                &runner.parent,
                runner.end_position.clone(),
                html,
            );
            scheduler::start_now();
            let state = TestableState {
                context: self_,
                name,
                generating_html: None,
                full_layout: None,
            };
            (state, scope)
        }
        Box::pin(render_app_impl(self, html))
    }

    fn finish(self) {}
}

impl<'s> Drop for TestStep<'s> {
    // Not #[track_caller], since that always points to ptr::mod.rs drop glue
    fn drop(&mut self) {
        let runner = self.context.as_runner_mut();
        runner.reconcile(Html::default());
        scheduler::start_now();
        assert_eq!(
            runner.parent.inner_html(),
            "END",
            "Independent detach failed for layout '{}'",
            self.name(),
        );
        gloo::console::log!(format!("Layout '{}' ... ok", self.name()));
    }
}

impl<'s> Drop for TestableState<'s> {
    fn drop(&mut self) {
        if self.name.is_empty() {
            let runner = self.context.as_runner_mut();
            runner.unnamed_test_count += 1;
        }
        if let Some(html) = self.generating_html.take() {
            if let Some(full_layout) = self.full_layout.take() {
                let saved_layout = ReplayableLayout {
                    _name: self.context.name().to_owned(),
                    _html: html,
                    _expected: full_layout.into(),
                };
                self.context
                    .as_runner_mut()
                    .push_replayable_test(saved_layout);
            }
        }
    }
}

impl Drop for TestRunner {
    fn drop(&mut self) {
        if self.unnamed_test_count > 0 {
            gloo::console::log!(format!(
                "[{}:{}:{}] {} unnamed layouts ... ok",
                self.location.file(),
                self.location.line(),
                self.location.column(),
                self.unnamed_test_count
            ));
        }
    }
}
