use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Register;

pub enum RegisterMsg {
    TryToRegister,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // TODO
    // fn update()

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onsubmit = {
            Callback::from(move |e: FocusEvent| {
                e.prevent_default();
            })
        };
        html! {
            <section id="container-register">
                <h1>{"Cadastrar-se"}</h1>
                <hr/>
                <form id="register-form" {onsubmit}>
                    <input name="username" type="text" placeholder="Nome completo"/>
                    <input name="email" type="email" placeholder="Endereço de e-mail"/>
                    <input name="password" type="password" placeholder="Senha"/>
                    <input name="password_confirmation" type="password" placeholder="Confirmar senha"/>
                    <input name="phone" type="tel" placeholder="Telefone"/>
                    <input class="button-dark" type="submit" value="Confirmar" onclick={link.callback(|_| RegisterMsg::TryToRegister)}/>
                </form>
                <Link<Route> to={Route::Login}><small>{"Já tem conta?"}</small></Link<Route>>
            </section>
        }
    }
}
