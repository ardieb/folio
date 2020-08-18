use log::*;
use yew::prelude::*;

use yew_router::switch::{Permissive};
use yew_router::{prelude::*};

use crate::views::cv::Cv;
use crate::views::about::About;
use crate::views::projects::Projects;

use crate::routes::AppRoute;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self {} }

    fn update(&mut self, _: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _: Self::Properties) -> ShouldRender { false }

    fn view(&self) -> Html {
        info!("rendered app!");
        html! {
            <div id="wrapper">
                <header id="header">
                    <h1 class="index-link">
                        <RouterAnchor<AppRoute> route=AppRoute::About> {"Arthur Miles Burke"} </RouterAnchor<AppRoute>>
                    </h1>
                    <nav class="links">
                        <ul>
                            <li><RouterAnchor<AppRoute> route=AppRoute::About> {"About"} </RouterAnchor<AppRoute>></li>
                            <li><RouterAnchor<AppRoute> route=AppRoute::Cv> {"Cv"} </RouterAnchor<AppRoute>></li>
                            <li><RouterAnchor<AppRoute> route=AppRoute::Projects> {"Projects"} </RouterAnchor<AppRoute>></li>
                        </ul>
                    </nav>
                </header>
                <div id="main">
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::About => html!{ <About /> },
                                AppRoute::Cv => html!{ <Cv /> },
                                AppRoute::Projects => html!{ <Projects /> },
                                AppRoute::NotFound(Permissive(None)) => html!{"Page Not Found"},
                                AppRoute::NotFound(Permissive(Some(missed))) => html!{format!("Page '{}' not found", missed)},
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::NotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
            </div>
        }
    }
}
