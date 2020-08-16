use log::*;
use yew::prelude::*;
use crate::data::models::Project;
use crate::data::store::PROJECTS;

pub struct Projects {}

impl Component for Projects {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self {} }

    fn update(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        info!("rendered Projects!");

        html! {
        <>
            <article class="post">
                <header>
                    <div class="title">
                        <h2>{"Personal Projects I've Developed"}</h2>
                    </div>
                </header>
            </article>
        </>
        }
    }
}
