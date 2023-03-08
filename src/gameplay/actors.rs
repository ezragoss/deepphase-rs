/** Handles game logic for a given round
*
*
 */

use crate::gameplay::tilemap::{Coord};
use std::vec;

/// Convenience abstraction for using the two different actions in generics.
pub(crate) trait Action {}

/// A single round action taken by the resistance player.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub(crate) struct ResistanceAction {
    pub public_coord: Coord,
    pub private_coord: Coord
}
impl Action for ResistanceAction {}
impl PartialEq for ResistanceAction {
    fn eq(&self, other: &Self) -> bool {
        self.public_coord == other.public_coord && self.private_coord == other.private_coord
    }
}

/// A single round action taken by the suppressing player
#[derive(Debug)]
#[derive(Clone)]
pub(crate) struct SuppressionAction {
    pub suppression_zone: Vec<Coord>
}
impl Action for SuppressionAction {}
impl PartialEq for SuppressionAction {
    fn eq(&self, other: &Self) -> bool {
        self.suppression_zone == other.suppression_zone
    }
}

/// The base actor trait shared by the two player types.
trait Actor<ActionType: Action> {
    fn take_action(&mut self, action: ActionType) {
        self.writable_action_queue().push(action)
    }
    fn last_action(&self) -> Option<&ActionType> {
        self.action_queue().last()
    }
    fn undo_action(&mut self) -> Option<&ActionType> {
        self.writable_action_queue().pop();
        return self.action_queue().last();
    }
    fn action_queue(&self) -> &Vec<ActionType>;
    fn writable_action_queue(&mut self) -> &mut Vec<ActionType>;
}

#[derive(Debug)]
pub(crate) struct ResistanceActor {
    action_queue: Vec<ResistanceAction>
}

impl ResistanceActor {
    /// Proxies the default constructor for the action queue.
    pub fn new() -> ResistanceActor {
        Self { action_queue: Vec::new() }
    }
}

impl Actor<ResistanceAction> for ResistanceActor {
    /// Returns this actor's action queue.
    fn action_queue(&self) -> &Vec<ResistanceAction> {
        &self.action_queue
    }

    /// Returns this actor's action queue in a mutable state
    fn writable_action_queue(&mut self) -> &mut Vec<ResistanceAction> {
        &mut self.action_queue
    }
}

pub struct SuppressionActor {
    action_queue: Vec<SuppressionAction>
}

impl SuppressionActor {
    /// Proxies the default constructor for the action queue.
    pub fn new() -> Self {
        Self { action_queue: Vec::new() }
    }
}

impl Actor<SuppressionAction> for SuppressionActor {
    fn action_queue(&self) -> &Vec<SuppressionAction> {
        &self.action_queue
    }
    fn writable_action_queue(&mut self) -> &mut Vec<SuppressionAction> {
        &mut self.action_queue
    }
}

#[cfg(test)]
mod tests {

    use crate::gameplay::actors::{ResistanceActor, ResistanceAction, Actor, Coord};

    #[test]
    pub fn test_resistance_action_eq() {
        let action_a = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (0, 0)
        };
        let action_b = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (0, 0)
        };
        let action_c = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (1, 0)
        };
        assert_eq!(action_a, action_b);
        assert_ne!(action_a, action_c);
    }

    #[test]
    pub fn test_take_action() {
        let mut resistance = ResistanceActor::new();
        let action = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (1, 0)
        };

        resistance.take_action(action);
        let mut control: Vec<ResistanceAction> = Vec::new();
        control.push(action.clone());
        assert_eq!(resistance.action_queue, control)
    }

    #[test]
    pub fn test_undo_action() {
        let mut resistance = ResistanceActor::new();
        let action_a = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (1, 0)
        };
        let action_b = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (0, 0)
        };
        let mut control: Vec<ResistanceAction> = Vec::new();
        control.push(action_a);

        resistance.take_action(action_a);
        resistance.take_action(action_b);
        let should_be_a = resistance.undo_action().unwrap();
        assert_eq!(*should_be_a, action_a);
        assert_eq!(*resistance.action_queue(), control);
    }
}