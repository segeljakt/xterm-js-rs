use js_sys::{Function, RegExp};
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlCanvasElement, HtmlElement, HtmlTextAreaElement, KeyboardEvent, MouseEvent, WebSocket,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum BellStyle {
    None = "none",
    Sound = "sound",
    Visual = "visual",
    Both = "both",
}

#[wasm_bindgen]
pub enum CursorStyle {
    Block = "block",
    Underline = "underline",
    Bar = "bar",
}

#[wasm_bindgen]
pub enum FastScrollModifier {
    Alt = "alt",
    Ctrl = "ctrl",
    Shift = "shift",
}

#[wasm_bindgen]
pub enum FontWeight {
    Normal = "normal",
    Bold = "bold",
    W100 = "100",
    W200 = "200",
    W300 = "300",
    W400 = "400",
    W500 = "500",
    W600 = "600",
    W700 = "700",
    W800 = "800",
    W900 = "900",
}
#[wasm_bindgen]
pub enum LogLevel {
    Debug = "debug",
    Info = "info",
    Warn = "warn",
    Error = "error",
    Off = "off",
}

#[wasm_bindgen]
pub enum RendererType {
    Dom = "dom",
    Canvas = "canvas",
}

#[wasm_bindgen]
pub enum BufferType {
    Normal = "normal",
    Alternate = "alternate",
}

#[wasm_bindgen]
pub enum WcWidth {
    Width0 = 0,
    Width1 = 1,
    Width2 = 2,
}

#[wasm_bindgen]
pub enum StringOptionKey {
    BellSound = "bellSound",
    BellStyle = "bellStyle",
    CursorStyle = "cursorStyle",
    FontFamily = "fontFamily",
    FontWeight = "fontWeight",
    FontWeightBold = "fontWeightBold",
    LogLevel = "logLevel",
    RendererType = "rendererType",
    TermName = "termName",
    WordSeparator = "wordSeparator",
}

#[wasm_bindgen]
pub enum BoolOptionKey {
    AllowTransparency = "allowTransparency",
    CancelEvents = "cancelEvents",
    ConvertEol = "convertEol",
    CursorBlink = "cursorBlink",
    DisableStdin = "disableStdin",
    MacOptionIsMeta = "macOptionIsMeta",
    RightClickSelectsWord = "rightClickSelectsWord",
    PopOnBell = "popOnBell",
    VisualBell = "visualBell",
    WindowsMode = "windowsMode",
}

#[wasm_bindgen]
pub enum NumberOptionKey {
    Cols = "cols",
    FontSize = "fontSize",
    LetterSpacing = "letterSpacing",
    LineHeight = "lineHeight",
    Rows = "rows",
    TabStopWidth = "tabStopWidth",
    Scrollback = "scrollback",
}

#[wasm_bindgen]
pub enum FontWeightKey {
    FontWeight = "fontWeight",
    FontWeightBold = "fontWeightBold",
}

#[wasm_bindgen]
pub enum LogLevelKey {
    LogLevel = "logLevel",
}

#[wasm_bindgen]
pub enum BellStyleKey {
    BellStyle = "bellStyle",
}

#[wasm_bindgen]
pub enum CursorStyleKey {
    CursorStyle = "cursorStyle",
}

#[wasm_bindgen]
pub enum ThemeKey {
    Theme = "theme",
}

