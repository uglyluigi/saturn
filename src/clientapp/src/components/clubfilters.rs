use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

pub struct ToolbarComponent {
    link: ComponentLink<Self>,
    props: Props,
}

enum Msg {
    NowActive(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub username: String,
}

impl Component for ToolbarComponent {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="clubfilters">
                <h1>{"Hi, "} {self.props.username.clone()}</h1>
                <div class="links">
                    <a class="active" href="/">{"Home"}</a>
                    <a href="#smth">{"Something"}</a>
                </div>
            </div>
        }
    }
    use filters::filter::Filter;
        let a = (|&a: &usize|{ a > 5 }).and_not(|&a: &usize| a < 20).or(|&a: &usize| a == 10);
        // We now have ((a > 5) && !(a < 20) ) || a == 10

        assert_eq!(a.filter(&21), true);
        assert_eq!(a.filter(&10), true);
        assert_eq!(a.filter(&11), false);
        assert_eq!(a.filter(&5), false);
}
