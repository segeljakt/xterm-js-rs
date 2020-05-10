<h1 align="center">🕸 xterm-js-rs 🦀</h1>
This crate provides *Rust*-*WebAssembly* bindings for the [xterm-js](https://www.github.com/xtermjs/xterm.js) *Javascript* library. The crate can be used to setup a custom web-based [*command-line*](https://www.segeljakt.github.io/xterm-js-rs), running at the client-side, without much effort.

* 🎥 To see it running in action on GitHub pages: https://www.segeljakt.github.io/xterm-js-rs.
* 📝 Code for the example can be found [here](https://www.github.com/segeljakt/xterm-js-rs/tree/master/example).
* 🚀 The GitHub Actions workflow for automatically deploying the website to GitHub pages can be found [here](https://www.github.com/segeljakt/xterm-js-rs/blob/master/.github/workflows/gh-pages.yml).
* 🔬 For an overview of what the bindings do, checkout the official [API](https://www.github.com/xtermjs/xterm.js/blob/master/typings/xterm.d.ts).
* 🎚 Conditionally, [addons](https://www.github.com/xtermjs/xterm.js/tree/master/addons) of `xterm-js` can be activated by compiling this crate with the corresponding feature enabled:
  - `xterm-addon-attach`
  - `xterm-addon-fit`
  - `xterm-addon-ligatures`
  - `xterm-addon-search`
  - `xterm-addon-serialize`
  - `xterm-addon-unicode11`
  - `xterm-addon-web-links`
  - `xterm-addon-webgl`
* ⚠️ If your npm-crate depends on this crate, then it must contain a `package.json` in the root directory and `www` directory which specifies the dependencies to the `xterm-js` library. As in the example:
  - [xterm-js-rs/example/package.json](https://www.github.com/segeljakt/xterm-js-rs/blob/c5c1a2ab5ba605c83d517330b41a90f658b2c123/example/package.json#L3-L4)
  - [xterm-js-rs/example/www/package.json](https://www.github.com/segeljakt/xterm-js-rs/blob/c5c1a2ab5ba605c83d517330b41a90f658b2c123/example/www/package.json#L31-L32)
