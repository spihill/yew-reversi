use game_logic::{
    ai_agent::{initialize_agent, renew_agent},
    game_state::GameState,
    types::{AgentType, Color, Coordinate, GameStatus},
};
use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq)]
pub struct GameStateWrapper {
    pub inner: GameState,
}

impl GameStateWrapper {
    pub fn new() -> Self {
        Self {
            inner: GameState::new(),
        }
    }
}

pub enum BoardAction {
    Move(Coordinate),
    StartGame(Color), // player color
}

impl Reducible for GameStateWrapper {
    type Action = BoardAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = GameStateWrapper {
            inner: GameState {
                board: self.inner.board,
                turn_count: self.inner.turn_count,
                turn: self.inner.turn,
                status: self.inner.status,
                ai_agent: renew_agent(&self.inner.ai_agent),
            },
        };
        match action {
            BoardAction::Move(coord) => {
                state.inner.make_move(coord).unwrap();
                Rc::new(state)
            }
            BoardAction::StartGame(player_color) => {
                state.inner.status = GameStatus::InProgress;
                state.inner.ai_agent = Some(initialize_agent(
                    AgentType::MonteCarlo,
                    player_color.opponent(),
                ));
                if player_color == Color::White {
                    state.inner.make_agent_move().unwrap();
                }
                Rc::new(state)
            }
        }
    }
}

pub type GameStateContext = UseReducerHandle<GameStateWrapper>;

#[derive(Properties, Debug, PartialEq)]
pub struct BoardProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn GameStateProvider(props: &BoardProviderProps) -> Html {
    let state = use_reducer(GameStateWrapper::new);

    html! {
        <ContextProvider<GameStateContext> context={state}>
            { props.children.clone() }
        </ContextProvider<GameStateContext>>
    }
}
