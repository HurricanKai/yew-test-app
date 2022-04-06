mod index;
mod layout;
mod page_1;
mod page_2;

use self::index::Index;
use self::layout::Layout;
use self::page_1::Page1;
use self::page_2::Page2;
use crate::pages::NotFound;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum NestedRoute {
    #[at("/nested")]
    NestedRoot,
    #[at("/nested/page1")]
    Page1,
    #[at("/nested/page2")]
    Page2,
    #[not_found]
    #[at("/nested/404")]
    NotFound,
}

fn switch(route: &NestedRoute) -> Html {
    match route {
        NestedRoute::NestedRoot => html! { <Index /> },
        NestedRoute::Page1 => html! { <Page1 /> },
        NestedRoute::Page2 => html! { <Page2 /> },
        NestedRoute::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
pub fn Nested() -> Html {
    html! {
        <Layout>
            <Switch<NestedRoute> render={Switch::render(switch)} />
        </Layout>
    }
}