#[wasm_bindgen(module = "xterm")]
extern "C" {

    pub type ITerminalOptions;

    #[wasm_bindgen(method, setter, js_name = "allowTransparency")]
    pub fn set_allow_transparency(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "bellSound")]
    pub fn set_bell_sound(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "bellStyle")]
    pub fn set_bell_style(this: &ITerminalOptions, val: BellStyle);

    #[wasm_bindgen(method, setter, js_name = "convertEol")]
    pub fn set_convert_eol(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "cols")]
    pub fn set_cols(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "cursorBlink")]
    pub fn set_cursor_blink(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "cursorStyle")]
    pub fn set_cursor_style(this: &ITerminalOptions, val: CursorStyle);

    #[wasm_bindgen(method, setter, js_name = "cursorWidth")]
    pub fn set_cursor_width(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "disableStdin")]
    pub fn set_disable_stdin(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "drawBoldTextInBrightColors")]
    pub fn set_draw_bold_text_in_bright_colors(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "fastScrollModifier")]
    pub fn set_fast_scroll_modifier(this: &ITerminalOptions, val: FastScrollModifier);

    #[wasm_bindgen(method, setter, js_name = "fastScrollSensitivity")]
    pub fn set_fast_scroll_sensitivity(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "fontSize")]
    pub fn set_font_size(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "fontFamily")]
    pub fn set_font_family(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "fontWeight")]
    pub fn set_font_weight(this: &ITerminalOptions, val: FontWeight);

    #[wasm_bindgen(method, setter, js_name = "fontWeightBold")]
    pub fn set_font_weight_bold(this: &ITerminalOptions, val: FontWeight);

    #[wasm_bindgen(method, setter, js_name = "letterSpacing")]
    pub fn set_letter_spacing(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "lineHeight")]
    pub fn set_line_height(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "linkTooltipHoverDuration")]
    pub fn set_link_tooltip_hover_duration(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "logLevel")]
    pub fn set_log_level(this: &ITerminalOptions, val: LogLevel);

    #[wasm_bindgen(method, setter, js_name = "macOptionIsMeta")]
    pub fn set_mac_option_is_meta(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "macOptionClickForcesSelection")]
    pub fn set_mac_option_click_forces_selection(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "minimumContrastRatio")]
    pub fn set_minimum_contrast_ratio(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "rendererType")]
    pub fn set_renderer_type(this: &ITerminalOptions, val: RendererType);

    #[wasm_bindgen(method, setter, js_name = "rightClickSelectsWord")]
    pub fn set_right_click_selects_word(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "rows")]
    pub fn set_rows(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "screenReaderMode")]
    pub fn set_screen_reader_mode(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "scrollback")]
    pub fn set_scrollback(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "scrollSensitivity")]
    pub fn set_scroll_sensitivity(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "tabStopWidth")]
    pub fn set_tab_stop_width(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "theme")]
    pub fn set_theme(this: &ITerminalOptions, val: ITheme);

    #[wasm_bindgen(method, setter, js_name = "windowsMode")]
    pub fn set_windows_mode(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "wordSeparator")]
    pub fn set_word_separator(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "windowOptions")]
    pub fn set_window_options(this: &ITerminalOptions, val: IWindowOptions);

    // ========================================================================

    pub type ITheme;

    #[wasm_bindgen(method, setter, js_name = "foreground")]
    pub fn set_foreground(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "background")]
    pub fn set_background(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cursor")]
    pub fn set_cursor(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cursorAccent")]
    pub fn set_cursor_accent(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "selection")]
    pub fn set_selection(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "black")]
    pub fn set_black(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "red")]
    pub fn set_red(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "green")]
    pub fn set_green(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "yellow")]
    pub fn set_yellow(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "blue")]
    pub fn set_blue(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "magenta")]
    pub fn set_magenta(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cyan")]
    pub fn set_cyan(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "white")]
    pub fn set_white(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightBlack")]
    pub fn set_bright_black(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightRed")]
    pub fn set_bright_red(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightGreen")]
    pub fn set_bright_green(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightYellow")]
    pub fn set_bright_yellow(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightBlue")]
    pub fn set_bright_blue(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightMagenta")]
    pub fn set_bright_magenta(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightCyan")]
    pub fn set_bright_cyan(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightWhite")]
    pub fn set_bright_white(this: &ITheme, val: String);

    // ========================================================================

    pub type ILinkMatcherOptions;

    #[wasm_bindgen(method, getter, js_name = "matchIndex")]
    pub fn get_match_index(this: &ILinkMatcherOptions) -> Option<u32>;

    #[wasm_bindgen(method, getter, js_name = "validationCallback")]
    pub fn get_validation_callback(this: &ILinkMatcherOptions) -> Option<Function>; // (uri: String, callback: (is_valid: bool) => void) => void;

    #[wasm_bindgen(method, getter, js_name = "tooltipCallback")]
    pub fn get_tooltip_callback(this: &ILinkMatcherOptions) -> Option<Function>; // (event: MouseEvent, uri: String, location: IViewportRange) => bool | void;

    #[wasm_bindgen(method, getter, js_name = "leaveCallback")]
    pub fn get_leave_callback(this: &ILinkMatcherOptions) -> Option<Function>; // () => void;

    #[wasm_bindgen(method, getter, js_name = "priority")]
    pub fn get_priority(this: &ILinkMatcherOptions) -> Option<u32>;

    #[wasm_bindgen(method, getter, js_name = "willLinkActivate")]
    pub fn get_will_link_activate(this: &ILinkMatcherOptions) -> Option<Function>; // (event: MouseEvent, uri: String) => bool;

    // ========================================================================

    pub type IDisposable;

    #[wasm_bindgen(method, js_name = "dispose")]
    pub fn dispose(this: &IDisposable);

    // ========================================================================

    pub type IEvent;

    //   export interface IEvent<T, U = void> {
    //     (listener: (arg1: T, arg2: U) => any): IDisposable;
    //   }

    // ========================================================================

    #[wasm_bindgen(extends = IDisposable)]
    pub type IMarker;

    #[wasm_bindgen(method, getter, js_name = "id")]
    pub fn get_id(this: &IMarker) -> u32;

    #[wasm_bindgen(method, getter, js_name = "isDisposed")]
    pub fn get_is_disposed(this: &IMarker) -> bool;

    #[wasm_bindgen(method, getter, js_name = "line")]
    pub fn get_line(this: &IMarker) -> u32;

    // ========================================================================

    pub type ILocalizableStrings;

    #[wasm_bindgen(method, setter, js_name = "promptLabel")]
    pub fn set_prompt_label(this: &ILocalizableStrings, val: String);

    #[wasm_bindgen(method, setter, js_name = "tooMuchOutput")]
    pub fn set_too_much_output(this: &ILocalizableStrings, val: String);

    // ========================================================================

    pub type IWindowOptions;

    #[wasm_bindgen(method, setter, js_name = "restoreWin")]
    pub fn set_restore_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "minimizeWin")]
    pub fn set_minimize_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinPosition")]
    pub fn set_win_position(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinSizePixels")]
    pub fn set_win_size_pixels(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "raiseWin")]
    pub fn set_raise_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "lowerWin")]
    pub fn set_lower_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "refreshWin")]
    pub fn set_refresh_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinSizeChars")]
    pub fn set_set_win_size_chars(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "maximizeWin")]
    pub fn set_maximize_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "fullscreenWin")]
    pub fn set_fullscreen_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, getter, js_name = "getWinState")]
    pub fn get_win_state(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinPosition")]
    pub fn get_win_position(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinSizePixels")]
    pub fn get_win_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getScreenSizePixels")]
    pub fn get_screen_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getCellSizePixels")]
    pub fn get_cell_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinSizeChars")]
    pub fn get_win_size_chars(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getScreenSizeChars")]
    pub fn get_screen_size_chars(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getIconTitle")]
    pub fn get_icon_title(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinTitle")]
    pub fn get_win_title(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "pushTitle")]
    pub fn set_push_title(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "popTitle")]
    pub fn set_pop_title(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinLines")]
    pub fn set_win_lines(this: &IWindowOptions, val: bool);

    // ========================================================================

    #[wasm_bindgen(extends = IDisposable)]
    pub type Terminal;

    #[wasm_bindgen(method, getter, js_name = "element")]
    pub fn get_element(this: &Terminal) -> HtmlElement;

    #[wasm_bindgen(method, getter, js_name = "textarea")]
    pub fn get_textarea(this: &Terminal) -> HtmlTextAreaElement;

    #[wasm_bindgen(method, getter, js_name = "rows")]
    pub fn get_rows(this: &Terminal) -> u32;

    #[wasm_bindgen(method, getter, js_name = "cols")]
    pub fn get_cols(this: &Terminal) -> u32;

    #[wasm_bindgen(method, getter, js_name = "buffer")]
    pub fn get_buffer(this: &Terminal) -> IBufferNamespace;

    #[wasm_bindgen(method, getter, js_name = "markers")]
    pub fn get_markers(this: &Terminal) -> Box<[JsValue]>; // Box<[IMarker]>

    #[wasm_bindgen(method, getter, js_name = "parser")]
    pub fn get_parser(this: &Terminal) -> IParser;

    #[wasm_bindgen(method, getter, js_name = "unicode")]
    pub fn get_unicode(this: &Terminal) -> IUnicodeHandling;

    #[wasm_bindgen(static_method_of = Terminal, js_name = "strings")]
    pub fn get_strings() -> ILocalizableStrings;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<ITerminalOptions>) -> Terminal;

    pub type OnKeyEvent;

    #[wasm_bindgen(method, getter, js_name = "key")]
    pub fn key(this: &OnKeyEvent) -> String;

    #[wasm_bindgen(method, getter, js_name = "domEvent")]
    pub fn dom_event(this: &OnKeyEvent) -> KeyboardEvent;

    #[wasm_bindgen(method, js_name = "onBinary")]
    pub fn on_binary(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onCursorMove")]
    pub fn on_cursor_move(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onData")]
    pub fn on_data(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onKey")]
    pub fn on_key(
        this: &Terminal,
        f: &Function, // IEvent<{key: String, dom_event: KeyboardEvent}>
    ); // -> IDisposable

    #[wasm_bindgen(method, js_name = "onLineFeed")]
    pub fn on_line_feed(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onScroll")]
    pub fn on_scroll(
        this: &Terminal,
        f: &Function, // IEvent<u32>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onSelectionChange")]
    pub fn on_selection_change(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onRender")]
    pub fn on_render(
        this: &Terminal,
        f: &Function, // IEvent<{start: u32, end: u32}>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onResize")]
    pub fn on_resize(
        this: &Terminal,
        f: &Function, // IEvent<{cols: u32, rows: u32}>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onTitleChange")]
    pub fn on_title_change(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "blur")]
    pub fn blur(this: &Terminal);

    #[wasm_bindgen(method, js_name = "focus")]
    pub fn focus(this: &Terminal);

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Terminal, columns: u32, rows: u32);

    #[wasm_bindgen(method, js_name = "open")]
    pub fn open(this: &Terminal, parent: HtmlElement);

    #[wasm_bindgen(method, js_name = "attachCustomKeyEventHandler")]
    pub fn attach_custom_key_event_handler(
        this: &Terminal,
        custom_key_event_handler: &Function, // (event: KeyboardEvent) => bool
    );

    #[wasm_bindgen(method, js_name = "registerLinkMatcher")]
    pub fn register_link_matcher(
        this: &Terminal,
        regex: RegExp,
        handler: &Function, // (event: MouseEvent, uri: String) => void
        options: Option<ILinkMatcherOptions>,
    ) -> u32;

    #[wasm_bindgen(method, js_name = "deregisterLinkMatcher")]
    pub fn deregister_link_matcher(this: &Terminal, matcher_id: u32);

    #[wasm_bindgen(method, js_name = "registerLinkProvider")]
    pub fn register_link_provider(this: &Terminal, link_provider: ILinkProvider) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerCharacterJoiner")]
    pub fn register_character_joiner(
        this: &Terminal,
        handler: &Function, // (text: String) => [u32, u32][]
    ) -> u32;

    #[wasm_bindgen(method, js_name = "deregisterCharacterJoiner")]
    pub fn deregister_character_joiner(this: &Terminal, joiner_id: u32);

    #[wasm_bindgen(method, js_name = "registerMarker")]
    pub fn register_marker(this: &Terminal, cursorYOffset: u32) -> Option<IMarker>;

    #[wasm_bindgen(method, js_name = "addMarker")]
    pub fn add_marker(this: &Terminal, cursorYOffset: u32) -> Option<IMarker>;

    #[wasm_bindgen(method, js_name = "hasSelection")]
    pub fn has_selection(this: &Terminal) -> bool;

    #[wasm_bindgen(method, js_name = "getSelection")]
    pub fn get_selection(this: &Terminal) -> String;

    #[wasm_bindgen(method, js_name = "getSelectionPosition")]
    pub fn get_selection_position(this: &Terminal) -> Option<ISelectionPosition>;

    #[wasm_bindgen(method, js_name = "clearSelection")]
    pub fn clear_selection(this: &Terminal);

    #[wasm_bindgen(method, js_name = "select")]
    pub fn select(this: &Terminal, column: u32, row: u32, length: u32);

    #[wasm_bindgen(method, js_name = "selectAll")]
    pub fn select_all(this: &Terminal);

    #[wasm_bindgen(method, js_name = "selectLines")]
    pub fn select_lines(this: &Terminal, start: u32, end: u32);

    #[wasm_bindgen(method, js_name = "dispose")]
    pub fn dispose(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollLines")]
    pub fn scroll_lines(this: &Terminal, amount: u32);

    #[wasm_bindgen(method, js_name = "scrollPages")]
    pub fn scroll_pages(this: &Terminal, page_count: u32);

    #[wasm_bindgen(method, js_name = "scrollToTop")]
    pub fn scroll_to_top(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollToBottom")]
    pub fn scroll_to_bottom(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollToLine")]
    pub fn scroll_to_line(this: &Terminal, line: u32);

    #[wasm_bindgen(method, js_name = "clear")]
    pub fn clear(this: &Terminal);

    #[wasm_bindgen(method, js_name = "write")]
    pub fn write(
        this: &Terminal,
        data: String,                // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "writeln")]
    pub fn writeln(
        this: &Terminal,
        data: String,                // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "writeUtf")]
    pub fn write_utf8(
        this: &Terminal,
        data: Box<[u8]>,             // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "paste")]
    pub fn paste(this: &Terminal, data: String);

    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_string_option(this: &Terminal, key: StringOptionKey) -> String;

    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_bool_option(this: &Terminal, key: BoolOptionKey) -> bool;

    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_number_option(this: &Terminal, key: NumberOptionKey) -> u32;

    #[wasm_bindgen(method, js_name = "getOption")]
    pub fn get_any_option(this: &Terminal, key: String) -> JsValue;

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_string_option(this: &Terminal, key: StringOptionKey, value: String);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_font_option(this: &Terminal, key: FontWeightKey, value: FontWeight);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_log_level_option(this: &Terminal, key: LogLevelKey, value: LogLevel);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_bell_style_option(this: &Terminal, key: BellStyleKey, value: BellStyle);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_cursor_option(this: &Terminal, key: CursorStyleKey, value: CursorStyle);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_bool_option(this: &Terminal, key: BoolOptionKey, value: bool);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_number_option(this: &Terminal, key: NumberOptionKey, value: u32);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_theme_option(this: &Terminal, key: ThemeKey, value: ITheme);

    #[wasm_bindgen(method, js_name = "setOption")]
    pub fn set_any_option(this: &Terminal, key: String, value: JsValue);

    #[wasm_bindgen(method, js_name = "refresh")]
    pub fn refresh(this: &Terminal, start: u32, end: u32);

    #[wasm_bindgen(method, js_name = "reset")]
    pub fn reset(this: &Terminal);

    #[wasm_bindgen(method, js_name = "loadAddon")]
    pub fn load_addon(this: &Terminal, addon: ITerminalAddon);

    // ========================================================================

    #[wasm_bindgen(extends = IDisposable)]
    pub type ITerminalAddon;

    #[wasm_bindgen(method)]
    pub fn activate(this: &ITerminalAddon, terminal: Terminal);

    pub type ISelectionPosition;

    #[wasm_bindgen(method, getter, js_name = "startColumn")]
    pub fn start_column(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "startRow")]
    pub fn start_row(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "endColumn")]
    pub fn end_column(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "endRow")]
    pub fn end_row(this: &ISelectionPosition) -> u32;

    // ========================================================================

    pub type IViewportRange;

    #[wasm_bindgen(method, setter, js_name = "start")]
    pub fn set_start(this: &IViewportRange, val: IViewportRangePosition);

    #[wasm_bindgen(method, setter, js_name = "end")]
    pub fn set_end(this: &IViewportRange, val: IViewportRangePosition);

    // ========================================================================

    pub type IViewportRangePosition;

    #[wasm_bindgen(method, setter, js_name = "x")]
    pub fn set_x(this: &IViewportRangePosition, val: u32);

    #[wasm_bindgen(method, setter, js_name = "y")]
    pub fn set_y(this: &IViewportRangePosition, val: u32);

    // ========================================================================

    pub type ILinkProvider;

    #[wasm_bindgen(method, js_name = "provideLinks")]
    pub fn provide_links(
        this: &ILinkProvider,
        buffer_lineu32: u32,
        callback: &Function, // (links: ILink[] | undefined) => void
    );

    // ========================================================================

    pub type ILink;

    #[wasm_bindgen(method, setter, js_name = "range")]
    pub fn set_range(this: &ILink, val: IBufferRange);

    #[wasm_bindgen(method, setter, js_name = "text")]
    pub fn set_text(this: &ILink, val: String);

    #[wasm_bindgen(method, setter, js_name = "decorations")]
    pub fn set_decorations(this: &ILink, val: ILinkDecorations);

    #[wasm_bindgen(method, js_name = "activate")]
    pub fn activate(this: &ILink, event: MouseEvent, text: String);

    #[wasm_bindgen(method, js_name = "hover")]
    pub fn hover(this: &ILink, event: MouseEvent, text: String);

    #[wasm_bindgen(method, js_name = "leave")]
    pub fn leave(this: &ILink, event: MouseEvent, text: String);

    // ========================================================================

    pub type ILinkDecorations;

    #[wasm_bindgen(method, setter, js_name = "pointerCursor")]
    pub fn set_pointer_cursor(this: &IBufferNamespace, val: bool);

    #[wasm_bindgen(method, setter, js_name = "underline")]
    pub fn set_underline(this: &IBufferNamespace, val: bool);

    // ========================================================================

    pub type IBufferRange;

    #[wasm_bindgen(method, getter, js_name = "start")]
    pub fn get_start(this: &IBufferRange) -> IBufferCellPosition;

    #[wasm_bindgen(method, getter, js_name = "end")]
    pub fn get_end(this: &IBufferRange) -> IBufferCellPosition;

    // ========================================================================

    pub type IBufferCellPosition;

    #[wasm_bindgen(method, getter, js_name = "x")]
    pub fn get_x(this: &IBufferCellPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "y")]
    pub fn get_y(this: &IBufferCellPosition) -> u32;

    // ========================================================================

    pub type IBuffer;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn get_type(this: &IBuffer) -> BufferType;

    #[wasm_bindgen(method, getter, js_name = "cursorY")]
    pub fn get_cursor_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "cursorX")]
    pub fn get_cursor_x(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "viewportY")]
    pub fn get_viewport_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "baseY")]
    pub fn get_base_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "length")]
    pub fn get_length(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, js_name = "getLine")]
    pub fn get_line(this: &IBuffer, y: u32) -> IBufferLine;

    #[wasm_bindgen(method, js_name = "getNullCell")]
    pub fn get_null_cell(this: &IBuffer) -> IBufferCell;

    // ========================================================================

    pub type IBufferNamespace;

    #[wasm_bindgen(method, getter, js_name = "active")]
    pub fn get_active(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, getter, js_name = "normal")]
    pub fn get_normal(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, getter, js_name = "alternate")]
    pub fn get_alternate(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, setter, js_name = "onBufferChange")]
    pub fn set_on_buffer_change(this: &IBufferNamespace, val: Function);

    // ========================================================================

    pub type IBufferLine;

    #[wasm_bindgen(method, getter, js_name = "isWrapped")]
    pub fn is_wrapped(this: &IBufferLine) -> bool;

    #[wasm_bindgen(method, getter, js_name = "length")]
    pub fn get_length(this: &IBufferLine) -> u32;

    #[wasm_bindgen(method, js_name = "getCell")]
    pub fn get_cell(this: &IBufferLine, x: u32, cell: Option<IBufferCell>) -> Option<IBufferCell>;

    #[wasm_bindgen(method, js_name = "translateToString")]
    pub fn translate_to_String(
        this: &IBufferLine,
        trim_right: bool,
        start_column: Option<u32>,
        end_column: Option<u32>,
    ) -> String;

    // ========================================================================

    pub type IBufferCell;

    #[wasm_bindgen(method, js_name = "getWidth")]
    pub fn get_width(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getChars")]
    pub fn get_chars(this: &IBufferCell) -> String;

    #[wasm_bindgen(method, js_name = "getCode")]
    pub fn get_code(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getFgColorMode")]
    pub fn get_fg_color_mode(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getBgColorMode")]
    pub fn get_bg_color_mode(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getFgColor")]
    pub fn get_fg_color(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getBgColor")]
    pub fn bg_color(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isBold")]
    pub fn is_bold(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isItalic")]
    pub fn is_italic(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isDim")]
    pub fn is_dim(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isUnderline")]
    pub fn is_underline(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isBlink")]
    pub fn is_blink(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isInverse")]
    pub fn is_inverse(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isInvisible")]
    pub fn is_invisible(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isFgRGB")]
    pub fn is_fg_rgb(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgRGB")]
    pub fn is_bg_rgb(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isFgPalette")]
    pub fn is_fg_palette(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgPalette")]
    pub fn is_bg_palette(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isFgDefault")]
    pub fn is_fg_default(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgDefault")]
    pub fn is_bg_default(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isAttributeDefault")]
    pub fn is_attribute_default(this: &IBufferCell) -> bool;

    // ========================================================================

    pub type IFunctionIdentifier;

    #[wasm_bindgen(method, setter, js_name = "prefix")]
    pub fn set_prefix(this: &IFunctionIdentifier, val: String);

    #[wasm_bindgen(method, setter, js_name = "intermediates")]
    pub fn set_intermediates(this: &IFunctionIdentifier, val: String);

    #[wasm_bindgen(method, setter, js_name = "final")]
    pub fn set_final(this: &IFunctionIdentifier, val: String);

    // ========================================================================

    pub type IParser;

    #[wasm_bindgen(method, js_name = "registerCsiHandler")]
    pub fn register_csi_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        callback: &Function, // (params: (u32 | u32[])[]) => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerDcsHandler")]
    pub fn register_dcs_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        callback: &Function, // (data: String, param: (u32 | u32[])[]) => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerEscHandler")]
    pub fn register_esc_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        handler: &Function, // () => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerOscHandler")]
    pub fn register_osc_handler(
        this: &IFunctionIdentifier,
        ident: u32,
        callback: &Function, // (data: String) => bool
    ) -> IDisposable;

    // ========================================================================

    pub type IUnicodeVersionProvider;

    #[wasm_bindgen(method, getter, js_name = "version")]
    pub fn get_version(this: &IUnicodeVersionProvider) -> String;

    #[wasm_bindgen(method, js_name = "wcwidth")]
    pub fn wcwidth(this: &IViewportRangePosition, codepoint: u32) -> WcWidth;

    // ========================================================================

    pub type IUnicodeHandling;

    #[wasm_bindgen(method, js_name = "register")]
    pub fn register(this: &IUnicodeHandling, provider: IUnicodeVersionProvider);

    #[wasm_bindgen(method, getter, js_name = "versions")]
    pub fn get_versions(this: &IUnicodeHandling) -> Box<[JsValue]>; // Box<[String]>

    #[wasm_bindgen(method, getter, js_name = "activeVersion")]
    pub fn get_active_version(this: &IUnicodeHandling) -> String;

}

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

#[wasm_bindgen(module = "xterm-addon-web-links")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type WebLinksAddon;

    #[wasm_bindgen(constructor)]
    pub fn new(
        handler: Option<&Function>, // (event: MouseEvent, uri: string) => void
        options: Option<&ILinkMatcherOptions>,
        useLinkProvider: Option<bool>,
    ) -> WebLinksAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &WebLinksAddon, terminal: &Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &WebLinksAddon);
}

#[wasm_bindgen(module = "xterm-addon-ligatures")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type LigaturesAddon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> LigaturesAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &LigaturesAddon, terminal: &Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &LigaturesAddon);

}

#[wasm_bindgen(module = "xterm-addon-serialize")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type SerializeAddon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> SerializeAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &SerializeAddon, terminal: Terminal);

    #[wasm_bindgen(method, method, js_name = "serialize")]
    pub fn serialize(this: &SerializeAddon, rows: Option<u32>) -> String;

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &SerializeAddon);

}

#[wasm_bindgen(module = "xterm-addon-unicode11")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type Unicode11Addon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Unicode11Addon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &Unicode11Addon, terminal: Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &Unicode11Addon);

}

#[wasm_bindgen(module = "xterm-addon-webgl")]
extern "C" {

    #[wasm_bindgen(extends = ITerminalAddon)]
    pub type WebglAddon;

    #[wasm_bindgen(method, setter, js_name = "textureAtlas")]
    pub fn set_texture_atlas(this: &WebglAddon, val: HtmlCanvasElement);

    #[wasm_bindgen(constructor)]
    pub fn new(preserve_drawing_buffer: Option<bool>) -> WebglAddon;

    #[wasm_bindgen(method, method, js_name = "activate")]
    pub fn activate(this: &WebglAddon, terminal: Terminal);

    #[wasm_bindgen(method, method, js_name = "dispose")]
    pub fn dispose(this: &WebglAddon);

}
