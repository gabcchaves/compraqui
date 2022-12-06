use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{
    login::Login,
    register::Register,
    product::Product,
    cart::Cart,
    home::Home,
};

pub mod login;
pub mod register;
pub mod product;
pub mod cart;
pub mod home;

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
    #[at("/product")]
    Product,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::Login => html! {
            <Login/>
        },
        Route::Register => html! {
            <Register/>
        },
        Route::Cart => html! {
            <Cart/>
        },
        Route::Explore => html! {
            <h1>{"Explorar"}</h1>
        },
        Route::Product => html! {
            <Product/>
        },
        Route::NotFound => html! {
            <h1>{"Não encontrado"}</h1>
        },
    }
}
