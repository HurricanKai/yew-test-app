pub mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use self::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Landing,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &MainRoute) -> Html {
    match routes {
        MainRoute::Landing => html! { <Landing />},
        MainRoute::Counter => html! { <Counter /> },
        MainRoute::NotFound => html! { <NotFound /> },
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
