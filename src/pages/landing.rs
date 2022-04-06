use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component]
pub fn Landing() -> Html {

  html! {
    <>
        <h1>{"Landing Page"}</h1>
        <p>{"Check out the pages below, they are an accumulation of experiments I've done with Yew."}</p>
        <ul>
            <li><Link<MainRoute> to={MainRoute::Counter}>{"Counter"}</Link<MainRoute>></li>
            <li><Link<MainRoute> to={MainRoute::NestedRoot}>{"Nested"}</Link<MainRoute>></li>
        </ul>
    </>
  }
}