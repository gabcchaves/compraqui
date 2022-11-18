use yew::prelude::*;
use yew_router::prelude::*;
use plante::components::{
    header::Header,
    footer::Footer,
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
            <div class="main-container">
                <Switch<Route> render={Switch::render(switch)}/>
            </div>
            <Footer/>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
