use log::*;
use yew::prelude::*;

pub struct Index {}

impl Component for Index
{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self {} }

    fn update(&mut self, _: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html
    {
        info!("rendered index!");
        html!
        {
            <article class="post" id="index">
                <header>
                    <div class="title">
                        <h2>{"About this site"}</h2>
                    </div>
                </header>
                <p>{
                    "Welcome to my website! Seems normal right? \n\
                    Under the hood, there's some really cool stuff going on. \n\
                    Aside from the root index.html and .scss files, all the code is powered by rust \n\
                    and the yew front end framework. This creates the foundation for a lightning fast webapp that runs\n
                    on webassembly code. "
                }</p>
            </article>
        }
    }
}
