use super::super::state;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct BoardProps {
    pub board: state::Board,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    html! {
        <div>
        { for props.board.iter().map(|r|
                html! {
                    <div>
                    { for r.iter().map(|cell|
                            html!{
                                { cell.letter }
                            }
                        )
                    }
                    </div>
                }
            )
        }
        </div>
    }
}
