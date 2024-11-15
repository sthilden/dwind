use crate::mixins::labelled_rect_mixin::labelled_rect_mixin;
use crate::prelude::{InputValueWrapper, ValidationResult};
use crate::theme::prelude::*;
use dominator::{events, html, with_node, Dom};
use dwind::prelude::*;
use futures_signals::signal::SignalExt;
use futures_signals::signal::{always, Mutable};
use futures_signals_component_macro::component;
use web_sys::HtmlInputElement;

#[component(render_fn=slider)]
struct Slider<TValue: InputValueWrapper + 'static = Mutable<String>> {
    #[default(Mutable::new("".to_string()))]
    value: TValue,

    #[signal]
    #[default(0.)]
    min: f32,

    #[signal]
    #[default(100.)]
    max: f32,

    #[signal]
    #[default(1.)]
    step: f32,

    #[signal]
    #[default("".to_string())]
    label: String,
}

pub fn slider(props: impl SliderPropsTrait + 'static) -> Dom {
    let SliderProps {
        value,
        min,
        max,
        step,
        label,
        apply,
    } = props.take();
    let value_signal = value.value_signal_cloned();

    html!("div", {
        .dwclass!("dwui-bg-void-900 is(.light *):dwui-bg-void-300 text-base")
        .dwclass!("grid")
        .child(html!("div", {
            .dwclass!("flex flex-row grid-col-1 grid-row-1 w-full align-items-center")
            .child(html!("input" => HtmlInputElement, {
                .dwclass!("h-10 grow")
                .attr("type", "range")
                .attr_signal("value", value.value_signal_cloned())
                .attr_signal("min", min.map(|v| v.to_string()))
                .attr_signal("max", max.map(|v| v.to_string()))
                .attr_signal("step", step.map(|v| v.to_string()))
                .with_node!(slider_node => {
                    .event(move |_: events::Input| {
                        value.set(slider_node.value());
                    })
                })
            }))
            .child(html!("div", {
                .dwclass!("w-10 text-center flex-none")
                .text_signal(value_signal)
            }))
        }))
        .apply(labelled_rect_mixin(label, always(true), always(ValidationResult::Valid)))
    })
}
