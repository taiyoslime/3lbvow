use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

use super::super::state;
use state::{AlphaStatus};

#[derive(PartialEq, Properties, Clone)]
pub struct KeyboardProps {
    pub onkeypress: Callback<state::Action>,
    pub alphabets_status: state::AlphabetsStatus
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let onkeypress = {
        let onkeypress = props.onkeypress.clone();
        move |e: &KeyboardEvent| match e.key().as_str() {
            "Enter" => onkeypress.emit(state::Action::PressEnter),
            "Backspace" => onkeypress.emit(state::Action::PressDelete),
            c if c.len() == 1 => {
                let ch = c.chars().nth(0).unwrap();
                if ch.is_ascii_alphabetic() {
                    onkeypress.emit(state::Action::PressChar(ch.to_ascii_uppercase()))
                }
            }
            _ => (),
        }
    };

    use_effect(move || {
        let document = gloo::utils::document();
        let listener = gloo::events::EventListener::new(&document, "keydown", move |event| {
            let e = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            onkeypress(e);
        });

        || drop(listener)
    });
    
    // TODO
    let onclick_enter = {
        let onkeypress = props.onkeypress.clone();
        Callback::from(move |_| onkeypress.emit(state::Action::PressEnter))
    };

    // TODO
    let onclick_delete = {
        let onkeypress = props.onkeypress.clone();
        Callback::from(move |_| onkeypress.emit(state::Action::PressDelete))
    };

    // TODO
    html! {
        <div class="select-none">
            <div class="flex justify-center space-x-1 mb-1">
                { for "QWERTYUIOP".chars().map(|ch| {
                        let onclick = {
                            let onkeypress = props.onkeypress.clone();
                            Callback::from(move |_| onkeypress.emit(state::Action::PressChar(ch)))
                        };
                        html! {
                            <Key onclick={onclick} status={props.alphabets_status[&ch].clone()} width={classes!("w-10")}>
                                {ch}
                            </Key>
                        }
                    })
                }
            </div>
            <div class="flex justify-center space-x-1 mb-1">
                { for "ASDFGHJKL".chars().map(|ch| {
                    let onclick = {
                        let onkeypress = props.onkeypress.clone();
                        Callback::from(move |_| onkeypress.emit(state::Action::PressChar(ch)))
                    };
                    html! {
                        <Key onclick={onclick} status={props.alphabets_status[&ch].clone()} width={classes!("w-10")}>
                            {ch}
                        </Key>
                    }})
                }
            </div>
            <div class="flex justify-center space-x-1 mb-1">
                <Key onclick={onclick_enter} status={AlphaStatus::Unknown} width={classes!("w-20")}>
                    {"ENTER"}
                </Key>
                
                { for "ZXCVBNM".chars().map(|ch| {
                    let onclick = {
                        let onkeypress = props.onkeypress.clone();
                        Callback::from(move |_| onkeypress.emit(state::Action::PressChar(ch)))
                    };
                    html! {
                        <Key onclick={onclick} status={props.alphabets_status[&ch].clone()} width={classes!("w-10")}>
                            {ch}
                        </Key>
                    }}
                )}
                <Key onclick={onclick_delete} status={AlphaStatus::Unknown} width={classes!("w-20")}>
                    {"DELETE"}
                </Key>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct KeyProps {
    status: state::AlphaStatus,
    children: Children,
    width: Classes,
    onclick: Callback<yew::MouseEvent>,
}

#[function_component(Key)]
pub fn cell(props: &KeyProps) -> Html {
    // TODO
    let bg = String::from(
        match props.status {
            AlphaStatus::Unknown => "bg-slate-200 hover:bg-slate-300 active:bg-slate-400",
            AlphaStatus::Correct => "bg-green-500 hover:bg-green-600 active:bg-green-700",
            AlphaStatus::Present => "bg-yellow-500 hover:bg-yellow-600 active:bg-yellow-700",
            AlphaStatus::Absent => "bg-gray-500 hover:bg-gray-600 active:bg-gray-700"
    });

    let tc = String::from(
        match props.status {
            AlphaStatus::Unknown => "text-neutral-700",
            _ => "text-slate-100"
    });

    let onclick = props.onclick.clone();

    html! {
        <div {onclick} class={classes!("h-14", props.width.clone(), "flex", "items-center", "justify-center", "rounded", "cursor-pointer", bg)} >
            <p class={classes!("font-bold", tc)}>{ props.children.clone() }</p>
        </div>
    }

}