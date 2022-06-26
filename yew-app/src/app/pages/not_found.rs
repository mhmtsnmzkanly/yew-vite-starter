use yew::{html, Component, Context, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum NotFoundMessages {}

#[derive(Debug, PartialEq, Properties)]
pub struct NotFoundProps;

#[derive(Debug, PartialEq)]
pub struct NotFound;

impl NotFound {
	pub fn default() -> Self {
        Self //{}
    }
}

impl Component for NotFound {
    type Message = NotFoundMessages;
    type Properties = NotFoundProps;

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
        log::info!("- view");
        
		html! {
            <>
				<h1 class="title"> { "404 Page Not Found" } </h1>
				<p class="subtitle"> { "An unexpected error has occurred. Please contact the site owner." } </p>
				<a class="button" href="/"> { "Home" } </a>
			</>
        }
    }
}
