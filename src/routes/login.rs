use yew::prelude::*;

pub struct Login;

pub enum LoginMsg {
    TryToLog,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link(); // Elo com contexto deste componente.
        let onsubmit = {
            Callback::from(move |e: FocusEvent| {
                e.prevent_default(); // Evita que página seja recarregada
                                     // na submissão de formulário.
            })
        };

        html! {
            <section id="login-container">
                <form id="login-form" {onsubmit}>
                    <input name="user_email" type="email" placeholder="Endereço de e-mail"/>
                    <input name="user_password" type="password" placeholder="Senha"/>
                    <input type="submit" class="button-dark" value="Entrar" onclick={link.callback(|_| LoginMsg::TryToLog)}/>
                </form>
            </section>
        }
    }
}
