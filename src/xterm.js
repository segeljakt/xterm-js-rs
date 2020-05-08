declare module 'xterm' {
  export type FontWeight = 'normal' | 'bold' | '100' | '200' | '300' | '400' | '500' | '600' | '700' | '800' | '900';
  export type LogLevel = 'debug' | 'info' | 'warn' | 'error' | 'off';
  export type RendererType = 'dom' | 'canvas';
  export interface ITerminalOptions {
    allowTransparency?: boolean;
    bellSound?: string;
    bellStyle?: 'none' | 'sound';
    convertEol?: boolean;
    cols?: number;
    cursorBlink?: boolean;
    cursorStyle?: 'block' | 'underline' | 'bar';
    cursorWidth?: number;
    disableStdin?: boolean;
    drawBoldTextInBrightColors?: boolean;
    fastScrollModifier?: 'alt' | 'ctrl' | 'shift' | undefined;
    fastScrollSensitivity?: number;
    fontSize?: number;
    fontFamily?: string;
    fontWeight?: FontWeight;
    fontWeightBold?: FontWeight;
    letterSpacing?: number;
    lineHeight?: number;
    linkTooltipHoverDuration?: number;
    logLevel?: LogLevel;
    macOptionIsMeta?: boolean;
    macOptionClickForcesSelection?: boolean;
    minimumContrastRatio?: number;
    rendererType?: RendererType;
    rightClickSelectsWord?: boolean;
    rows?: number;
    screenReaderMode?: boolean;
    scrollback?: number;
    scrollSensitivity?: number;
    tabStopWidth?: number;
    theme?: ITheme;
    windowsMode?: boolean;
    wordSeparator?: string;
    windowOptions?: IWindowOptions;
  }

  export interface ITheme {
    foreground?: string;
    background?: string;
    cursor?: string;
    cursorAccent?: string;
    selection?: string;
    black?: string;
    red?: string;
    green?: string;
    yellow?: string;
    blue?: string;
    magenta?: string;
    cyan?: string;
    white?: string;
    brightBlack?: string;
    brightRed?: string;
    brightGreen?: string;
    brightYellow?: string;
    brightBlue?: string;
    brightMagenta?: string;
    brightCyan?: string;
    brightWhite?: string;
  }

  export interface ILinkMatcherOptions {
    matchIndex?: number;
    validationCallback?: (uri: string, callback: (isValid: boolean) => void) => void;
    tooltipCallback?: (event: MouseEvent, uri: string, location: IViewportRange) => boolean | void;
    leaveCallback?: () => void;
    priority?: number;
    willLinkActivate?: (event: MouseEvent, uri: string) => boolean;
  }

  export interface IDisposable {
    dispose(): void;
  }
  export interface IEvent<T, U = void> {
    (listener: (arg1: T, arg2: U) => any): IDisposable;
  }

  export interface IMarker extends IDisposable {
    readonly id: number;
    readonly isDisposed: boolean;
    readonly line: number;
  }

  export interface ILocalizableStrings {
    promptLabel: string;
    tooMuchOutput: string;
  }

  export interface IWindowOptions {
    restoreWin?: boolean;
    minimizeWin?: boolean;
    setWinPosition?: boolean;
    setWinSizePixels?: boolean;
    raiseWin?: boolean;
    lowerWin?: boolean;
    refreshWin?: boolean;
    setWinSizeChars?: boolean;
    maximizeWin?: boolean;
    fullscreenWin?: boolean;
    getWinState?: boolean;
    getWinPosition?: boolean;
    getWinSizePixels?: boolean;
    getScreenSizePixels?: boolean;
    getCellSizePixels?: boolean;
    getWinSizeChars?: boolean;
    getScreenSizeChars?: boolean;
    getIconTitle?: boolean;
    getWinTitle?: boolean;
    pushTitle?: boolean;
    popTitle?: boolean;
    setWinLines?: boolean;
  }

  export class Terminal implements IDisposable {
    readonly element: HTMLElement | undefined;
    readonly textarea: HTMLTextAreaElement | undefined;
    readonly rows: number;
    readonly cols: number;
    readonly buffer: IBufferNamespace;
    readonly markers: ReadonlyArray<IMarker>;
    readonly parser: IParser;
    readonly unicode: IUnicodeHandling;
    static strings: ILocalizableStrings;
    constructor(options?: ITerminalOptions);
    onBinary: IEvent<string>;
    onCursorMove: IEvent<void>;
    onData: IEvent<string>;
    onKey: IEvent<{key: string, domEvent: KeyboardEvent}>;
    onLineFeed: IEvent<void>;
    onScroll: IEvent<number>;
    onSelectionChange: IEvent<void>;
    onRender: IEvent<{start: number, end: number}>;
    onResize: IEvent<{cols: number, rows: number}>;
    onTitleChange: IEvent<string>;
    blur(): void;
    focus(): void;
    resize(columns: number, rows: number): void;
    open(parent: HTMLElement): void;
    attachCustomKeyEventHandler(customKeyEventHandler: (event: KeyboardEvent) => boolean): void;
    registerLinkMatcher(regex: RegExp, handler: (event: MouseEvent, uri: string) => void, options?: ILinkMatcherOptions): number;
    deregisterLinkMatcher(matcherId: number): void;
    registerLinkProvider(linkProvider: ILinkProvider): IDisposable;
    registerCharacterJoiner(handler: (text: string) => [number, number][]): number;
    deregisterCharacterJoiner(joinerId: number): void;
    registerMarker(cursorYOffset: number): IMarker | undefined;
    addMarker(cursorYOffset: number): IMarker | undefined;
    hasSelection(): boolean;
    getSelection(): string;
    getSelectionPosition(): ISelectionPosition | undefined;
    clearSelection(): void;
    select(column: number, row: number, length: number): void;
    selectAll(): void;
    selectLines(start: number, end: number): void;
    dispose(): void;
    scrollLines(amount: number): void;
    scrollPages(pageCount: number): void;
    scrollToTop(): void;
    scrollToBottom(): void;
    scrollToLine(line: number): void;
    clear(): void;
    write(data: string | Uint8Array, callback?: () => void): void;
    writeln(data: string | Uint8Array, callback?: () => void): void;
    writeUtf8(data: Uint8Array, callback?: () => void): void;
    paste(data: string): void;
    getOption(key: 'bellSound' | 'bellStyle' | 'cursorStyle' | 'fontFamily' | 'fontWeight' | 'fontWeightBold' | 'logLevel' | 'rendererType' | 'termName' | 'wordSeparator'): string;
    getOption(key: 'allowTransparency' | 'cancelEvents' | 'convertEol' | 'cursorBlink' | 'disableStdin' | 'macOptionIsMeta' | 'rightClickSelectsWord' | 'popOnBell' | 'visualBell' | 'windowsMode'): boolean;
    getOption(key: 'cols' | 'fontSize' | 'letterSpacing' | 'lineHeight' | 'rows' | 'tabStopWidth' | 'scrollback'): number;
    getOption(key: string): any;
    setOption(key: 'fontFamily' | 'termName' | 'bellSound' | 'wordSeparator', value: string): void;
    setOption(key: 'fontWeight' | 'fontWeightBold', value: null | 'normal' | 'bold' | '100' | '200' | '300' | '400' | '500' | '600' | '700' | '800' | '900'): void;
    setOption(key: 'logLevel', value: LogLevel): void;
    setOption(key: 'bellStyle', value: null | 'none' | 'visual' | 'sound' | 'both'): void;
    setOption(key: 'cursorStyle', value: null | 'block' | 'underline' | 'bar'): void;
    setOption(key: 'allowTransparency' | 'cancelEvents' | 'convertEol' | 'cursorBlink' | 'disableStdin' | 'macOptionIsMeta' | 'popOnBell' | 'rightClickSelectsWord' | 'visualBell' | 'windowsMode', value: boolean): void;
    setOption(key: 'fontSize' | 'letterSpacing' | 'lineHeight' | 'tabStopWidth' | 'scrollback', value: number): void;
    setOption(key: 'theme', value: ITheme): void;
    setOption(key: 'cols' | 'rows', value: number): void;
    setOption(key: string, value: any): void;
    refresh(start: number, end: number): void;
    reset(): void;
    loadAddon(addon: ITerminalAddon): void;
  }

  export interface ITerminalAddon extends IDisposable {
    activate(terminal: Terminal): void;
  }

  interface ISelectionPosition {
    startColumn: number;
    startRow: number;
    endColumn: number;
    endRow: number;
  }

  export interface IViewportRange {
    start: IViewportRangePosition;
    end: IViewportRangePosition;
  }

  interface IViewportRangePosition {
    x: number;
    y: number;
  }

  interface ILinkProvider {
    provideLinks(bufferLineNumber: number, callback: (links: ILink[] | undefined) => void): void;
  }

  interface ILink {
    range: IBufferRange;
    text: string;
    decorations?: ILinkDecorations;
    activate(event: MouseEvent, text: string): void;
    hover?(event: MouseEvent, text: string): void;
    leave?(event: MouseEvent, text: string): void;
  }

  interface ILinkDecorations {
    pointerCursor: boolean;
    underline: boolean;
  }

  interface IBufferRange {
    start: IBufferCellPosition;
    end: IBufferCellPosition;
  }

  interface IBufferCellPosition {
    x: number;
    y: number;
  }

  interface IBuffer {
    readonly type: 'normal' | 'alternate';
    readonly cursorY: number;
    readonly cursorX: number;
    readonly viewportY: number;
    readonly baseY: number;
    readonly length: number;
    getLine(y: number): IBufferLine | undefined;
    getNullCell(): IBufferCell;
  }

  interface IBufferNamespace {
    readonly active: IBuffer;
    readonly normal: IBuffer;
    readonly alternate: IBuffer;
    onBufferChange: IEvent<IBuffer>;
  }

  interface IBufferLine {
    readonly isWrapped: boolean;
    readonly length: number;
    getCell(x: number, cell?: IBufferCell): IBufferCell | undefined;
    translateToString(trimRight?: boolean, startColumn?: number, endColumn?: number): string;
  }

  interface IBufferCell {
    getWidth(): number;
    getChars(): string;
    getCode(): number;
    getFgColorMode(): number;
    getBgColorMode(): number;
    getFgColor(): number;
    getBgColor(): number;
    isBold(): number;
    isItalic(): number;
    isDim(): number;
    isUnderline(): number;
    isBlink(): number;
    isInverse(): number;
    isInvisible(): number;
    isFgRGB(): boolean;
    isBgRGB(): boolean;
    isFgPalette(): boolean;
    isBgPalette(): boolean;
    isFgDefault(): boolean;
    isBgDefault(): boolean;
    isAttributeDefault(): boolean;
  }

  export interface IFunctionIdentifier {
    prefix?: string;
    intermediates?: string;
    final: string;
  }

  export interface IParser {
    registerCsiHandler(id: IFunctionIdentifier, callback: (params: (number | number[])[]) => boolean): IDisposable;
    registerDcsHandler(id: IFunctionIdentifier, callback: (data: string, param: (number | number[])[]) => boolean): IDisposable;
    registerEscHandler(id: IFunctionIdentifier, handler: () => boolean): IDisposable;
    registerOscHandler(ident: number, callback: (data: string) => boolean): IDisposable;
  }

  export interface IUnicodeVersionProvider {
    readonly version: string;
    wcwidth(codepoint: number): 0 | 1 | 2;
  }

  export interface IUnicodeHandling {
    register(provider: IUnicodeVersionProvider): void;
    readonly versions: ReadonlyArray<string>;
    activeVersion: string;
  }
}
