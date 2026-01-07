use std::sync::LazyLock;

use dioxus::prelude::{Asset, asset, manganis};

pub static SOURCE_CODE_URL: LazyLock<String> =
    LazyLock::new(|| format!("{}/tree/{}", env!("CARGO_PKG_REPOSITORY"), env!("GIT_REV_SHORT")));

pub const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
pub const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
pub const FAVICON_PNG: Asset = asset!("assets/favicon.png");
