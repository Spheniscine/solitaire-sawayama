use std::ops::Range;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game::NUM_SUITS;


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
}

pub const NUM_DEPOTS: usize = DepotRole::Tableau.offset() + DepotRole::Tableau.number_of();