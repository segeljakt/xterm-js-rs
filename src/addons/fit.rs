use crate::{ITerminalAddon, Terminal};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm-addon-fit")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type FitAddon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> FitAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &FitAddon, terminal: &Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &FitAddon);

    #[wasm_bindgen(method, method, js_name = "fit")]
    pub fn fit(this: &FitAddon);

    #[wasm_bindgen(method, method, js_name = "proposeDimensions")]
    pub fn propose_dimensions(this: &FitAddon) -> ITerminalDimensions;

    // ========================================================================

    pub type ITerminalDimensions;

    #[wasm_bindgen(method, setter, js_name = "rows")]
    pub fn rows(this: &ITerminalDimensions) -> u32;

    #[wasm_bindgen(method, setter, js_name = "cols")]
    pub fn cols(this: &ITerminalDimensions) -> u32;
}
