use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::nested::NestedRoute;

#[function_component]
pub fn Index() -> Html {

  html! {
    <>
        <h1>{"Nested Index"}</h1>
        <p>
            {"These routes just demonstrate nested routing. They don't show anything interesting right now."}
        </p>
        <ul>
            <li><Link<NestedRoute> to={NestedRoute::Page1}>{"Page 1"}</Link<NestedRoute>></li>
            <li><Link<NestedRoute> to={NestedRoute::Page2}>{"Page 2"}</Link<NestedRoute>></li>
        </ul>
    </>
  }
}