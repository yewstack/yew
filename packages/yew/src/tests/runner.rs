use std::borrow::Cow;
use std::pin::Pin;

use futures::Future;
use web_sys::{Element, Text};

use crate::dom_bundle::{BSubtree, Bundle};
use crate::html::AnyScope;
use crate::{scheduler, Html, NodeRef};

#[derive(Debug)]
struct ReplayableLayout {
    _name: String,
    _html: Html,
    _expected: Cow<'static, str>,
}

pub struct TestRunner {
    // Information needed for running the test
    scope: AnyScope,
    parent: Element,
    root: BSubtree,
    _end_node: Text,
    end_position: NodeRef,
    // Changing over the course of the test
    bundle: Bundle,
    // Collect a database of fully-specified layouts we can re-test again later
    full_layouts: Vec<ReplayableLayout>,
}

trait TestContext {
    fn as_runner(&self) -> &TestRunner;
    fn as_runner_mut(&mut self) -> &mut TestRunner;
    fn wrap_html(&self, html: Html) -> Html;
    fn name(&self) -> &str;
}

impl TestContext for TestRunner {
    fn as_runner(&self) -> &TestRunner {
        self
    }

    fn as_runner_mut(&mut self) -> &mut TestRunner {
        self
    }

    fn wrap_html(&self, html: Html) -> Html {
        html
    }

    fn name(&self) -> &str {
        ""
    }
}

pub struct TestStep<'s> {
    name: String,
    context: &'s mut dyn TestContext,
    wrap_html: Box<dyn 's + Fn(Html) -> Html>,
}

impl<'s> TestContext for TestStep<'s> {
    fn as_runner(&self) -> &TestRunner {
        self.context.as_runner()
    }

    fn as_runner_mut(&mut self) -> &mut TestRunner {
        self.context.as_runner_mut()
    }

    fn wrap_html(&self, html: Html) -> Html {
        (self.wrap_html)(html)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl TestRunner {
    pub fn new() -> Self {
        let document = gloo_utils::document();
        let parent = document.create_element("div").unwrap();
        Self::new_in(parent)
    }

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
            bundle,
            full_layouts: vec![],
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

pub struct TestableState<'s> {
    context: &'s mut dyn TestContext,
    name: String,
    html: Html,
    is_simple: bool,
    full_layout: Option<&'static str>,
}

impl<'s> TestableState<'s> {
    pub fn parent(&mut self) -> Element {
        self.is_simple = false;
        self.context.as_runner().parent.clone()
    }

    pub(crate) fn bundle(&mut self) -> &Bundle {
        self.is_simple = false;
        &self.context.as_runner().bundle
    }

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
    fn finish(self);
}

impl<T: TestContext> TestCase for T {
    fn step(&mut self, name: &'static str) -> TestStep<'_> {
        let name = if self.name().is_empty() {
            name.to_owned()
        } else {
            format!("{}.{}", self.name(), name)
        };
        TestStep {
            name,
            context: self,
            wrap_html: Box::new(|html| html),
        }
    }

    fn render<'s>(
        &'s mut self,
        html: Html,
    ) -> Pin<Box<dyn 's + Future<Output = TestableState<'_>>>> {
        async fn then_impl<T: TestContext>(self_: &mut T, html: Html) -> TestableState<'_> {
            let html = self_.wrap_html(html);
            let name = self_.name().to_owned();
            let runner = self_.as_runner_mut();
            runner.apply(html.clone()).await;
            TestableState {
                context: self_,
                name,
                html,
                is_simple: true,
                full_layout: None,
            }
        }
        Box::pin(then_impl(self, html))
    }

    fn finish(self) {}
}

impl<'s> Drop for TestStep<'s> {
    #[track_caller]
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
        if self.is_simple {
            if let Some(full_layout) = self.full_layout {
                let saved_layout = ReplayableLayout {
                    _name: self.context.name().to_owned(),
                    _html: std::mem::take(&mut self.html),
                    _expected: full_layout.into(),
                };
                self.context
                    .as_runner_mut()
                    .push_replayable_test(saved_layout);
            }
        }
    }
}
