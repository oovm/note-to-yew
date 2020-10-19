pub use crate::encode::Converter;
use yew::prelude::*;

pub fn view() -> Html {
    let title = "Markups to Yew";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/note-to-yew">{"Fork me!"}</a>
    </header>
    }
}
