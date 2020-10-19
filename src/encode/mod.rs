use docsify_cli::{codegen::markdown_str2ast, ToYew};
use yew::{html, prelude::*, Component, ComponentLink, Html, ShouldRender};
use yew::ChangeData;

#[derive(Debug)]
pub enum Markup {
    Markdown,
    Notedown,
    OrgMode,
    RichTextFormat,
}

pub enum Event {
    Input(String),
    SwitchTo(ChangeData),
    Upload,
    Clean,
    Copy,
}

pub struct Converter {
    link: ComponentLink<Self>,
    text: String,
    markup: Markup,
}

impl Component for Converter {
    type Message = Event;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, text: String::from(include_str!("readme.md")), markup: Markup::Markdown }
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
            Event::SwitchTo(u) => {
                 if let ChangeData::Select(s) = u {
                     self.markup = match s.value().as_str() {
                         "0" => Markup::Markdown,
                         "1" => Markup::Notedown,
                         "2" => Markup::OrgMode,
                         "3" => Markup::RichTextFormat,
                         _ => unreachable!(),
                     }
                 };
                true
            }
            Event::Upload => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let lang = format!("Input {:?}", self.markup);
        let input: Html = html! {
        <div class="form-group">
            <label class="form-label">{lang}</label>
            <textarea
                rows="30"
                oninput=self.link.callback(|input: InputData| Event::Input(input.value))
                value=&self.text
            />
        </div>
        };
        let out_text = if self.text.is_empty() {
            String::new()
        }
        else {
            let ast = match self.markup {
                Markup::Markdown => markdown_str2ast(&self.text).to_yew(),
                Markup::Notedown => String::from("unimplemented!"),
                Markup::OrgMode => String::from("unimplemented!"),
                Markup::RichTextFormat => String::from("unimplemented!"),
            };
            ast
        };
        let output: Html = html! {
        <div class="form-group">
            <label class="form-label">{"Output Yew"}</label>
            <textarea readonly=true rows="30" value=&out_text/>
        </div>
        };
        let bottoms: Html = html! {
        <div class="button-group">
        <button
        class="button"
        onclick=self.link.callback(|_| Event::Copy)
        >
        <i class="fas fa-file-upload"></i>
        <span>{"Upload"}</span>
        </button>
        <button
        class="button danger"
        onclick=self.link.callback(|_| Event::Clean)
        >
        <i class="fas fa-trash"></i>
        <span>{"Clear"}</span>
        </button>
        <button
        class="button"
        onclick=self.link.callback(|_| Event::Copy)
        >
        <i class="fas fa-clipboard"></i>
        <span>{"Copy"}</span>
        </button>

        <select onchange=self.link.callback(|v: ChangeData| Event::SwitchTo(v))>
        <option value=0>{"markdown"}</option>
        <option value=1>{"notedown"}</option>
        <option value=2>{"org-mode"}</option>
        <option value=3>{"richtext"}</option>
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
