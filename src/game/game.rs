use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_RANKS, RANKS, Skin, Suit};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos, auto: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub already_won: bool,
    pub num_wins: i32,

    // pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub auto_play: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let deal = Self::new_deal(&mut rand::rng());
        let board = Board::from_deal(&deal);
        Self {
            board,
            deal,
            animation_key: 0,
            history: vec![],
            already_won: false,
            num_wins: 0,
            allow_undo: true,
            auto_play: true,
            skin: Skin::default(),
        }
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back.suit.color() != front.suit.color() && front.rank + 1 == back.rank
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::Foundation => false,
            DepotRole::FreeCell => { slice.len() <= 1 },
            DepotRole::Stock => false,
            DepotRole::Waste => { slice.len() <= 1 },
            DepotRole::Tableau => {
                slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
            },
        }
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn is_won(&self) -> bool {
        DepotRole::Foundation.range().all(|i| {
            self.board.depots[i].len() == NUM_RANKS
        })
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if DepotRole::role(pos.depot_index) == Some(DepotRole::Stock) {
            let mut src = self.board.top_pos(pos.depot_index);
            src.card_index = src.card_index.saturating_sub(3);
            let dest = self.board.top_pos(DepotRole::Waste.id(0));
            self.board.do_move(src, dest);
            self.history.push(ActionRecord { pos1: src, pos2: dest, auto: false });
            return;
        }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            // let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            // if !self.can_move(src, dest) { return; }
            // self.board.do_move(src, dest);
            // self.history.push(ActionRecord { pos1: src, pos2: dest, auto: false });
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        // if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    
}