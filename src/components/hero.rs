use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::CardComponent, game::{Card, Skin, Suit}};

#[component]
pub fn Hero() -> Element {

    let test_cards = (1..=24).map(|i| {
        (Card { rank: i, suit: Suit::Spades }, 22. + (i-1) as f32 * 6.)
    });
    let skin = Skin::default();

    rsx! {
        div {
            id: "hero",
            for (c, y) in test_cards {
                CardComponent {
                    position: Vec2 { x: 2., y },
                    width: 11.,
                    card: c,
                    skin,
                }
            }

            div {
                position: "absolute",
                top: "40rem",
                left: "40rem",
                div {
                    style: "place-items: center",
                    display: "grid",
                    background: "#fff",
                    width: "11rem",
                    height: "12rem",
                    border: "0.25rem solid #000",
                    border_radius: "1.5rem",

                    div {
                        class: "card-pattern-1",
                        width: "9.75rem",
                        height: "10.75rem",
                        border_radius: "1rem",
                        //padding: "1rem",
                    }
                }
            }
        }
    }
}