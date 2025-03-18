mod components;
mod game_state_context;
use components::{reversi_table::ReversiTable, setting_header::SettingHeader};
use game_state_context::GameStateProvider;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("p-10")}>
            <GameStateProvider>
                <div class={classes!("pb-4")}>
                    <SettingHeader />
                </div>
                <ReversiTable />
            </GameStateProvider>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
