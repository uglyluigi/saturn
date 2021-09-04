use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct ThreeJSViewport {
    link: ComponentLink<Self>,
    time: i64,
}

pub enum Msg {
    InitThree,
    DoTick,
}


impl Component for ThreeJSViewport {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        //TODO like button is already toggled based on if the user liked this club
        Self { link, time: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InitThree => init(),

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
            </div>
        }
    }

    fn rendered(&mut self, first: bool) {
        if first {
            self.link.send_message(Msg::InitThree)
        }
    }
}

#[wasm_bindgen(module = "/src/effect.js")]
extern "C" {
    #[wasm_bindgen(js_name = "init")]
    pub fn init();
}