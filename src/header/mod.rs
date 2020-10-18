pub use crate::encode::Converter;
use yew::prelude::*;

pub fn view() -> Html {
    let title = "Brainfuck Encoder/Decoder";
    html! {
    <header>
        <h1 color="#009688">{title}</h1>
        <a href="https://github.com/GalAster/note-to-yew">{"Fork me!"}</a>
    </header>
    }
}

pub fn svg_trash() -> Html {
    html! {
    <svg data-icon="trash" width="16" height="16" viewBox="0 0 16 16">
        <desc>{"trash"}</desc>
        <path
            d="M14.49 3.99h-13c-.28 0-.5.22-.5.5s.22.5.5.5h.5v10c0 .55.45 1 1 1h10c.55 0 1-.45 1-1v-10h.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5zm-8.5 9c0 .55-.45 1-1 1s-1-.45-1-1v-6c0-.55.45-1 1-1s1 .45 1 1v6zm3 0c0 .55-.45 1-1 1s-1-.45-1-1v-6c0-.55.45-1 1-1s1 .45 1 1v6zm3 0c0 .55-.45 1-1 1s-1-.45-1-1v-6c0-.55.45-1 1-1s1 .45 1 1v6zm2-12h-4c0-.55-.45-1-1-1h-2c-.55 0-1 .45-1 1h-4c-.55 0-1 .45-1 1v1h14v-1c0-.55-.45-1-1-1z"
            fill-rule="evenodd"
        >
        </path>
    </svg>
    }
}

pub fn svg_clipboard() -> Html {
    html! {
    <svg data-icon="clipboard" width="16" height="16" viewBox="0 0 16 16">
        <desc>{"clipboard"}</desc>
        <path
                d="M11 2c0-.55-.45-1-1-1h.22C9.88.4 9.24 0 8.5 0S7.12.4 6.78 1H7c-.55 0-1 .45-1 1v1h5V2zm2 0h-1v2H5V2H4c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h9c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1z"
                fill-rule="evenodd"
        >
        </path>
    </svg>
    }
}

