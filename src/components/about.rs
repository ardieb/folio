use log::*;
use yew::prelude::*;

use crate::data::models::Contact;
use crate::data::store::CONTACTS;
use crate::data::store::BLURB;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self {} }

    fn update(&mut self, _: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        info!("rendered About!");
        html! {
        <>
            <article class="post" id="About">
                <header>
                    <div class="title">
                        <h2>{"About"}</h2>
                    </div>
                </header>
                <p>{
                    "Welcome to my website! Seems normal right? \n\
                    Under the hood, there's some really cool stuff going on. \n\
                    Aside from the root About.html and .scss files, all the code is powered by rust \n\
                    and the yew front end framework. This creates the foundation for a lightning fast webapp that runs\n
                    on webassembly code. "
                }</p>
                <p>{BLURB}</p>
            </article>
            <article class="post" id="contact">
                <header>
                    <div class="title">
                        <h2>{"Contact"}</h2>
                    </div>
                </header>
                <div class="email-at">
                    <ul class="icons">
                        {CONTACTS.iter().map(|contact: &Contact| {
                            match contact.icon {
                                Some(ico) => html!{
                                    <li>
                                        <a href={contact.link}>
                                            <i class={ico} aria-hidden={"true"} />
                                        </a>
                                    </li>
                                },
                                None => html!{
                                    <li>
                                        <a href={contact.link}>
                                            {contact.label}
                                        </a>
                                    </li>
                                },
                            }
                        }).collect::<Html>()}
                    </ul>
                </div>
              </article>
        </>
        }
    }
}
