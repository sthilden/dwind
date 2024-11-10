use crate::theme::prelude::*;
use dominator::{events, html, Dom};
use dwind::prelude::*;
use dwind_macros::dwgenerate;
use futures_signals::signal::SignalExt;
use futures_signals_component_macro::component;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ButtonType {
    Flat,
    Border,
}

#[component(render_fn = button)]
struct Button<THandler: Fn(events::Click) -> () = fn(events::Click) -> ()> {
    #[signal]
    #[default(None)]
    content: Option<Dom>,
    #[default(| _: events::Click | {})]
    on_click: THandler,
    #[signal]
    #[default(false)]
    disabled: bool,
    #[signal]
    #[default(ButtonType::Flat)]
    button_type: ButtonType,
}

pub fn button(props: impl ButtonPropsTrait + 'static) -> Dom {
    let ButtonProps {
        content,
        on_click,
        disabled,
        button_type,
        apply,
    } = props.take();

    let button_type = button_type.broadcast();

    html!("button", {
        .dwclass!("is(.light *):dwui-text-on-primary-800 dwui-text-on-primary-50")
        .dwclass_signal!("dwui-bg-primary-800 hover:dwui-bg-primary-900", button_type.signal().map(|v| v == ButtonType::Flat))
        .dwclass_signal!("is(.light *):dwui-bg-primary-400 is(.light *):hover:dwui-bg-primary-500", button_type.signal().map(|v| v == ButtonType::Flat))
        .dwclass_signal!("dwui-border-primary-700 hover:dwui-border-primary-800 border bg-unset", button_type.signal().map(|v| v == ButtonType::Border))
        .dwclass_signal!("is(.light *):dwui-border-primary-200 is(.light *):hover:dwui-border-primary-300 border bg-unset", button_type.signal().map(|v| v == ButtonType::Border))
        .dwclass!("disabled:dwui-text-on-primary-500 disabled:hover:dwui-border-primary-800")
        .dwclass!("is(.light *):disabled:dwui-text-on-primary-600 is(.light *):disabled:hover:dwui-border-primary-200")
        .dwclass!("w-full h-7 font-bold p-1 cursor-pointer rounded-full")
        .apply_if(apply.is_some(), move |b| {
            b.apply(apply.unwrap())
        })
        .attr_signal("disabled", disabled.map(|v| if v { Some("disabled") } else { None }))
        .child_signal(content)
        .event(move |e: events::Click| {
            on_click(e);
        })
    })
}
