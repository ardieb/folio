use log::*;
use yew::prelude::*;

use crate::data::models::Skill;
use crate::data::models::Education;
use crate::data::models::Experience;

use crate::data::store::EXPERIENCE;
use crate::data::store::EDUCATION;
use crate::data::store::SKILLS;

pub struct Cv {
    link: ComponentLink<Self>,
}

pub enum Msg {
    ScrollTo(String),
}

impl Component for Cv {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self { Self { link } }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let doc = yew::utils::document();
        match msg {
            Msg::ScrollTo(id) => {
                info!("Scroll to {}", id);
                let el = doc.get_element_by_id(id.as_str()).expect("Cannot find element with `id`");
                el.scroll_to();
                info!("Updated cv!");
                true
            }
        }
    }

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
                            <a
                                href={"#education"}
                                onclick=self.link.callback(|_| Msg::ScrollTo(String::from("education")))
                            >
                                <h4>{"Education"}</h4>
                            </a>
                            <a
                                href={"#experience"}
                                onclick=self.link.callback(|_| Msg::ScrollTo(String::from("experience")))
                            >
                                <h4>{"Experience"}</h4>
                            </a>
                            <a
                                href={"#expertise"}
                                onclick=self.link.callback(|_| Msg::ScrollTo(String::from("skills")))
                            >
                                <h4>{"Expertise"}</h4>
                            </a>
                        </div>
                    </div>
                </header>
                <div class="education">
                    <div class="link-to" id="education" />
                    <div class="title">
                        <h3>{"Education"}</h3>
                    </div>
                </div>
                {EDUCATION.iter().map(|education: &Education| {
                    html!{
                        <article class="jobs-container">
                            <header>
                                <h4 class="degree">{format!("{} - {}", education.school,education.degree)}</h4>
                                <p class="daterange">{format!("{} - {}",education.start, match education.end {
                                    Some(end) => end,
                                    None => "current",
                                })}</p>
                                <p>{format!("{}\n - GPA: {}", education.desc, education.gpa)}</p>
                            </header>
                        </article>
                    }
                }).collect::<Html>()}
                <hr />
                <div class="experience">
                    <div class="link-to" id="experience" />
                    <div class="title">
                        <h3>{"Experience"}</h3>
                    </div>
                    {EXPERIENCE.iter().map(|experience: &Experience| {
                        html!{
                            <article class="jobs-container">
                                <header>
                                    <h4>
                                        <a href={match experience.href {
                                            Some(link) => link,
                                            None => "",
                                        }}>{
                                            format!("{} - {}",experience.title,experience.org)
                                        }</a>
                                    </h4>
                                    <p class="daterange">{format!("{} - {}",experience.start,match experience.end {
                                        Some(end) => end,
                                        None => "current",
                                    })}</p>
                                </header>
                                <p>{experience.desc}</p>
                            </article>
                        }
                    }).collect::<Html>()}
                </div>
                <hr />
                <div class="experience">
                    <div class="link-to" id="expertise" />
                    <div class="title">
                        <h3>{"Expertise"}</h3>
                    </div>
                    {SKILLS.iter().map(|skill: &Skill| {
                        html!{
                            <article>
                                <div class="skill-row-container">
                                    <h4>{skill.name}</h4>
                                    <p>{skill.details}</p>
                                </div>
                            </article>
                        }
                    }).collect::<Html>()}

                </div>
            </article>
        </>
        }
    }
}