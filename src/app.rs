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
        answer: helper::generate_new_answer(),
        alphabets_status: helper::generate_new_alphabets_status(),
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
        <div class="min-h-screen bg-slate-50">
            <div class="container mx-auto p-6">
                <div class="flex justify-center">
                    <div class="w-96 flex frex-row justify-end px-6">
                        <div onclick={reset} class="w-16 bg-blue-500 hover:bg-blue-600 active:bg-blue-700 py-2 px-3 rounded cursor-pointer"><p class="text-white font-bold">{ "reset" } </p> </div>
                    </div>
                </div>
                <div class="m-4">
                    <Board board={state.board.clone()} />
                </div>
                <div class="m-4">
                    <Keyboard alphabets_status={state.alphabets_status.clone()} onkeypress={onkeypress} />
                </div>
                <Alert>{ state.alert_message.clone() }</Alert>
            </div>
        </div>
    }
}
