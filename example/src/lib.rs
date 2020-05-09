mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use xterm_js::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn prompt(term: &Terminal) {
    term.write("\r\n$ ".to_owned(), None);
}

// Keyboard keys
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_L: u32 = 76;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let terminal: Terminal = Terminal::new(None);

    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();

    terminal.open(elem.dyn_into()?);
    prompt(&terminal);

    let mut line = String::new();

    let term: Terminal = terminal.clone().dyn_into()?;
    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();
        match event.key_code() {
            KEY_ENTER => {
                if !line.is_empty() {
                    let msg = format!("You entered {} characters '{}'", line.len(), line);
                    term.writeln("".to_owned(), None);
                    term.writeln(msg, None);
                    line.clear();
                }
                prompt(&term);
            }
            KEY_BACKSPACE => {
                if line.len() > 2 {
                    term.write(r"\b \b".to_owned(), None);
                    line.pop();
                }
            }
            KEY_L if event.ctrl_key() => {
                term.clear();
            }
            _ => {
                if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    term.write(event.key(), None);
                    line.push_str(&e.key());
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    terminal.on_key(callback.as_ref().unchecked_ref());

    callback.forget();

    let addon = FitAddon::new();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();
    terminal.focus();

    Ok(())
}
