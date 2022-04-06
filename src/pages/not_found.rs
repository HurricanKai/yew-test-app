use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component]
pub fn NotFound() -> Html {
    html! {
      <>
          <h1>{ "404 - Not Found" }</h1>
          <p>{"
            We aren't sure what happened, but this page could not be found.
        "}
          <Link<MainRoute> to={MainRoute::Landing}>{"Go to the Landing page"}</Link<MainRoute>>
          </p>
      </>
    }
}
