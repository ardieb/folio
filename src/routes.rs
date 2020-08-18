use yew_router::switch::{Permissive};
use yew_router::{Switch};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/cv"]
    Cv,
    #[to = "/projects"]
    Projects,
    #[to = "/not-found"]
    NotFound(Permissive<String>),
    #[to = "/"]
    About,
}