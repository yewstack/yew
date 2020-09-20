use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="section tile is-ancestor">
                <div class="tile is-parent">
                    <article class="tile is-child notification is-info">
                        <p class="title">{ "Welcome..." }</p>
                        <p class="subtitle">{ "...to the best yew content" }</p>
                        <figure class="image is-4by3">
                            <img src="https://source.unsplash.com/r_Gozq2ApBU/1280x960" />
                        </figure>
                    </article>
                </div>
                <div class="tile is-parent is-vertical">
                    <article class="tile is-child notification is-danger">
                        <p class="title">{ "What are yews?" }</p>
                        <p class="subtitle">{ "Everything you need to know!" }</p>
                        <div class="content">
                        {r#"
                            A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
                            The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
                            The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
                            but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
                            except on erect leading shoots where the spiral arrangement is more obvious.
                            The leaves are poisonous.
                        "#}
                        </div>
                    </article>
                    <article class="tile is-child notification is-primary">
                        <p class="title">{ "Who are we?" }</p>
                        <div class="content">
                        { "We're a small team of just 2" }
                        <sup>{ 64 }</sup>
                        { " members working tirelessly to bring you the low-effort yew content we all desperately crave." }
                        <br />
                        {r#"
                            We put a ton of effort into fact-checking our posts.
                            Some say they read like a Wikipedia article and we take that as a compliment!
                        "#}
                        </div>
                    </article>
                </div>
            </div>
        }
    }
}
