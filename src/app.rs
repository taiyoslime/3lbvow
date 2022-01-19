use yew::prelude::*;

mod components;
mod constants;
mod helper;
mod state;

use state::*;

use components::{alert::Alert, board::Board, keyboard::Keyboard};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| State {
        answer: helper::generate_new_answer().to_string(),
        ..Default::default()
    });

    let onkeypress = {
        let state = state.clone();
        Callback::from(move |action: state::Action| state.dispatch(action))
    };

    let reset = {
        let state = state.clone();
        Callback::from(move |_| state.dispatch(Action::Reset))
    };

    html! {
        <>
            <Board board={state.board.clone()} />
            <Keyboard onkeypress={onkeypress} />
            <Alert>
                { state.alert_message.clone() }
            </Alert>
            {format!("{:?}", state.game_status)}
            <button onclick={reset}> { "reset" } </button>
        </>
    }
}
