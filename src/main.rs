use yew::prelude::*;
use yew_router::prelude::*;
use plante::components::{
    header::Header,
};
use plante::routes::{
    login::Login,
    Route,
    switch,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header>
            {""}
            </Header>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
