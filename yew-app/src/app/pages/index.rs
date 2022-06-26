use yew::{html, Component, Context, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum IndexMessages {}

#[derive(Debug, PartialEq, Properties)]
pub struct IndexProps;

#[derive(Debug, PartialEq)]
pub struct Index;

impl Index {
	pub fn default() -> Self {
        Self //{}
    }
}

impl Component for Index {
    type Message = IndexMessages;
    type Properties = IndexProps;

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
		<>
		</>
     }
    }
}
