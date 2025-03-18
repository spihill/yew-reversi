use yew::prelude::*;

use game_logic::types::Color;

use crate::game_state_context::{BoardAction, GameStateContext};

#[derive(Properties, PartialEq)]
pub struct ReversiCellProps {
    pub color: Option<Color>,
    pub x: u32,
    pub y: u32,
    pub is_valid_move: bool,
}

pub struct ReversiCell;

impl Component for ReversiCell {
    type Message = ();
    type Properties = ReversiCellProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ReversiCell
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let piece = match ctx.props().color {
            Some(Color::Black) => html! {
                <div
                    style="
                    width: 30px;
                    height: 30px;
                    background-color: black;
                    border-radius: 50%;
                    margin: auto;
                "
                />
            },
            Some(Color::White) => html! {
                <div
                    style="
                    width: 30px;
                    height: 30px;
                    background-color: white;
                    border-radius: 50%;
                    margin: auto;
                "
                />
            },
            None => html! {},
        };

        let (game_state_context, _) = ctx
            .link()
            .context::<GameStateContext>(Callback::noop())
            .unwrap();

        let onclick = if ctx.props().is_valid_move {
            let coordinate = (ctx.props().x, ctx.props().y).into();
            Callback::from(move |_| {
                game_state_context.dispatch(BoardAction::Move(coordinate));
            })
        } else {
            Callback::noop()
        };

        html! {
            <td
                class={classes!("w-10", "h-10", "border", "border-black", "bg-green-700","text-center", ctx.props().is_valid_move.then_some("cursor-pointer hover:bg-green-500"))}
                {onclick}
            >
                { piece }
            </td>
        }
    }
}
