use docsify_cli::{codegen::markdown_str2ast, ToYew};
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};

pub enum Event {
    Input(String),
    Clean,
    Copy,
}

pub struct Converter {
    link: ComponentLink<Self>,
    text: String,
}

impl Component for Converter {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, text: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Event::Input(text) => {
                self.text = text;
                true
            }
            Event::Clean => {
                self.text = String::new();
                true
            }
            Event::Copy => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let input: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Input Brainfuck Code"}</label>
            <textarea
                rows="30"
                oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                value=&self.text
            />
        </div>
        };
        let out_text = if self.text.is_empty() { String::new() } else { markdown_str2ast(&self.text).to_yew() };
        let output: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Output String"}</label>
            <textarea readonly=true rows="30" value=&out_text/>
        </div>
        };
        let bottoms: Html = html! {
                <div class="button-group">
                    <button
                        class="button danger"
                        onclick=self.link.callback(|_| Event::Clean)
                    >
                        {crate::header::svg_trash()}
                        <span>{"Clear"}</span>
                    </button>
                    <button
                        class="button"
                        onclick=self.link.callback(|_| Event::Copy)
                    >
                        {crate::header::svg_clipboard()}
                        <span>{"Copy"}</span>
                    </button>
                    <select>
        <option value="volvo">{"Volvo"}</option>
        <option value="saab">{"Saab"}</option>
        <option value="mercedes">{"Mercedes"}</option>
        <option value="audi">{"Audi"}</option>
        </select>
                </div>
                };
        html! {
        <>
                            {bottoms}
                    <div id="converter">

                    {input}
                    {output}
                    </div>
        </>
                }
    }
}
