use js_sys::{Function, RegExp};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlTextAreaElement, MouseEvent};

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

    type ITerminalOptions;

    #[wasm_bindgen(method, setter, js_name = "allowTransparency")]
    fn set_allow_transparency(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "bellSound")]
    fn set_bell_sound(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "bellStyle")]
    fn set_bell_style(this: &ITerminalOptions, val: BellStyle);

    #[wasm_bindgen(method, setter, js_name = "convertEol")]
    fn set_convert_eol(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "cols")]
    fn set_cols(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "cursorBlink")]
    fn set_cursor_blink(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "cursorStyle")]
    fn set_cursor_style(this: &ITerminalOptions, val: CursorStyle);

    #[wasm_bindgen(method, setter, js_name = "cursorWidth")]
    fn set_cursor_width(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "disableStdin")]
    fn set_disable_stdin(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "drawBoldTextInBrightColors")]
    fn set_draw_bold_text_in_bright_colors(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "fastScrollModifier")]
    fn set_fast_scroll_modifier(this: &ITerminalOptions, val: FastScrollModifier);

    #[wasm_bindgen(method, setter, js_name = "fastScrollSensitivity")]
    fn set_fast_scroll_sensitivity(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "fontSize")]
    fn set_font_size(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "fontFamily")]
    fn set_font_family(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "fontWeight")]
    fn set_font_weight(this: &ITerminalOptions, val: FontWeight);

    #[wasm_bindgen(method, setter, js_name = "fontWeightBold")]
    fn set_font_weight_bold(this: &ITerminalOptions, val: FontWeight);

    #[wasm_bindgen(method, setter, js_name = "letterSpacing")]
    fn set_letter_spacing(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "lineHeight")]
    fn set_line_height(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "linkTooltipHoverDuration")]
    fn set_link_tooltip_hover_duration(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "logLevel")]
    fn set_log_level(this: &ITerminalOptions, val: LogLevel);

    #[wasm_bindgen(method, setter, js_name = "macOptionIsMeta")]
    fn set_mac_option_is_meta(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "macOptionClickForcesSelection")]
    fn set_mac_option_click_forces_selection(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "minimumContrastRatio")]
    fn set_minimum_contrast_ratio(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "rendererType")]
    fn set_renderer_type(this: &ITerminalOptions, val: RendererType);

    #[wasm_bindgen(method, setter, js_name = "rightClickSelectsWord")]
    fn set_right_click_selects_word(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "rows")]
    fn set_rows(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "screenReaderMode")]
    fn set_screen_reader_mode(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "scrollback")]
    fn set_scrollback(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "scrollSensitivity")]
    fn set_scroll_sensitivity(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "tabStopWidth")]
    fn set_tab_stop_width(this: &ITerminalOptions, val: u32);

    #[wasm_bindgen(method, setter, js_name = "theme")]
    fn set_theme(this: &ITerminalOptions, val: ITheme);

    #[wasm_bindgen(method, setter, js_name = "windowsMode")]
    fn set_windows_mode(this: &ITerminalOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "wordSeparator")]
    fn set_word_separator(this: &ITerminalOptions, val: String);

    #[wasm_bindgen(method, setter, js_name = "windowOptions")]
    fn set_window_options(this: &ITerminalOptions, val: IWindowOptions);

    // ========================================================================

    type ITheme;

    #[wasm_bindgen(method, setter, js_name = "foreground")]
    fn set_foreground(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "background")]
    fn set_background(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cursor")]
    fn set_cursor(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cursorAccent")]
    fn set_cursor_accent(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "selection")]
    fn set_selection(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "black")]
    fn set_black(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "red")]
    fn set_red(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "green")]
    fn set_green(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "yellow")]
    fn set_yellow(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "blue")]
    fn set_blue(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "magenta")]
    fn set_magenta(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "cyan")]
    fn set_cyan(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "white")]
    fn set_white(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightBlack")]
    fn set_bright_black(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightRed")]
    fn set_bright_red(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightGreen")]
    fn set_bright_green(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightYellow")]
    fn set_bright_yellow(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightBlue")]
    fn set_bright_blue(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightMagenta")]
    fn set_bright_magenta(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightCyan")]
    fn set_bright_cyan(this: &ITheme, val: String);

    #[wasm_bindgen(method, setter, js_name = "brightWhite")]
    fn set_bright_white(this: &ITheme, val: String);

    // ========================================================================

    type ILinkMatcherOptions;

    #[wasm_bindgen(method, getter, js_name = "matchIndex")]
    fn get_match_index(this: &ILinkMatcherOptions) -> Option<u32>;

    #[wasm_bindgen(method, getter, js_name = "validationCallback")]
    fn get_validation_callback(this: &ILinkMatcherOptions) -> Option<Function>; // (uri: String, callback: (is_valid: bool) => void) => void;

    #[wasm_bindgen(method, getter, js_name = "tooltipCallback")]
    fn get_tooltip_callback(this: &ILinkMatcherOptions) -> Option<Function>; // (event: MouseEvent, uri: String, location: IViewportRange) => bool | void;

    #[wasm_bindgen(method, getter, js_name = "leaveCallback")]
    fn get_leave_callback(this: &ILinkMatcherOptions) -> Option<Function>; // () => void;

    #[wasm_bindgen(method, getter, js_name = "priority")]
    fn get_priority(this: &ILinkMatcherOptions) -> Option<u32>;

    #[wasm_bindgen(method, getter, js_name = "willLinkActivate")]
    fn get_will_link_activate(this: &ILinkMatcherOptions) -> Option<Function>; // (event: MouseEvent, uri: String) => bool;

    // ========================================================================

    type IDisposable;

    #[wasm_bindgen(method, js_name = "dispose")]
    fn dispose(this: &IDisposable);

    // ========================================================================

    type IEvent;

    //   export interface IEvent<T, U = void> {
    //     (listener: (arg1: T, arg2: U) => any): IDisposable;
    //   }

    // ========================================================================

    type IMarker;

    #[wasm_bindgen(method, getter, js_name = "id")]
    fn get_id(this: &IMarker) -> u32;

    #[wasm_bindgen(method, getter, js_name = "isDisposed")]
    fn get_is_disposed(this: &IMarker) -> bool;

    #[wasm_bindgen(method, getter, js_name = "line")]
    fn get_line(this: &IMarker) -> u32;

    // ========================================================================

    type ILocalizableStrings;

    #[wasm_bindgen(method, setter, js_name = "promptLabel")]
    fn set_prompt_label(this: &ILocalizableStrings, val: String);

    #[wasm_bindgen(method, setter, js_name = "tooMuchOutput")]
    fn set_too_much_output(this: &ILocalizableStrings, val: String);

    // ========================================================================

    type IWindowOptions;

    #[wasm_bindgen(method, setter, js_name = "restoreWin")]
    fn set_restore_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "minimizeWin")]
    fn set_minimize_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinPosition")]
    fn set_win_position(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinSizePixels")]
    fn set_win_size_pixels(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "raiseWin")]
    fn set_raise_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "lowerWin")]
    fn set_lower_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "refreshWin")]
    fn set_refresh_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinSizeChars")]
    fn set_set_win_size_chars(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "maximizeWin")]
    fn set_maximize_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "fullscreenWin")]
    fn set_fullscreen_win(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, getter, js_name = "getWinState")]
    fn get_win_state(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinPosition")]
    fn get_win_position(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinSizePixels")]
    fn get_win_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getScreenSizePixels")]
    fn get_screen_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getCellSizePixels")]
    fn get_cell_size_pixels(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinSizeChars")]
    fn get_win_size_chars(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getScreenSizeChars")]
    fn get_screen_size_chars(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getIconTitle")]
    fn get_icon_title(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = "getWinTitle")]
    fn get_win_title(this: &IWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "pushTitle")]
    fn set_push_title(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "popTitle")]
    fn set_pop_title(this: &IWindowOptions, val: bool);

    #[wasm_bindgen(method, setter, js_name = "setWinLines")]
    fn set_win_lines(this: &IWindowOptions, val: bool);

    // ========================================================================

    type Terminal;

    #[wasm_bindgen(method, getter, js_name = "element")]
    fn get_element(this: &Terminal) -> HtmlElement;

    #[wasm_bindgen(method, getter, js_name = "textarea")]
    fn get_textarea(this: &Terminal) -> HtmlTextAreaElement;

    #[wasm_bindgen(method, getter, js_name = "rows")]
    fn get_rows(this: &Terminal) -> u32;

    #[wasm_bindgen(method, getter, js_name = "cols")]
    fn get_cols(this: &Terminal) -> u32;

    #[wasm_bindgen(method, getter, js_name = "buffer")]
    fn get_buffer(this: &Terminal) -> IBufferNamespace;

    #[wasm_bindgen(method, getter, js_name = "markers")]
    fn get_markers(this: &Terminal) -> Box<[JsValue]>; // Box<[IMarker]>

    #[wasm_bindgen(method, getter, js_name = "parser")]
    fn get_parser(this: &Terminal) -> IParser;

    #[wasm_bindgen(method, getter, js_name = "unicode")]
    fn get_unicode(this: &Terminal) -> IUnicodeHandling;

    #[wasm_bindgen(static_method_of = Terminal, js_name = "strings")]
    fn get_strings() -> ILocalizableStrings;

    // Getter:
    //  '<,'>s/: \(.*\);/(this: \&I) -> \1;/
    //  '<,'>s/\(\s*\)\([a-zA-Z]\+\)/\1#[wasm_bindgen(method, getter, js_name = "\2")]\r\1fn get_\2/
    // Setter:
    //  '<,'>s/: \(.*\);/(this: \&I, val: \1);/
    //  '<,'>s/\(\s*\)\([a-zA-Z]\+\)/\1#[wasm_bindgen(method, setter, js_name = "\2")]\r\1fn set_\2/
    // Method:
    //  '<,'>s/(\(.*\)): \(.*\);/(this: \&I, \1) ->\2;/
    //  '<,'>s/\(\s*\)\([a-zA-Z]\+\)/\1#[wasm_bindgen(method, js_name = "\2")]\r\1fn set_\2/

    #[wasm_bindgen(constructor)]
    fn new(options: Option<ITerminalOptions>) -> Terminal;

    #[wasm_bindgen(method, js_name = "onBinary")]
    fn on_binary(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onCursorMove")]
    fn on_cursor_move(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onData")]
    fn on_data(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onKey")]
    fn on_key(
        this: &Terminal,
        f: &Function, // IEvent<{key: String, dom_event: KeyboardEvent}>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onLineFeed")]
    fn on_line_feed(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onScroll")]
    fn on_scroll(
        this: &Terminal,
        f: &Function, // IEvent<u32>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onSelectionChange")]
    fn on_selection_change(
        this: &Terminal,
        f: &Function, // IEvent<void>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onRender")]
    fn on_render(
        this: &Terminal,
        f: &Function, // IEvent<{start: u32, end: u32}>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onResize")]
    fn on_resize(
        this: &Terminal,
        f: &Function, // IEvent<{cols: u32, rows: u32}>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "onTitleChange")]
    fn on_title_change(
        this: &Terminal,
        f: &Function, // IEvent<String>
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "blur")]
    fn blur(this: &Terminal);

    #[wasm_bindgen(method, js_name = "focus")]
    fn focus(this: &Terminal);

    #[wasm_bindgen(method, js_name = "resize")]
    fn resize(this: &Terminal, columns: u32, rows: u32);

    #[wasm_bindgen(method, js_name = "open")]
    fn open(this: &Terminal, parent: HtmlElement);

    #[wasm_bindgen(method, js_name = "attachCustomKeyEventHandler")]
    fn attach_custom_key_event_handler(
        this: &Terminal,
        custom_key_event_handler: &Function, // (event: KeyboardEvent) => bool
    );

    #[wasm_bindgen(method, js_name = "registerLinkMatcher")]
    fn register_link_matcher(
        this: &Terminal,
        regex: RegExp,
        handler: &Function, // (event: MouseEvent, uri: String) => void
        options: Option<ILinkMatcherOptions>,
    ) -> u32;

    #[wasm_bindgen(method, js_name = "deregisterLinkMatcher")]
    fn deregister_link_matcher(this: &Terminal, matcher_id: u32);

    #[wasm_bindgen(method, js_name = "registerLinkProvider")]
    fn register_link_provider(this: &Terminal, link_provider: ILinkProvider) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerCharacterJoiner")]
    fn register_character_joiner(
        this: &Terminal,
        handler: &Function, // (text: String) => [u32, u32][]
    ) -> u32;

    #[wasm_bindgen(method, js_name = "deregisterCharacterJoiner")]
    fn deregister_character_joiner(this: &Terminal, joiner_id: u32);

    #[wasm_bindgen(method, js_name = "registerMarker")]
    fn register_marker(this: &Terminal, cursorYOffset: u32) -> Option<IMarker>;

    #[wasm_bindgen(method, js_name = "addMarker")]
    fn add_marker(this: &Terminal, cursorYOffset: u32) -> Option<IMarker>;

    #[wasm_bindgen(method, js_name = "hasSelection")]
    fn has_selection(this: &Terminal) -> bool;

    #[wasm_bindgen(method, js_name = "getSelection")]
    fn get_selection(this: &Terminal) -> String;

    #[wasm_bindgen(method, js_name = "getSelectionPosition")]
    fn get_selection_position(this: &Terminal) -> Option<ISelectionPosition>;

    #[wasm_bindgen(method, js_name = "clearSelection")]
    fn clear_selection(this: &Terminal);

    #[wasm_bindgen(method, js_name = "select")]
    fn select(this: &Terminal, column: u32, row: u32, length: u32);

    #[wasm_bindgen(method, js_name = "selectAll")]
    fn select_all(this: &Terminal);

    #[wasm_bindgen(method, js_name = "selectLines")]
    fn select_lines(this: &Terminal, start: u32, end: u32);

    #[wasm_bindgen(method, js_name = "dispose")]
    fn dispose(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollLines")]
    fn scroll_lines(this: &Terminal, amount: u32);

    #[wasm_bindgen(method, js_name = "scrollPages")]
    fn scroll_pages(this: &Terminal, page_count: u32);

    #[wasm_bindgen(method, js_name = "scrollToTop")]
    fn scroll_to_top(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollToBottom")]
    fn scroll_to_bottom(this: &Terminal);

    #[wasm_bindgen(method, js_name = "scrollToLine")]
    fn scroll_to_line(this: &Terminal, line: u32);

    #[wasm_bindgen(method, js_name = "clear")]
    fn clear(this: &Terminal);

    #[wasm_bindgen(method, js_name = "write")]
    fn write(
        this: &Terminal,
        data: String,                // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "writeln")]
    fn writeln(
        this: &Terminal,
        data: String,                // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "writeUtf")]
    fn write_utf8(
        this: &Terminal,
        data: Box<[u8]>,             // String | uint8array
        callback: Option<&Function>, // () => void
    );

    #[wasm_bindgen(method, js_name = "paste")]
    fn paste(this: &Terminal, data: String);

    #[wasm_bindgen(method, js_name = "getOption")]
    fn get_string_option(this: &Terminal, key: StringOptionKey) -> String;

    #[wasm_bindgen(method, js_name = "getOption")]
    fn get_bool_option(this: &Terminal, key: BoolOptionKey) -> bool;

    #[wasm_bindgen(method, js_name = "getOption")]
    fn get_number_option(this: &Terminal, key: NumberOptionKey) -> u32;

    #[wasm_bindgen(method, js_name = "getOption")]
    fn get_any_option(this: &Terminal, key: String) -> JsValue;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_string_option(this: &Terminal, key: StringOptionKey, value: String);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_font_option(this: &Terminal, key: FontWeightKey, value: FontWeight);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_log_level_option(this: &Terminal, key: LogLevelKey, value: LogLevel);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_bell_style_option(this: &Terminal, key: BellStyleKey, value: BellStyle);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_cursor_option(this: &Terminal, key: CursorStyleKey, value: CursorStyle);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_bool_option(this: &Terminal, key: BoolOptionKey, value: bool);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_number_option(this: &Terminal, key: NumberOptionKey, value: u32);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_theme_option(this: &Terminal, key: ThemeKey, value: ITheme);

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_any_option(this: &Terminal, key: String, value: JsValue);

    #[wasm_bindgen(method, js_name = "refresh")]
    fn refresh(this: &Terminal, start: u32, end: u32);

    #[wasm_bindgen(method, js_name = "reset")]
    fn reset(this: &Terminal);

    #[wasm_bindgen(method, js_name = "loadAddon")]
    fn load_addon(this: &Terminal, addon: ITerminalAddon);

    // ========================================================================

    type ITerminalAddon;

    #[wasm_bindgen(method)]
    fn activate(this: &ITerminalAddon, terminal: Terminal);

    type ISelectionPosition;

    #[wasm_bindgen(method, getter, js_name = "startColumn")]
    fn start_column(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "startRow")]
    fn start_row(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "endColumn")]
    fn end_column(this: &ISelectionPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "endRow")]
    fn end_row(this: &ISelectionPosition) -> u32;

    // ========================================================================

    type IViewportRange;

    #[wasm_bindgen(method, setter, js_name = "start")]
    fn set_start(this: &IViewportRange, val: IViewportRangePosition);

    #[wasm_bindgen(method, setter, js_name = "end")]
    fn set_end(this: &IViewportRange, val: IViewportRangePosition);

    // ========================================================================

    type IViewportRangePosition;

    #[wasm_bindgen(method, setter, js_name = "x")]
    fn set_x(this: &IViewportRangePosition, val: u32);

    #[wasm_bindgen(method, setter, js_name = "y")]
    fn set_y(this: &IViewportRangePosition, val: u32);

    // ========================================================================

    type ILinkProvider;

    #[wasm_bindgen(method, js_name = "provideLinks")]
    fn provide_links(
        this: &ILinkProvider,
        buffer_lineu32: u32,
        callback: &Function, // (links: ILink[] | undefined) => void
    );

    // ========================================================================

    type ILink;

    #[wasm_bindgen(method, setter, js_name = "range")]
    fn set_range(this: &ILink, val: IBufferRange);

    #[wasm_bindgen(method, setter, js_name = "text")]
    fn set_text(this: &ILink, val: String);

    #[wasm_bindgen(method, setter, js_name = "decorations")]
    fn set_decorations(this: &ILink, val: ILinkDecorations);

    #[wasm_bindgen(method, js_name = "activate")]
    fn activate(this: &ILink, event: MouseEvent, text: String);

    #[wasm_bindgen(method, js_name = "hover")]
    fn hover(this: &ILink, event: MouseEvent, text: String);

    #[wasm_bindgen(method, js_name = "leave")]
    fn leave(this: &ILink, event: MouseEvent, text: String);

    // ========================================================================

    type ILinkDecorations;

    #[wasm_bindgen(method, setter, js_name = "pointerCursor")]
    fn set_pointer_cursor(this: &IBufferNamespace, val: bool);

    #[wasm_bindgen(method, setter, js_name = "underline")]
    fn set_underline(this: &IBufferNamespace, val: bool);

    // ========================================================================

    type IBufferRange;

    #[wasm_bindgen(method, getter, js_name = "start")]
    fn get_start(this: &IBufferRange) -> IBufferCellPosition;

    #[wasm_bindgen(method, getter, js_name = "end")]
    fn get_end(this: &IBufferRange) -> IBufferCellPosition;

    // ========================================================================

    type IBufferCellPosition;

    #[wasm_bindgen(method, getter, js_name = "x")]
    fn get_x(this: &IBufferCellPosition) -> u32;

    #[wasm_bindgen(method, getter, js_name = "y")]
    fn get_y(this: &IBufferCellPosition) -> u32;

    // ========================================================================

    type IBuffer;

    #[wasm_bindgen(method, getter, js_name = "type")]
    fn get_type(this: &IBuffer) -> BufferType;

    #[wasm_bindgen(method, getter, js_name = "cursorY")]
    fn get_cursor_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "cursorX")]
    fn get_cursor_x(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "viewportY")]
    fn get_viewport_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "baseY")]
    fn get_base_y(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, getter, js_name = "length")]
    fn get_length(this: &IBuffer) -> u32;

    #[wasm_bindgen(method, js_name = "getLine")]
    fn get_line(this: &IBuffer, y: u32) -> IBufferLine;

    #[wasm_bindgen(method, js_name = "getNullCell")]
    fn get_null_cell(this: &IBuffer) -> IBufferCell;

    // ========================================================================

    type IBufferNamespace;

    #[wasm_bindgen(method, getter, js_name = "active")]
    fn get_active(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, getter, js_name = "normal")]
    fn get_normal(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, getter, js_name = "alternate")]
    fn get_alternate(this: &IBufferNamespace) -> IBuffer;

    #[wasm_bindgen(method, setter, js_name = "onBufferChange")]
    fn set_on_buffer_change(this: &IBufferNamespace, val: Function);

    // ========================================================================

    type IBufferLine;

    #[wasm_bindgen(method, getter, js_name = "isWrapped")]
    fn is_wrapped(this: &IBufferLine) -> bool;

    #[wasm_bindgen(method, getter, js_name = "length")]
    fn get_length(this: &IBufferLine) -> u32;

    #[wasm_bindgen(method, js_name = "getCell")]
    fn get_cell(this: &IBufferLine, x: u32, cell: Option<IBufferCell>) -> Option<IBufferCell>;

    #[wasm_bindgen(method, js_name = "translateToString")]
    fn translate_to_String(
        this: &IBufferLine,
        trim_right: bool,
        start_column: Option<u32>,
        end_column: Option<u32>,
    ) -> String;

    // ========================================================================

    type IBufferCell;

    #[wasm_bindgen(method, js_name = "getWidth")]
    fn get_width(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getChars")]
    fn get_chars(this: &IBufferCell) -> String;

    #[wasm_bindgen(method, js_name = "getCode")]
    fn get_code(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getFgColorMode")]
    fn get_fg_color_mode(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getBgColorMode")]
    fn get_bg_color_mode(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getFgColor")]
    fn get_fg_color(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "getBgColor")]
    fn bg_color(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isBold")]
    fn is_bold(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isItalic")]
    fn is_italic(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isDim")]
    fn is_dim(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isUnderline")]
    fn is_underline(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isBlink")]
    fn is_blink(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isInverse")]
    fn is_inverse(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isInvisible")]
    fn is_invisible(this: &IBufferCell) -> u32;

    #[wasm_bindgen(method, js_name = "isFgRGB")]
    fn is_fg_rgb(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgRGB")]
    fn is_bg_rgb(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isFgPalette")]
    fn is_fg_palette(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgPalette")]
    fn is_bg_palette(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isFgDefault")]
    fn is_fg_default(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isBgDefault")]
    fn is_bg_default(this: &IBufferCell) -> bool;

    #[wasm_bindgen(method, js_name = "isAttributeDefault")]
    fn is_attribute_default(this: &IBufferCell) -> bool;

    // ========================================================================

    type IFunctionIdentifier;

    #[wasm_bindgen(method, setter, js_name = "prefix")]
    fn set_prefix(this: &IFunctionIdentifier, val: String);

    #[wasm_bindgen(method, setter, js_name = "intermediates")]
    fn set_intermediates(this: &IFunctionIdentifier, val: String);

    #[wasm_bindgen(method, setter, js_name = "final")]
    fn set_final(this: &IFunctionIdentifier, val: String);

    // ========================================================================

    type IParser;

    #[wasm_bindgen(method, js_name = "registerCsiHandler")]
    fn register_csi_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        callback: &Function, // (params: (u32 | u32[])[]) => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerDcsHandler")]
    fn register_dcs_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        callback: &Function, // (data: String, param: (u32 | u32[])[]) => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerEscHandler")]
    fn register_esc_handler(
        this: &IFunctionIdentifier,
        id: IFunctionIdentifier,
        handler: &Function, // () => bool
    ) -> IDisposable;

    #[wasm_bindgen(method, js_name = "registerOscHandler")]
    fn register_osc_handler(
        this: &IFunctionIdentifier,
        ident: u32,
        callback: &Function, // (data: String) => bool
    ) -> IDisposable;

    // ========================================================================

    type IUnicodeVersionProvider;

    #[wasm_bindgen(method, getter, js_name = "version")]
    fn get_version(this: &IUnicodeVersionProvider) -> String;

    #[wasm_bindgen(method, js_name = "wcwidth")]
    fn wcwidth(this: &IViewportRangePosition, codepoint: u32) -> WcWidth;

    // ========================================================================

    type IUnicodeHandling;

    #[wasm_bindgen(method, js_name = "register")]
    fn register(this: &IUnicodeHandling, provider: IUnicodeVersionProvider);

    #[wasm_bindgen(method, getter, js_name = "versions")]
    fn get_versions(this: &IUnicodeHandling) -> Box<[JsValue]>; // Box<[String]>

    #[wasm_bindgen(method, getter, js_name = "activeVersion")]
    fn get_active_version(this: &IUnicodeHandling) -> String;

}
