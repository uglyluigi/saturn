use lazy_static::lazy_static;

lazy_static! {
    pub static ref IS_DEBUG_MODE: bool = yew::utils::host().unwrap().starts_with("localhost");
}