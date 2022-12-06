use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home;

//pub enum HomeMsg {
//}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <content id="container-home">
                <section id="introduction">
                    <h2>{"Bem-vindo(a) ao Plantê!"}</h2>
                    <p>{"Um sistema de comércio de plantas ornamentais e medicinais."}</p>
                </section>

                <section id="random-ornamental">
                    <h2>{"Plantas ornamentais"}</h2>
                    // Alguns cartões de planta devem ser exibidos nesta seção
                </section>
            </content>
        }
    }
}
