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
                <article>
                <h3>{"About This Site"}</h3>
                    <p>{r"
                        Welcome to my website. It's powered entirely by the yew frontend rust
                        framework. There is no javascript or html code (aside from
                        the webpack.config.js file and root index.html). It's all rust compiled
                        down to web assembly. I believe web assembly projects will be increasingly
                        prevalent in the web ecosystem and foresee many more web projects powered by
                        rust or emscripten going forwards.
                    "}</p>
                </article>
                <hr />
                <article>
                <h3>{"About Me"}</h3>
                    <p>{r"
                        I'm a full stack software engineer and physicist. My interests a pretty diverse;
                        Most of my free time is spent cooking or weightlifting, occasionally gaming,
                        and often writing code for personal projects. I am a senior at Cornell.
                        Throughout my experience there, I have been drawn to a variety of people and
                        activities; I am a member of the Cornell Political Union, a former member of
                        the Engineers for a Sustainable World project team, and a former member of the
                        CisLunar Satellite Mission project team. Some of my favourite Cornell courses have
                        been History of the British Empire, Introduction to China, Applications of Quantum
                        Mechanics, and Computer Networks and Design.
                    "}</p>
                </article>
                <hr />
                <h3>{"Contact Me"}</h3>
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
