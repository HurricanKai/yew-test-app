use yew::prelude::*;
use std::ops::Deref;

#[function_component]
fn App() -> Html {
    let counter_handle : UseStateHandle<i32> = use_state(|| 0);
    let counter = counter_handle.deref().clone();
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let value = counter + 1;
            counter_handle.set(value);
            web_sys::console::log_1(&counter.to_string().into());
        })
    };

    html! {
        <div>
            <button onclick={onclick}>{ "+1" }</button>
            <p>{ counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
