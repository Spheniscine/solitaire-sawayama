use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, CardComponent}, game::{Board, Card, DepotRole, GameState, Skin, Suit}};

#[component]
pub fn Hero() -> Element {
    let mut state = use_signal(|| {
        // if let Some(mut state) = LocalStorage.load_game_state() {
        //     state.board.selected = None;
        //     state.screen_state = ScreenState::Game;
        //     return state;
        // }
        GameState::init()
    });

    let st = state.read();

    let test_cards = (1..=24).map(|i| {
        Card { rank: i, suit: Suit::Spades }
    });
    let skin = Skin::default();


    rsx! {
        div {
            id: "hero",
            class: "select-none",
            overflow: "hidden",
            // for (c, y) in test_cards {
            //     CardComponent {
            //         position: Vec2 { x: 2., y },
            //         width: 11.,
            //         card: c,
            //         skin,
            //     }
            // }

            // CardComponent {
            //     position: Vec2 { x: 40., y: 40. },
            //     width: 11.,
            //     skin,
            //     number_hint: 24,
            // }

            BoardComponent { 
                position: Vec2 { x: 0., y: 20. },
                board: st.board.clone(),
                skin,
            }
        }
    }
}