use dominator::{text, Dom};
use dwind::prelude::*;
use dwind_macros::dwclass;
use dwui::prelude::*;

pub fn doc_page_title(title: &str) -> Dom {
    heading!({
        .apply(|b| {
            dwclass!(b, "m-t-8")
        })
        .content(text(title))
        .text_size(TextSize::ExtraLarge)
    })
}

pub fn doc_page_sub_header(title: &str) -> Dom {
    heading!({
        .apply(|b| {
            dwclass!(b, "m-t-10")
        })
        .content(text(title))
        .text_size(TextSize::Large)
    })
}
