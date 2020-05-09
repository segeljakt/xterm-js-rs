use crate::{ITerminalAddon, Terminal};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm-addon-search")]
extern "C" {

    pub type ISearchOptions;

    #[wasm_bindgen(method, setter, js_name = "regex")]
    fn set_regex(this: &ISearchOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "wholeWord")]
    fn set_whole_word(this: &ISearchOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "caseSensitive")]
    fn set_case_sensitive(this: &ISearchOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "incremental")]
    fn set_incremental(this: &ISearchOptions, val: bool);

    // ========================================================================

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type SearchAddon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> SearchAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &SearchAddon, terminal: Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &SearchAddon);

    #[wasm_bindgen(method, method, js_name = "findNext")]
    pub fn find_next(
        this: &SearchAddon,
        term: String,
        search_options: Option<ISearchOptions>,
    ) -> bool;

    #[wasm_bindgen(method, method, js_name = "findPrevious")]
    pub fn find_previous(
        this: &SearchAddon,
        term: String,
        search_options: Option<ISearchOptions>,
    ) -> bool;
}
