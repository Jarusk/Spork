#![warn(missing_docs)]
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://jarusk.github.io/spork")]

//! Some sample documentation
//!

mod str_map;
mod wrapped_val;
pub use str_map::StrMap as Spork;
