use yew::{html, Component, Context, Html, Properties};

#[derive(Debug, PartialEq)]
pub enum t_Messages {}

#[derive(Debug, PartialEq, Properties)]
pub struct t_Props;

#[derive(Debug, PartialEq)]
pub struct t_;

impl t_ {
	pub fn default() -> Self {
        Self //{}
    }
}

impl Component for t_ {
    type Message = t_Messages;
    type Properties = t_Props;

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
