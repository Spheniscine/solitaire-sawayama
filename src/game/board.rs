use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game::{Card, NUM_SUITS};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, EnumIter)]
pub enum DepotRole {
    Foundation,
    FreeCell,
    Stock,
    Waste,
    Tableau,
}

impl DepotRole {
    pub const fn number_of(&self) -> usize {
        match self {
            DepotRole::Foundation => NUM_SUITS,
            DepotRole::FreeCell => 1,
            DepotRole::Stock => 1,
            DepotRole::Waste => 1,
            DepotRole::Tableau => 7,
        }
    }

    pub const fn offset(&self) -> usize {
        use DepotRole::*;
        match self {
            Foundation => 0,
            FreeCell => Foundation.number_of(),
            Stock => Foundation.number_of() + FreeCell.number_of(),
            Waste => Foundation.number_of() + FreeCell.number_of() + Stock.number_of(),
            Tableau => Foundation.number_of() + FreeCell.number_of() + Stock.number_of() + Waste.number_of(),
        }
    }

    pub const fn range(&self) -> Range<usize> {
        self.offset() .. self.offset() + self.number_of()
    }

    pub fn role_and_subindex(i: usize) -> Option<(DepotRole, usize)> {
        for role in Self::iter() {
            if role.range().contains(&i) {
                return Some((role, i - role.offset()))
            }
        }
        None
    }

    pub fn role(i: usize) -> Option<DepotRole> {
        Self::role_and_subindex(i).map(|x| x.0)
    }

    pub fn id(&self, i: usize) -> usize {
        self.offset() + i
    }

    pub fn is_face_up(&self) -> bool {
        *self != DepotRole::Stock
    }
}

pub const NUM_DEPOTS: usize = DepotRole::Tableau.offset() + DepotRole::Tableau.number_of();

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

impl BoardPos {
    pub fn new(depot_index: usize, card_index: usize) -> Self {
        Self { depot_index, card_index }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AnimationAct {
    Move(Vec<Card>, BoardPos, BoardPos),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub selected: Option<BoardPos>,
    pub animation_acts: Vec<AnimationAct>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            depots: vec![vec![]; NUM_DEPOTS],
            selected: None,
            animation_acts: vec![],
        }
    }
}