use crate::routes::login::Login;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/cart")]
    Cart,
    #[at("/explore")]
    Explore,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <h1>{"Página Inicial"}</h1>
        },
        Route::Login => html! {
            <>
                <Login/>
            </>
        },
        Route::NotFound => html! {
            <h1>{"Não encontrado"}</h1>
        },
        Route::Register => html! {
            <h1>{"Cadastro"}</h1>
        },
        Route::Cart => html! {
            <h1>{"Carrinho"}</h1>
        },
        Route::Explore => html! {
            <h1>{"Explorar"}</h1>
        },
    }
}
