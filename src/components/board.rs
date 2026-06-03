use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardFrame, SkinTrait, rem}, game::{AnimationKey, Board, BoardPos, Card, DepotRole, NUM_DEPOTS, Skin, Suit}};

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
    let spacer = 1.4f32;

    let center_x = |n: usize, i: usize| 
        50. - (card_width * n as f32 + spacer * (n-1) as f32) / 2. + (card_width + spacer) * i as f32;

    let start_y = 2f32;
    let pos_y = |i: usize| start_y + (card_height + spacer) * i as f32;
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
                Vec2::new(center_x(num_grid_columns, DepotRole::Tableau.number_of()), pos_y(0)),
            DepotRole::Tableau => 
                Vec2::new(center_x(num_grid_columns, index), pos_y(1)),
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

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

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
            }
        }
    }
}