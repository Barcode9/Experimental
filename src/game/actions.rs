//! Game action types.

/// An action a player can take in a push/fold decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Give up the pot.
    Fold,
    /// Move all-in for the effective stack.
    Push,
}

impl Action {
    /// Returns `true` when this action immediately ends the hand.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Action::Fold)
    }
}

/// A player-indexed action entry recorded in hand history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionEvent {
    pub player: usize,
    pub action: Action,
}

impl ActionEvent {
    pub const fn new(player: usize, action: Action) -> Self {
        Self { player, action }
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn fold_is_terminal() {
        assert!(Action::Fold.is_terminal());
        assert!(!Action::Push.is_terminal());
    }
}
