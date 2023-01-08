use yew::{function_component, Html, Properties};

#[derive(PartialEq, Properties, Default)]
pub struct TestScaffoldProps {
    #[prop_or_default]
    pub test_case: Html,
}

#[function_component]
pub fn TestScaffold(TestScaffoldProps { test_case }: &TestScaffoldProps) -> Html {
    test_case.clone()
}
