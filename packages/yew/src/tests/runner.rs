use std::borrow::Cow;
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

pub struct TestStep<'s> {
    name: String,
    context: &'s mut dyn TestContext,
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
    pub fn new() -> Self {
        let document = gloo_utils::document();
        let parent = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&parent).unwrap();
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
    generating_html: Option<Html>,
    full_layout: Option<&'static str>,
}

impl<'s> TestableState<'s> {
    pub fn parent(&mut self) -> Element {
        self.generating_html = None;
        self.context.as_runner().parent.clone()
    }

    pub(crate) fn bundle(&mut self) -> &Bundle {
        self.generating_html = None;
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
