use yew::prelude::*;

pub struct ThreeJSViewport {
    link: ComponentLink<Self>,
    time: i64,
    props: Props,
}

pub enum Msg {
    InitThree,
    DoTick,
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
        Self { link, time: 0, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InitThree => match &self.props.effect {
                ThreeJSEffect::VertexStar => js::vertex_star::init(),
            },

            Msg::DoTick => {
                self.time += 1;
            }
        };

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="space-container">
                <div id="canvas_container">
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first: bool) {
        if first {
            self.link.send_message(Msg::InitThree)
        }
    }
}

pub mod js {
    pub mod vertex_star {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(module = "/src/js/rotating_vertex_star.js")]
        extern "C" {
            #[wasm_bindgen(js_name = "init")]
            pub fn init();
        }
    }
}
