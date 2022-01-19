use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AlertProps {
    pub children: Children,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div> { props.children.clone() }</div>
    }
}
