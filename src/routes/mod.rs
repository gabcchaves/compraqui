use crate::routes::login::Login;

pub mod login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <h1>{"Home"}</h1>
        },
        Route::Login => html! {
            <Login/>
        },
        Route::NotFound => html! {
            <h1>{"Not Found"}</h1>
        },
    }
}
