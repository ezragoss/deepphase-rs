/** Round management logic
*
*
*/

use std::ops::Index;
use crate::gameplay::actors::{ResistanceActor, ResistanceAction, SuppressionActor, SuppressionAction, Action};
use crate::gameplay::round_manager::RoundResult::{ResistanceGainsPoint, ResistanceGainsTemporary};
use crate::gameplay::tilemap::{Coord, CoordinateSystem, Tile};

pub struct RoundState {
    resistance: ResistanceActor,
    suppression: SuppressionActor,
    max_turns: u32,  // the number of turns the resistance player has to win
    current_turn: u32,
    temp_turn_count: u32,  // the amount of turns a temp resistance tile has until it returns to normal
    turn_buffer: (Option<SuppressionAction>, Option<ResistanceAction>),  // the buffer processing received turns before locking them in
    turn_history: Vec<(SuppressionAction, ResistanceAction)>,
    resistance_perm_tiles: Vec<Coord>,
    resistance_temp_tiles: Vec<(Coord, u32)>,  // The coordinate and the number of turns until it returns to normal
    score_to_win: u32  // The number of resistance perm tiles needed for resistance player to win
}

#[derive(PartialEq)]
enum RoundResult {
    ResistanceGainsPoint(Coord),
    ResistanceGainsTemporary(Coord)
}

impl RoundState {
    /// Intake a suppression action to the buffer
    fn intake_suppression_action(&mut self, action: SuppressionAction) {
        self.turn_buffer.0 = Some(action);
        self.process_turn_buffer()
    }

    /// Intake a resistance action to the buffer
    fn intake_resistance_action(&mut self, action: ResistanceAction) {
        self.turn_buffer.1 = Some(action);
        self.process_turn_buffer()
    }

    /// If the turn buffer is full - initiate turn change
    fn process_turn_buffer(&mut self) {
        // Todo: More idiomatic way to do this?
        if self.turn_buffer.0 != None && self.turn_buffer.1 != None {
            self.resolve_turn()
        }
    }

    /// Resolve the current turn
    fn resolve_turn(&mut self) {
        // let results = self.round_results();
    }

    /// Get the number of neighbors still suppressed
    fn number_of_suppressed_neighbors(&self, coord: &Coord, temps: &Vec<Coord>) -> u8 {
        let total_neighbors = Tile::neighbors_of(coord);
        let surrounding_resistance: Vec<Coord> = total_neighbors.iter().cloned().filter(
            |neighbor| {
                let is_temp = temps.iter().any(|x| x.eq(neighbor));
                if self.resistance_perm_tiles.contains(neighbor) || is_temp {
                    // This coordinate is either a perm or temp resistance territory
                    return true;
                }
                return false;
            }
        ).collect();
        return (total_neighbors.len() - surrounding_resistance.len()) as u8
    }

    /// Compare a resistance and suppression action
    fn round_results(&mut self, resistance: &ResistanceAction, suppression: &SuppressionAction) -> Vec<RoundResult> {
        let mut results: Vec<RoundResult> = Vec::new();
        let mut temps: Vec<Coord> = self.resistance_temp_tiles.iter().map(|t| t.0).collect();
        let already_resistance_temp = self.resistance_temp_tiles.iter().any(|coord | coord.0 == resistance.public_coord);
        if !already_resistance_temp && !suppression.suppression_zone.contains(&resistance.public_coord) {
            // The public coordinate is outside the suppression zone, add it to the temporaries
            results.push(ResistanceGainsTemporary(resistance.public_coord));
            temps.push(resistance.public_coord)
        }

        for coord in &temps {
            let still_suppressed = self.number_of_suppressed_neighbors(&coord, &temps);
            if still_suppressed == 0 {
                // This is now a perm because it is totally surrounded
                results.push(ResistanceGainsPoint(*coord))
            }
        }

        return results
    }

    fn process_results(&mut self, results: Vec<RoundResult>) {
        for result in results {
            match result {
                RoundResult::ResistanceGainsPoint(coord) => {
                    let now_perm = coord;
                    let index = self.resistance_temp_tiles.iter().position(|x| x.0.eq(&coord)).unwrap();
                    self.resistance_temp_tiles.remove(index);
                    self.resistance_perm_tiles.push(now_perm);
                },
                RoundResult::ResistanceGainsTemporary(coord) => self.resistance_temp_tiles.push((coord, self.temp_turn_count))
            }
        };
    }

    /// Increment the relevant timers
    fn decrement_timers(&mut self) {
        // Decrement everything down to at or above 0
        for tile in &mut self.resistance_temp_tiles {
            tile.1 -= 1;
        }
        // Remove all 0s
        self.resistance_temp_tiles = self.resistance_temp_tiles.iter().filter(|tile| {
            if tile.1 == 0 {
                return false;
            }
            return true;
        }).cloned().collect();

        // Process turn count
        if self.current_turn == self.max_turns {
            println!("Implement end_game!")
            // end_game();
        } else {
            self.current_turn += 1;
        }
    }
}

