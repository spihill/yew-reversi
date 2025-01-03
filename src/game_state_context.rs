use crate::game_logic::{
    ai_agent::{initialize_agent, renew_agent},
    game_state::GameState,
    types::{AgentType, Color, Coordinate, GameStatus},
};
use std::rc::Rc;
use yew::prelude::*;

pub type GameStateContextInner = GameState;

pub enum BoardAction {
    Move(Coordinate),
    StartGame(Color), // player color
}

impl Reducible for GameStateContextInner {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = GameState {
            board: self.board,
            turn_count: self.turn_count,
            turn: self.turn,
            status: self.status,
            ai_agent: renew_agent(&self.ai_agent),
        };
        match action {
            BoardAction::Move(coord) => {
                state.make_move(coord).unwrap();
                Rc::new(state)
            }
            BoardAction::StartGame(player_color) => {
                state.status = GameStatus::InProgress;
                state.ai_agent = Some(initialize_agent(AgentType::Random, player_color.opponent()));
                if player_color == Color::White {
                    state.make_agent_move().unwrap();
                }
                Rc::new(state)
            }
        }
    }
}

pub type GameStateContext = UseReducerHandle<GameStateContextInner>;

#[derive(Properties, Debug, PartialEq)]
pub struct BoardProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn GameStateProvider(props: &BoardProviderProps) -> Html {
    let state = use_reducer(GameState::new);

    html! {
        <ContextProvider<GameStateContext> context={state}>
            { props.children.clone() }
        </ContextProvider<GameStateContext>>
    }
}
