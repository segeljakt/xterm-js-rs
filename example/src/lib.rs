mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, MessageEvent, Window};
use xterm_js::*;
use std::sync::{Mutex, Arc};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn prompt(term: &Terminal) {
    term.write("\r\n$ ".to_owned(), None);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

    let term: Terminal = Terminal::new(None);

    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap()
        .dyn_into::<HtmlElement>()?;

    term.open(elem);

    let mut buf = Arc::new(Mutex::new(String::new()));
    let mut outer = buf.clone();

    let callback_term: Terminal = term.clone().dyn_into()?;
    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        prompt(&callback_term);
        buf.as_ref().lock().unwrap().push_str(&e.key());
        callback_term.write(format!("Entered {}", e.key()), None);
    }) as Box<dyn FnMut(_)>);

    term.on_key(callback.as_ref().unchecked_ref());

    outer.as_ref().lock().unwrap().len();

    callback.forget();

    term.focus();

    Ok(())
}
