//! State machine for a two-player push/fold hand.

use crate::game::actions::{Action, ActionEvent};
use crate::game::params::GameParams;

/// Which player is currently the aggressor/button.
const BUTTON: usize = 0;
/// Which player is the big blind defender.
const BIG_BLIND: usize = 1;

/// Terminal hand result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terminal {
    /// Button folded and big blind wins blinds.
    ButtonFolded,
    /// Both players are all-in and equity calculation should decide winner.
    AllIn,
}

/// Current game node in the push/fold tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// Button acts first: fold or push.
    ButtonToAct,
    /// Big blind responds to a push: fold or call (represented as push).
    BigBlindToAct,
    /// Hand is complete.
    Terminal(Terminal),
}

/// Mutable hand state for a single push/fold decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub params: GameParams,
    pub node: Node,
    pub pot: u32,
    pub history: Vec<ActionEvent>,
}

impl GameState {
    /// Creates a fresh hand with blinds in the pot and button to act.
    pub fn new(params: GameParams) -> Self {
        Self {
            pot: params.initial_pot(),
            params,
            node: Node::ButtonToAct,
            history: Vec::new(),
        }
    }

    /// Applies an action at the current node.
    ///
    /// Returns `true` when the action was legal and applied.
    pub fn apply(&mut self, action: Action) -> bool {
        match self.node {
            Node::ButtonToAct => match action {
                Action::Fold => {
                    self.history.push(ActionEvent::new(BUTTON, Action::Fold));
                    self.node = Node::Terminal(Terminal::ButtonFolded);
                    true
                }
                Action::Push => {
                    self.history.push(ActionEvent::new(BUTTON, Action::Push));
                    self.pot += self.params.stack - self.params.small_blind;
                    self.node = Node::BigBlindToAct;
                    true
                }
            },
            Node::BigBlindToAct => match action {
                Action::Fold => {
                    self.history.push(ActionEvent::new(BIG_BLIND, Action::Fold));
                    self.node = Node::Terminal(Terminal::ButtonFolded);
                    true
                }
                Action::Push => {
                    self.history.push(ActionEvent::new(BIG_BLIND, Action::Push));
                    self.pot += self.params.stack - self.params.big_blind;
                    self.node = Node::Terminal(Terminal::AllIn);
                    true
                }
            },
            Node::Terminal(_) => false,
        }
    }

    /// Returns the acting player at non-terminal nodes.
    pub const fn acting_player(&self) -> Option<usize> {
        match self.node {
            Node::ButtonToAct => Some(BUTTON),
            Node::BigBlindToAct => Some(BIG_BLIND),
            Node::Terminal(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::actions::Action;
    use crate::game::params::GameParams;
    use crate::game::state::{GameState, Node, Terminal};

    #[test]
    fn starts_with_blinds_and_button_to_act() {
        let state = GameState::new(GameParams::new(20, 1, 2).unwrap());
        assert_eq!(state.pot, 3);
        assert_eq!(state.node, Node::ButtonToAct);
        assert_eq!(state.acting_player(), Some(0));
    }

    #[test]
    fn fold_ends_hand() {
        let mut state = GameState::new(GameParams::default());
        assert!(state.apply(Action::Fold));
        assert_eq!(state.node, Node::Terminal(Terminal::ButtonFolded));
        assert!(!state.apply(Action::Push));
    }

    #[test]
    fn push_call_reaches_all_in_with_expected_pot() {
        let mut state = GameState::new(GameParams::new(20, 1, 2).unwrap());

        assert!(state.apply(Action::Push));
        assert_eq!(state.pot, 22);
        assert_eq!(state.node, Node::BigBlindToAct);

        assert!(state.apply(Action::Push));
        assert_eq!(state.pot, 40);
        assert_eq!(state.node, Node::Terminal(Terminal::AllIn));
    }
}
