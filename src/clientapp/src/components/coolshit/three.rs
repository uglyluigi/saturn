use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct ThreeJSViewport {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    InitThree,
}

#[derive(Clone)]
pub enum ThreeJSEffect {
    VertexStar,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_else(|| ThreeJSEffect::VertexStar)]
    effect: ThreeJSEffect,
}

impl Component for ThreeJSViewport {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        //TODO like button is already toggled based on if the user liked this club
        Self {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InitThree => match &self.props.effect {
                ThreeJSEffect::VertexStar => {
                },
            },
        };

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <canvas id="login-canvas"/>
                <script src="./assets/bg.js"/>
            </div>
        }
    }

    fn rendered(&mut self, first: bool) {
        if first {
            self.link.send_message(Msg::InitThree)
        }
    }
}