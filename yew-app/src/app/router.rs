use super::pages::*;
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoutables {
    #[at("/")]
    Index,
    
	#[not_found]
    #[at("/404")]
    NotFound,
}

pub fn route_app_pages(route: &AppRoutables) -> Html {
    match route.clone() {
        AppRoutables::Index => {
            html! { <index::Index  /> }
        }
        AppRoutables::NotFound => {
            html! { <not_found::NotFound /> }
        }
    }
}