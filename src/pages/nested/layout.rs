use yew::prelude::*;

use crate::utils::use_render_count;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Layout(props: &Props) -> Html {
    let render_count = use_render_count();

    html! {
      <>
          <div style="border: 4px dotted blue">
              <h1>{" This is just a demo Layout | Render Count: "} {render_count}</h1>
          </div>
          <div style="border: 4px dotted red">
              { for props.children.iter() }
          </div>
      </>
    }
}
