use dioxus::prelude::*;

use crate::{components::{CardText, VIDEO_GAMEPLAY, rem}, game::{Card, ColorSkin, GameState, ScreenState, Suit}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let skin = st.skin;

    let stack_example = || {
        let mut ite = [
            Card { rank: 5, suit: Suit::Spades },
            Card { rank: 4, suit: Suit::Hearts },
            Card { rank: 3, suit: Suit::Clubs },
            Card { rank: 2, suit: Suit::Hearts },
        ].into_iter().map(|card| {
            rsx! {
                CardText { 
                    card, skin, color_mode: crate::game::ColorMode::Light,
                }
            }
        });


        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on the ", {last}
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4.25rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The ",Emph {"tableau"}," consists of 7 columns. Cards in the tableau are stacked by descending ranks of 
                    alternating color (",
                    {if skin.colors == ColorSkin::TwoColor {"red/black"} else {"warm/cool"}}
                    ,"). Such stacks of any size can be moved as a unit. (e.g. ",{stack_example()}")"
                }

                p {
                    Emph {"NOTE:"}, " To move cards, click to select a card or stack, then click the destination. ", Emph{"“Drag and drop” is not required."}
                }

                p {
                    "Any card or stack may be moved into an empty tableau column."
                }

                p {
                    "Click on the ",Emph{"stock"}," to deal three cards to the ",Emph{"waste"},". Only the frontmost card of the waste can be moved."
                }

                p {
                    "When the stock runs out, it leaves behind a ",Emph{"free cell"}," that may be used to store a single card."
                }

                p {
                    "To ",Emph{"win the game"},", stack all the cards to the foundations in ascending order by suit."
                }

                p {
                    Emph{"Shortcut note:"}," Double-clicking on a card will try to move it to the foundations if possible, or the free cell otherwise."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",

                    a {
                        href: VIDEO_GAMEPLAY,
                        target: "_blank",
                        text_decoration: "none",
                        margin_right: rem(4.),
                        div {
                            width: rem(30.),
                            position: "relative",
                            class: "game-button",
                            "Example video"
                        }
                    }

                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                        "Back to game"
                    }
                }
            }
        }
    }
}