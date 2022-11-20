/* Item do carrinho.
 * Quando o item é atualizado, a operação é efetuada no banco de dados,
 * e então o carrinho é atualizado. */
use yew::prelude::*;

pub struct CartProduct;

pub struct CartProductProps {
    id: u8,
    name: String,
    quantity: u16,
}

pub enum CartProductMsg {
    ChangeQuantity(u16),
    RemoveProduct(u8),
}

impl Component for CartProductMsg {
    type Message = CartProductMsg;
    type Properties = CartProductProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // TODO: fn update()
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        let oninput = {};
        html! {
            <div class="card-product">
                <strong>{props.name}</strong>
                <input type="number" value={props.quantity} {oninput}/>
                <button id="btn-remove-product">{"Remover"}</button>
            </div>
        }
    }
}