impl Default for RoundState {
    fn default() -> Self {
        RoundState {
            resistance: ResistanceActor::new(),
            suppression: SuppressionActor::new(),
            max_turns: 20,
            current_turn: 0,
            temp_turn_count: 3,
            turn_buffer: (None, None),
            turn_history: Vec::new(),
            resistance_perm_tiles: Vec::new(),
            resistance_temp_tiles: Vec::new(),
            score_to_win: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gameplay::actors::{ResistanceAction, SuppressionAction};
    use crate::gameplay::round_manager::{RoundResult, RoundState};
    use crate::gameplay::tilemap::{Coord, CoordinateSystem, Tile};

    /** Todo: Would be cool to have a macro like matches! but over an iterable for any-like query
    macro_rule! matches_any {
        ($iter:item, $e:expr) => {

        }
    }
    */

    /// Tests that round_results will not generate any results for successful suppression
    #[test]
    fn test_round_results_successful_suppression() {
        let mut state = RoundState::default();
        let resistance_action = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (0, 1)
        };
        let suppression_action = SuppressionAction {
            suppression_zone: vec![
                (0, 0), (0, 1), (1, 0), (1, 1)
            ]
        };
        let results = state.round_results(&resistance_action, &suppression_action);
        assert_eq!(results.len(), 0);
    }

    /// Tests whether round_results will generate the correct enum for a successful temp resistance
    #[test]
    fn test_round_results_successful_resistance_placement() {
        let mut state = RoundState::default();
        let resistance_action = ResistanceAction {
            public_coord: (0, 0),
            private_coord: (0, 1)
        };
        let suppression_action = SuppressionAction {
            suppression_zone: vec![
                (1, 1), (1, 1), (2, 0), (2, 2)
            ]
        };
        let results = state.round_results(&resistance_action, &suppression_action);
        assert_eq!(results.len(), 1);
        if let RoundResult::ResistanceGainsTemporary(coord) = results.last().unwrap() {
            assert_eq!(coord, &resistance_action.public_coord)
        }
    }

    /// Tests that round_results will generate the correct results for surround a temp tile
    #[test]
    fn test_round_results_resistance_gains_point() {
        let mut state = RoundState::default();
        let control_coord = ((0, 0), state.temp_turn_count);
        let mut neighbors_of_control: Vec<(Coord, u32)> = Tile::neighbors_of(&control_coord.0).iter().map(
            |neighbor| (*neighbor, state.temp_turn_count)
        ).collect();
        let final_neighbor = neighbors_of_control.pop().unwrap().0;
        state.resistance_temp_tiles.push(control_coord);
        state.resistance_temp_tiles.extend(neighbors_of_control.iter());
        let resistance_action = ResistanceAction {
            public_coord: final_neighbor,
            private_coord: (0, 0)
        };
        // Something clearly out of the way
        let suppression_action = SuppressionAction {
            suppression_zone: vec![
                (-10, -10), (-10, 9), (-9, -9), (-9, -10)
            ]
        };
        let results = state.round_results(&resistance_action, &suppression_action);
        assert!(results.iter().any(|result| *result == RoundResult::ResistanceGainsPoint(control_coord.0)));
    }

    /// Tests that process_results will create a temp tile with the correct timer
    #[test]
    fn test_process_results_resistance_gains_temp() {
        let mut state = RoundState::default();
        let control_coord = (0, 0);
        assert!(!state.resistance_temp_tiles.iter().any(|coord| coord.0 == control_coord));
        let results: Vec<RoundResult> = vec![
            RoundResult::ResistanceGainsTemporary(control_coord)
        ];
        state.process_results(results);
        assert!(state.resistance_temp_tiles.contains(&(control_coord, state.temp_turn_count)));
    }

    /// Tests that process_results will move a point from temp to perm given correct result
    #[test]
    fn test_process_results_resistance_gains_point() {
        let mut state = RoundState::default();
        let control_coord = (0, 0);
        state.resistance_temp_tiles.push((control_coord, state.temp_turn_count));
        let results: Vec<RoundResult> = vec![
            RoundResult::ResistanceGainsPoint(control_coord)
        ];
        assert!(!state.resistance_perm_tiles.contains(&control_coord));
        state.process_results(results);
        assert!(!state.resistance_temp_tiles.iter().any(|coord| coord.0 == control_coord));
        assert!(state.resistance_perm_tiles.contains(&control_coord));
    }

    /// Tests that decrement_timers progresses the turn count
    #[test]
    fn test_decrement_timers_progresses_turn() {
        let mut state = RoundState::default();
        state.decrement_timers();
        assert_eq!(1, state.current_turn)
    }

    /// Tests that decrement_timers removes timed out tiles
    #[test]
    fn test_decrement_timers_removes_timed_out_tiles() {
        let mut state = RoundState::default();
        let temp_tiles: Vec<(Coord, u32)> = vec![
            ((0, 0), 1),
            ((0, 1), 2),
            ((0, 2), 3)
        ];
        state.resistance_temp_tiles.extend(temp_tiles);
        state.decrement_timers();
        assert_eq!(state.resistance_temp_tiles.len(), 2);
        state.decrement_timers();
        assert_eq!(state.resistance_temp_tiles.len(), 1);
    }

}