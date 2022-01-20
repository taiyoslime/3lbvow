use super::super::state;
use state::{CellStatus};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct BoardProps {
    pub board: state::Board,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {

    html! {
        <>
        { for props.board.iter().map(|r|
                html! {
                    <div class="flex justify-center space-x-0.5 mb-0.5">
                    { for r.iter().map(|cell|
                            html!{
                                <Cell cell={cell.clone()}/>
                            }
                        )
                    }
                    </div>
                }
            )
        }
        </>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct CellProps {
    cell: state::Cell
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let bg = String::from(
        match props.cell.status {
            CellStatus::Unknown => "bg-slate-200",
            CellStatus::Correct => "bg-green-500",
            CellStatus::Present => "bg-yellow-500",
            CellStatus::Absent => "bg-gray-500"
    });

    let tc = String::from(
        match props.cell.status {
            CellStatus::Unknown => "text-neutral-700",
            _ => "text-slate-100"
    });

    html! {
        <div class={classes!("h-16", "w-16", "flex", "items-center", "justify-center", "rounded", bg)} >
            <p class={classes!("text-3xl", "font-bold", tc)}>{ props.cell.letter }</p>
        </div>
    }

}