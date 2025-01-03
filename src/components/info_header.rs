use yew::prelude::*;

use crate::{
    game_logic::types::Color,
    setting_context::{SettingAction, SettingContext},
};

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
    selected_color: Option<TurnColor>,
    show: bool,
}

impl Component for SettingHeader {
    type Message = SettingHeaderMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SettingHeader {
            selected_color: None,
            show: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SettingHeaderMessage::SetColor(color) => {
                self.selected_color = Some(color);
            }
            SettingHeaderMessage::Start => {
                if let Some(color) = self.selected_color {
                    let (setting_context, _) = ctx
                        .link()
                        .context::<SettingContext>(Callback::noop())
                        .unwrap();
                    setting_context.dispatch(SettingAction::SetPlayerColor(match color {
                        TurnColor::Black => Some(Color::Black),
                        TurnColor::White => Some(Color::White),
                        TurnColor::Random => None,
                    }));
                    self.show = false;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.show {
            return html! {};
        }

        let disabled = self.selected_color.is_none();

        html! {
            <div class="p-4 rounded">
                <h2 class="text-lg font-bold mb-2">{ "設定" }</h2>
                <div class="mb-4">
                    <label class="mr-4">
                        <input
                            type="radio"
                            name="turn"
                            value="black"
                            checked={self.selected_color == Some(TurnColor::Black)}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::Black))}
                        />
                        { "黒" }
                    </label>
                    <label class="mr-4">
                        <input
                            type="radio"
                            name="turn"
                            value="white"
                            checked={self.selected_color == Some(TurnColor::White)}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::White))}
                        />
                        { "白" }
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="turn"
                            value="random"
                            checked={self.selected_color == Some(TurnColor::Random)}
                            onchange={ctx.link().callback(|_| SettingHeaderMessage::SetColor(TurnColor::Random))}
                        />
                        { "ランダム" }
                    </label>
                </div>
                <button
                    class={classes!("px-4", "py-2", "rounded", "hover:bg-blue-600", "text-white", "bg-blue-500", disabled.then_some("opacity-50 cursor-not-allowed hover:bg-blue-500"))}
                    onclick={ctx.link().callback(|_| SettingHeaderMessage::Start)}
                    {disabled}
                >
                    { "開始" }
                </button>
            </div>
        }
    }
}
