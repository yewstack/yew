use yew::prelude::*;

pub(crate) struct TestResults;

pub(crate) enum ComponentMsg {
    CloseModalClicked,
}

impl Component for TestResults {
    type Message = ComponentMsg;
    type Properties = crate::app::ResultsModalProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onclick = ctx.link().callback(|_| ComponentMsg::CloseModalClicked);

        html! {
            <div class={"absolute h-screen bg-base-100/90 backdrop-blur-xs flex justify-center items-center z-40 w-full"}>
                <div class={"bg-base-200 size-1/2 rounded-lg shadow-md outline-none text-base-content"}>
                    <div class={"border-b md:p-5 p-2 flex items-center justify-between"}>
                        <h1 class="font-bold text-lg">{"Results"}</h1>
                        <button
                            class="size-9 inline-flex items-center justify-center bg-base-300 cursor-pointer rounded-lg"
                            {onclick}>
                            <span class={"icon-[mdi-light--plus] text-base-content text-4xl rotate-45"}></span>
                        </button>
                    </div>
                    <div class="h-full flex items-center justify-center">
                        <h3 class="italic font-semibold">{"For the reader: Implement your own way of showing results here!"}</h3>
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentMsg::CloseModalClicked => {
                ctx.props().on_test_results_modal_closed.emit(());
                true
            }
        }
    }
}
