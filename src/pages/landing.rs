use yew::prelude::*;
use yew_router::prelude::*;

use crate::{utils::use_render_count, MainRoute};

#[function_component]
pub fn Landing() -> Html {
    let render_count = use_render_count();

    html! {
      <>
          <h1>{"Landing Page"}</h1>
          <p>{"On some pages you will notice a render counter, this is to showcase how often specific parts of the page are re-calculated"}</p>
          <p>{"Render Count: "} {render_count}</p>
          <p>{"Check out the pages below, they are an accumulation of experiments I've done with Yew."}</p>
          <ul>
              <li><Link<MainRoute> to={MainRoute::Counter}>{"Counter"}</Link<MainRoute>></li>
              <li><Link<MainRoute> to={MainRoute::NestedRoot}>{"Nested"}</Link<MainRoute>></li>
              <li>{"TODO: Suspense"}</li>
              <li>{"TODO: Portals"}</li>
              <li>{"TODO: SSR"}</li>
          </ul>
      </>
    }
}
