use crate::pages::nested::NestedRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Index() -> Html {
    html! {
      <>
          <h1>{"Nested Index"}</h1>
          <p>{"These routes just demonstrate nested routing. They don't show anything interesting right now."}</p>
          <p>{"The main takeaway here is that by introducing a nested router, when the URL is changed to something only the nested router handles, only the children of this router are re-rendered, elements outside of this (such as the above layout) are untouched"}</p>
          <ul>
              <li><Link<NestedRoute> to={NestedRoute::Page1}>{"Page 1"}</Link<NestedRoute>></li>
              <li><Link<NestedRoute> to={NestedRoute::Page2}>{"Page 2"}</Link<NestedRoute>></li>
          </ul>
      </>
    }
}
