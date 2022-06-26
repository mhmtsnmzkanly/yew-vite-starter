pub mod components;
pub mod pages;
pub mod router;

use router::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AppMessages {}

#[derive(Debug, PartialEq, Properties, Default)]
pub struct AppProps;

#[derive(Debug, PartialEq)]
pub struct App;


impl App {
    pub fn default() -> Self {
        Self //{}
    }
}

impl Component for App {
    type Message = AppMessages;
    type Properties = AppProps;

    fn create(_ctx: &Context<Self>) -> Self {
		Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        log::info!("- update: msg => {0:?}", _msg);
		
        //match _msg {
            /*Self::Message::* => {
                false
            },*/
        //}
		false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<AppRoutables> render={Switch::render(route_app_pages)} />
            </BrowserRouter>
        }
    }
}
