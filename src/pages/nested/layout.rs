use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Layout(props: &Props) -> Html {

  html! {
    <>
        <div style="border: 4px dotted blue">
            <h1>{" This is just a demo Layout "}</h1>
        </div>
        <div style="border: 4px dotted red">
            { for props.children.iter() }
        </div>
    </>
  }
}