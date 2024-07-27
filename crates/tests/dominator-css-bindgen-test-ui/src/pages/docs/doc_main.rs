use crate::pages::docs::doc_pages::colors::colors_page;
use crate::pages::docs::doc_pages::flex::flex_page;
use crate::pages::docs::DocPage;
use dominator::Dom;
use futures_signals::signal::{Signal, SignalExt};
use crate::pages::docs::doc_pages::responsive_design::responsive_design;

pub fn doc_main_view(current_doc: impl Signal<Item = Option<DocPage>> + 'static) -> Dom {
    html!("div", {
        .child_signal(current_doc.map(|doc| {
            doc.map(|doc| {
                match doc {
                    DocPage::Flex => flex_page(),
                    DocPage::Colors => colors_page(),
                    DocPage::ResponsiveDesign => responsive_design(),
                    _ => html!("div", { .text("todo")})
                }
            })
        }))
    })
}
