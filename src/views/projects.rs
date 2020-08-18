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
                {PROJECTS.iter().map(|project: &Project| {
                    html!{
                        <div>
                            <article className="jobs-container">
                                <header>
                                    {match project.href {
                                        Some(link) => html!{<a href={link}><h4>{project.name}</h4></a>},
                                        None => html!{<h4>{project.name}</h4>},
                                    }}
                                    <p className="daterange">{project.start}</p>
                                </header>
                                <div>
                                    <p className="description">{project.desc}</p>
                                </div>
                            </article>
                            <hr />
                        </div>
                    }
                }).collect::<Html>()}
            </article>
        </>
        }
    }
}
