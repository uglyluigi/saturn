use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::*;

pub type Amogus<T> = EventBus<T>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request<T> {
    EventBusMsg(T),
}

pub struct EventBus<T: Clone + Serialize + Deserialize<'static> + 'static> {
    link: AgentLink<EventBus<T>>,
    subscribers: HashSet<HandlerId>,
}

impl<T: Clone + Serialize + Deserialize<'static> + 'static> Agent for EventBus<T> {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Request<T>;
    type Output = T;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match msg {
            Request::EventBusMsg(s) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, s.clone());
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}