mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use xterm_js::addons::fit::FitAddon;
use xterm_js::{BellStyle, OnKeyEvent, Terminal, TerminalOptions, Theme};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn prompt(term: &Terminal) {
    term.write("\r\n$ ");
}

// Keyboard keys
// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_UP_ARROW: u32 = 38;
const KEY_DOWN_ARROW: u32 = 40;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_L: u32 = 76;

const CURSOR_UP: &str = "\x1b[A";
const CURSOR_DOWN: &str = "\x1b[B";
const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let terminal: Terminal = Terminal::new(
        TerminalOptions::new()
            .with_rows(50)
            .with_cursor_blink(true)
            .with_cursor_width(10)
            .with_font_size(20)
            .with_draw_bold_text_in_bright_colors(true)
            .with_right_click_selects_word(true)
            .with_theme(
                Theme::new()
                    .with_foreground("#98FB98")
                    .with_background("#000000"),
            ),
    );

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
                    term.writeln("");
                    term.writeln(&format!("You entered {} characters '{}'", line.len(), line));
                    line.clear();
                }
                prompt(&term);
            }
            KEY_BACKSPACE => {
                if !line.is_empty() {
                    term.write("\u{0008} \u{0008}");
                    line.pop();
                }
            }
            KEY_UP_ARROW => term.write(CURSOR_UP),
            KEY_DOWN_ARROW => term.write(CURSOR_DOWN),
            KEY_LEFT_ARROW => term.write(CURSOR_LEFT),
            KEY_RIGHT_ARROW => term.write(CURSOR_RIGHT),
            KEY_L if event.ctrl_key() => term.clear(),
            _ => {
                if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    term.write(&event.key());
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
