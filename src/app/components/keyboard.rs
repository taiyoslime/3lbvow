//use wasm_bindgen::JsCast;
//use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use super::super::state;

#[derive(PartialEq, Properties, Clone)]
pub struct KeyboardProps {
    pub onkeypress: Callback<state::Action>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let onkeypress = {
        let onclick = props.onkeypress.clone();
        move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" => onclick.emit(state::Action::PressEnter),
            "Backspace" => onclick.emit(state::Action::PressDelete),
            c if c.len() == 1 => onclick.emit(state::Action::PressChar(c.chars().nth(0).unwrap())),
            _ => (),
        }
    };

    /*
    use_effect(move || {
        let document = gloo::utils::document();
        let listener = gloo::events::EventListener::new(&document, "keydown", |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            let key = event.key();
            *onclick(key.as_str());
        });

        || drop(listener)
    });
    */

    // TODO
    html! {
        <div {onkeypress} tabIndex="0">{ "keyboard" } </div>
    }
}
