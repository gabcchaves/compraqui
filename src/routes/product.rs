/* A ideia é que esta rota necessite de variáveis referentes ao produto
 * passadas pelo endereço (método GET). e.g.: plante/product?id=1 */
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

pub struct Product;

pub enum ProductMsg {
    AddToCart,
    RemoveFromCart,
    Return,
}

impl Component for Product {
    type Message = ProductMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    // TODO
    // fn update()

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <section id="product-container">
                <div class="img-container"></div>
                <div class="product-info-container">
                    <Link<Route> to={Route::Explore}><img src="assets/img/bx-chevron-left.svg" alt="Voltar"/></Link<Route>>
                    <h1 id="product-name"></h1>
                    <p id="product-description"></p>
                    <p id="product-stock-amount"></p>
                    <p id="product-price"></p>
                    <form id="form-add-to-cart">
                        <input name="quantity" type="number" placeholder="Qtd.(1)"/>
                        <input class="button-dark" type="submit" value="Adicionar ao carrinho" onclick={link.callback(|_| ProductMsg::AddToCart)}/>
                    </form>
                </div>
            </section>
        }
    }
}
