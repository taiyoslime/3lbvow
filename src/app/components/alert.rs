use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AlertProps {
    pub children: Children,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div class="flex justify-center items-center h-12"> 
            <p class="text-xl font-bold">{ props.children.clone() } </p>
        </div>
    }
}
