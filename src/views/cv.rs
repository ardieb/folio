use log::*;
use yew::prelude::*;

struct Cv {}

impl Component for Cv {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self {} }

    fn update(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        info!("rendered Cv!");

        html! {
        <>
            <article class="post">
                <header>
                    <div class="title">
                        <h2>{"Cv"}</h2>
                        <div class="link-container">
                            <h4>
                                <a href={"#Education"}>{"Education"}</a>
                                <a href={"#Experience"}>{"Experience"}</a>
                                <a href={"#Skills"}>{"Skills"}</a>
                            </h4>
                        </div>
                    </div>
                </header>
            </article>
        </>
        }
    }
}