use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardComponent, CardFrame, SkinTrait, rem}, game::{AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, Skin, Suit}};

#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    #[props(default)]
    onclick: EventHandler<BoardPos>,
    #[props(default)]
    ondoubleclick: EventHandler<BoardPos>,
    #[props(default)]
    animation_key: AnimationKey,
) -> Element {
    let card_width = 11f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer_x = 1.1f32;
    let spacer_y = 1.5f32;

    let center_x = |n: usize, i: usize| 
        50. - (card_width * n as f32 + spacer_x * (n-1) as f32) / 2. + (card_width + spacer_x) * i as f32;

    let start_y = 2f32;
    let pos_y = |i: usize| start_y + (card_height + spacer_y) * i as f32;
    let column_card_offset = Vec2::new(0., 6.);

    let num_grid_columns = DepotRole::Tableau.number_of() + 1;

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotRole::role_and_subindex(depot).unwrap();
        match role {
            DepotRole::Foundation => 
                Vec2::new(center_x(num_grid_columns, index), pos_y(0)),
            DepotRole::FreeCell | DepotRole::Stock => 
                Vec2::new(center_x(num_grid_columns, DepotRole::Tableau.number_of()-1), pos_y(0)),
            DepotRole::Waste => 
                Vec2::new(center_x(num_grid_columns, DepotRole::Tableau.number_of()), pos_y(0)) + column_card_offset * ord as f32,
            DepotRole::Tableau => 
                Vec2::new(center_x(num_grid_columns, index), pos_y(1)) + column_card_offset * ord as f32,
        }
    };

    let get_hint = |depot: usize| {
        let role = DepotRole::role(depot).unwrap();
        match role {
            DepotRole::Foundation => 
                Some(skin.render_rank(&Card { rank: 1, suit: Suit::Spades })),
            DepotRole::FreeCell => 
                Some(
                    rsx!{
                        span {
                            font_family: "'Noto Sans Symbols 2'",
                            position: "relative",
                            top: "0.12em",
                            "✽"
                        }
                    }
                ),
            DepotRole::Stock =>
                None,
            DepotRole::Waste => 
                None,
            DepotRole::Tableau => 
                Some(rsx!{})
        }
    };

    let is_face_up = |depot: usize| {
        DepotRole::role(depot).unwrap().is_face_up()
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotRole::role(depot_index).unwrap() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let waste_background_x = center_x(num_grid_columns, DepotRole::Tableau.number_of()) - spacer_x * 0.8;

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            div {
                position: "absolute",
                top: rem(1.),
                left: rem(waste_background_x),
                width: rem(100. - waste_background_x),
                height: rem(160.),
                background_color: "#006622",
            }

            for depot in 0..NUM_DEPOTS {
                if let Some(hint) = get_hint(depot) {
                    CardFrame { 
                        position: get_pos(depot, 0),
                        width: card_width,
                        hint,
                        onclick: move |_| {
                            onclick.call(BoardPos { depot_index: depot, card_index: !0 })
                        },
                    }
                }

                for i in 0..board.depots[depot].len() {
                    CardComponent { 
                        position: get_pos(depot, i),
                        width: card_width,
                        card: if is_face_up(depot) {board.depots[depot][i]},
                        number_hint: if !is_face_up(depot) {i + 1},
                        skin,
                        onclick: move |_| {
                            onclick.call(BoardPos { depot_index: depot, card_index: i })
                        },
                        ondoubleclick: move |_| {
                            ondoubleclick.call(BoardPos { depot_index: depot, card_index: i })
                        },
                    }
                }
            }
        }
    }
}