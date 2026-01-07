use dioxus::prelude::*;

use crate::layout::Layout;
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Routes {
    #[layout(Layout)]
        #[route("/")]
        HomePage {},
}

impl Routes {
    pub fn home() -> Self {
        Self::HomePage {}
    }
}
