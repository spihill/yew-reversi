use yew::prelude::*;

use crate::game_state_context::{BoardAction, GameStateContext};
use game_logic::types::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum TurnColor {
    Black,
    White,
    Random,
}

pub enum SettingHeaderMessage {
    SetColor(TurnColor),
    Start,
}

pub struct SettingHeader {
    selected_color: TurnColor,
    show: bool,
}

impl Component for SettingHeader {
    type Message = SettingHeaderMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SettingHeader {
            selected_color: TurnColor::Random,
            show: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingHeaderMessage::SetColor(color) => {
                self.selected_color = color;
            }
            SettingHeaderMessage::Start => {
                let (game_state_context, _) = ctx
                    .link()
                    .context::<GameStateContext>(Callback::noop())
                    .unwrap();

                let player_color = match self.selected_color {
                    TurnColor::Black => Color::Black,
                    TurnColor::White => Color::White,
                    TurnColor::Random => {
                        if rand::random() {
                            Color::Black
                        } else {
                            Color::White
                        }
                    }
                };
                game_state_context.dispatch(BoardAction::StartGame(player_color));
                self.show = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.show {
            return html! {};
        }

        html! {
            <div class="p-4 rounded">
                <h2 class="text-lg font-bold mb-2">{ "設定" }</h2>
                <div class="mb-4">
                    <span class="mr-2">{ "あなたの手番" }</span>
                    <label class="mr-4">
                        <input
                            type="radio"
                            name="turn"
                            value="black"
                            checked={self.selected_color == TurnColor::Black}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::Black))}
                        />
                        { "黒" }
                    </label>
                    <label class="mr-4">
                        <input
                            type="radio"
                            name="turn"
                            value="white"
                            checked={self.selected_color == TurnColor::White}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::White))}
                        />
                        { "白" }
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="turn"
                            value="random"
                            checked={self.selected_color == TurnColor::Random}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::Random))}
                        />
                        { "ランダム" }
                    </label>
                </div>
                <button
                    class={classes!("px-4", "py-2", "rounded", "hover:bg-blue-600", "text-white", "bg-blue-500")}
                    onclick={ctx.link().callback(|_| SettingHeaderMessage::Start)}
                >
                    { "開始" }
                </button>
            </div>
        }
    }
}
