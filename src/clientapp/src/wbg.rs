use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    pub fn cancelInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}