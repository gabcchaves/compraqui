use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Header;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    // pub is_user_logged: bool,
    pub children: Children,
}

pub enum HeaderMsg {
    UserLoggedIn,
    UserLoggedOut,
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // TODO
    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> booL {}
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        // Link de acesso às propriedades deste componente.
        let link = ctx.link();
        html! {
            <header id="main-header">
                <div class="dropdown-menu">
                    <button id="btn-menu">
                        <img class="icon-light" src="assets/img/bx-dots-horizontal-rounded.svg" alt="Menu"/>
                    </button>
                    <div class="dropdown-content">
                        <Link<Route> to={Route::Login}>{"Entrar"}</Link<Route>>
                        <Link<Route> to={Route::Register}>{"Cadastrar-se"}</Link<Route>>
                    </div>
                </div>
                <div class="logo-area">
                    <Link<Route> to={Route::Home}>
                        <img class="icon-light" src="assets/img/bx-leaf.svg" alt="Logo"/>
                        <strong>{"Plantê"}</strong>
                    </Link<Route>>
                </div>

                <div class="main-options">
                    <Link<Route> to={Route::Explore}>{"Explorar"}</Link<Route>>
                    <Link<Route> to={Route::Cart}>{"Carrinho"}</Link<Route>>
                </div>

                <div class="search-area">
                    <form id="search-form">
                        <input name="search-pattern" id="search-field" type="text" placeholder="Pesquisar"/>
                        <button type="submit">
                            <img src="assets/img/bx-search.svg" alt="Pesquisar"/>
                        </button>
                    </form>
                </div>

                <div class="user-options">
                    <Link<Route> to={Route::Login}>{"Entrar"}</Link<Route>>
                    <Link<Route> to={Route::Register}>{"Cadastrar-se"}</Link<Route>>
                </div>
                { for ctx.props().children.iter() }
            </header>
        }
    }
}
