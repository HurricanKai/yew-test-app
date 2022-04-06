pub mod components;
pub mod pages;
pub mod utils;

use self::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Landing,
    #[at("/counter")]
    Counter,
    #[at("/loading")]
    Loading,
    #[at("/nested")]
    NestedRoot,
    #[at("/nested/*")]
    Nested,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &MainRoute) -> Html {
    match routes {
        MainRoute::Landing => html! { <Landing />},
        MainRoute::Counter => html! { <Counter /> },
        MainRoute::Loading => html! { <Loading /> },
        MainRoute::NotFound => html! { <NotFound /> },
        MainRoute::Nested | MainRoute::NestedRoot => html! { <Nested /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
