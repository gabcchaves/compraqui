use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer id="main-footer">
            <div class="logo-area">
                <img class="icon-light" src="assets/img/bx-leaf.svg" alt="Logo"/>
                <strong>{"Plantê"}</strong>
            </div>
            <div class="info-text"><strong>{"gabcchaves@zohomail.com"}</strong></div>
        </footer>
    }
}
