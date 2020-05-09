#[cfg(feature = "xterm-addon-attach")]
pub mod attach;
#[cfg(feature = "xterm-addon-attach")]
pub use attach::*;

#[cfg(feature = "xterm-addon-fit")]
pub mod fit;
#[cfg(feature = "xterm-addon-fit")]
pub use fit::*;

#[cfg(feature = "xterm-addon-ligatures")]
pub mod ligatures;
#[cfg(feature = "xterm-addon-ligatures")]
pub use ligatures::*;

#[cfg(feature = "xterm-addon-search")]
pub mod search;
#[cfg(feature = "xterm-addon-search")]
pub use search::*;

#[cfg(feature = "xterm-addon-serialize")]
pub mod serialize;
#[cfg(feature = "xterm-addon-serialize")]
pub use serialize::*;

#[cfg(feature = "xterm-addon-unicode11")]
pub mod unicode11;
#[cfg(feature = "xterm-addon-unicode11")]
pub use unicode11::*;

#[cfg(feature = "xterm-addon-web-links")]
pub mod web_links;
#[cfg(feature = "xterm-addon-web-links")]
pub use web_links::*;

#[cfg(feature = "xterm-addon-webgl")]
pub mod webgl;
#[cfg(feature = "xterm-addon-webgl")]
pub use webgl::*;
