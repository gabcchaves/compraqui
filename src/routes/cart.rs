use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Cart; // TODO: Receber tabela de produtos.

pub enum CartMsg {
    Pay,
    RefreshCart, // Operações são feitas invidualmente aos itens. Então se atualiza a lista.
}

impl Component for Cart {
    type Message = CartMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // TODO
    // fn update()
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="cart-container">
                <h1>{"Carrinho"}</h1>
                <button id="btn-pay" class="button-dark">{"Pagar"}</button>
                <hr/>
                // Produtos
            </section>
        }
    }
}
