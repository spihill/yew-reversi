use crate::{
    components::reversi_cell::ReversiCell, game_logic::types::GameStatus,
    game_state_context::GameStateContext,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ReversiTableProps {}

pub struct ReversiTable {
    game_state: GameStateContext,
    _context_listener: ContextHandle<GameStateContext>,
}

pub enum ReversiTableMessage {
    Update(GameStateContext),
}

impl Component for ReversiTable {
    type Message = ReversiTableMessage;
    type Properties = ReversiTableProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (board, context_listner) = ctx
            .link()
            .context::<GameStateContext>(ctx.link().callback(ReversiTableMessage::Update))
            .unwrap();
        ReversiTable {
            game_state: board,
            _context_listener: context_listner,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ReversiTableMessage::Update(game_state) => {
                self.game_state = game_state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let valid_moves = self.game_state.valid_moves();
        html! {
            <table style="border-collapse: collapse;">
                <tbody>
                    { for self.game_state.board.as_array().iter().enumerate().map(|(y, row)| {
                            html! {
                                <tr key={y}>
                                    {
                                        for row.iter().enumerate().map(|(x, &cell)| {
                                            html! {
                                                <ReversiCell key={x} color={cell} x={x as u32} y={y as u32} is_valid_move={
                                                    self.game_state.status == GameStatus::InProgress &&  valid_moves.iter().any(|&coord| coord == (x as u32, y as u32).into())
                                                }/>
                                            }
                                        })
                                    }
                                </tr>
                            }
                        }) }
                </tbody>
            </table>
        }
    }
}
