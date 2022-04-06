use gloo_console::console;
use yew::prelude::*;
use std::ops::Deref;

#[function_component]
pub fn Counter() -> Html {
    let counter_handle : UseStateHandle<i32> = use_state(|| 0);
    let counter = counter_handle.deref().clone();
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = counter + 1;
            counter_handle.set(value);
            console!(counter.to_string());
        })
    };

    use_effect_with_deps(move |_| {
        gloo_utils::document().set_title(&format!("Counter: {}", counter));

        || gloo_utils::document().set_title("Counter: 0")
    }, counter);

    html! {
        <div>
            <button onclick={onclick}>{ "+1" }</button>
            <p>{ counter }</p>
        </div>
    }
}