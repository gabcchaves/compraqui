use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::login::Login;
use crate::routes::register::Register;

pub mod login;
pub mod register;

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
            <Login/>
        },
        Route::Register => html! {
            <Register/>
        },
        Route::Cart => html! {
            <h1>{"Carrinho"}</h1>
        },
        Route::Explore => html! {
            <h1>{"Explorar"}</h1>
        },
        Route::NotFound => html! {
            <h1>{"Não encontrado"}</h1>
        },
    }
}
