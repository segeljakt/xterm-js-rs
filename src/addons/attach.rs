use crate::{ITerminalAddon, Terminal};
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

#[wasm_bindgen(module = "xterm-addon-attach")]
extern "C" {

    pub type IAttachOptions;

    #[wasm_bindgen(method, setter, js_name = "bidirectional")]
    fn set_bidirectional(this: &IAttachOptions, val: bool);

    // ========================================================================

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type AttachAddon;

    #[wasm_bindgen(constructor)]
    pub fn new(socket: WebSocket, options: Option<IAttachOptions>) -> AttachAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &IAttachOptions, terminal: Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &IAttachOptions);
}
