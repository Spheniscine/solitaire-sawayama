use async_std::stream::StreamExt;
use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, CardComponent}, game::{ANIMATION_DURATION, AnimationKey, Board, Card, DepotRole, GameState, Skin, Suit}};

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
    let clean = !st.is_busy(); // interactions should test this before write()-ing to state, to prevent slowdowns

    let animate_timer = use_coroutine(move |mut rx: UnboundedReceiver<AnimationKey>| async move {
        while let Some(key) = rx.next().await {
            async_std::task::sleep(ANIMATION_DURATION).await;
            state.write().advance_animations(key);
        }
    });

    if st.is_acting() {
        animate_timer.send(st.animation_key);
    }

    // let test_cards = (1..=24).map(|i| {
    //     Card { rank: i, suit: Suit::Spades }
    // });
    // let skin = Skin::default();


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
                skin: st.skin,
                onclick: move |pos| if clean {state.write().onclick(pos);},
                animation_key: st.animation_key,
                is_won: st.is_won(),
            }
        }
    }
}