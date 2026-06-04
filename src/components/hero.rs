use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, CardComponent}, game::{Board, Card, Skin, Suit}};

#[component]
pub fn Hero() -> Element {

    let test_cards = (1..=24).map(|i| {
        (Card { rank: i, suit: Suit::Spades }, 22. + (i-1) as f32 * 6.)
    });
    let skin = Skin::default();

    let board = Board::empty();

    rsx! {
        div {
            id: "hero",
            class: "select-none",
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
                board,
                skin,
            }
        }
    }
}