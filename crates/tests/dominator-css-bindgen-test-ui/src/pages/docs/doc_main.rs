use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::DocPage;
use dominator::Dom;
use futures_signals::signal::{Signal, SignalExt};

pub fn doc_main_view(current_doc: impl Signal<Item = Option<DocPage>> + 'static) -> Dom {
    html!("div", {
        .child_signal(current_doc.map(|doc| {
            doc.map(|doc| {
                match doc {
                    DocPage::Flex => flex_page(),
                    _ => html!("div", { .text("todo")})
                }
            })
        }))
    })
}