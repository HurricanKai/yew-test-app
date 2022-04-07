pub mod components;
pub mod pages;
pub mod utils;

use self::pages::*;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
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
    let fallback = html! {<div>{"Loading..."}</div>};

    match routes {
        MainRoute::Landing => html! { <Suspense {fallback}><Landing /></Suspense>},
        MainRoute::Counter => html! { <Suspense {fallback}><Counter /></Suspense> },
        MainRoute::Loading => html! { <Suspense {fallback}><Loading /></Suspense> },
        MainRoute::NotFound => html! { <Suspense {fallback}><NotFound /></Suspense> },
        MainRoute::Nested | MainRoute::NestedRoot => {
            html! { <Suspense {fallback}><Nested /></Suspense> }
        }
    }
}

#[function_component]
fn Layout() -> Html {
    html! {
        <Switch<MainRoute> render={Switch::render(switch)} />
    }
}

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <BrowserRouter>
                <Layout />
            </BrowserRouter>
        </Suspense>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub history: String,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.history);

    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Router {history}>
                <Layout />
            </Router>
        </Suspense>
    }
}
