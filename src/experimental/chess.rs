use crate::entity::{Entity, EscalationLayer};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::fmt::Write;

/// Generates a procedural text-based chess board that degrades as the player descends.
pub struct ChessSimulator;

impl ChessSimulator {
    /// Generates a chess board based on the entity's current layer.
    #[must_use]
    pub fn generate_board(entity: &Entity, base_seed: u64) -> String {
        let interaction_seed = base_seed.wrapping_add(u64::from(entity.interaction_count()));
        let mut rng = ChaCha8Rng::seed_from_u64(interaction_seed);

        let layer = entity.layer();
        let mut output = String::new();

        match layer {
            EscalationLayer::Surface => Self::generate_surface_board(&mut output),
            EscalationLayer::Corruption => Self::generate_corruption_board(&mut output, &mut rng),
            EscalationLayer::Presence => Self::generate_presence_board(&mut output, &mut rng),
            EscalationLayer::Infection => Self::generate_infection_board(&mut output, &mut rng),
        }

        output
    }

    fn generate_surface_board(output: &mut String) {
        let _ = writeln!(output, "CHESS SIMULATOR v1.0");
        let _ = writeln!(output, "  A B C D E F G H");
        let _ = writeln!(output, "8 r n b q k b n r 8");
        let _ = writeln!(output, "7 p p p p p p p p 7");
        let _ = writeln!(output, "6 . . . . . . . . 6");
        let _ = writeln!(output, "5 . . . . . . . . 5");
        let _ = writeln!(output, "4 . . . . P . . . 4");
        let _ = writeln!(output, "3 . . . . . . . . 3");
        let _ = writeln!(output, "2 P P P P . P P P 2");
        let _ = writeln!(output, "1 R N B Q K B N R 1");
        let _ = writeln!(output, "  A B C D E F G H");
    }

    fn generate_corruption_board(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "CHESS SIMULATOR v0.9-beta");
        let pieces = [
            'p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K', '.', '?', '!',
        ];

        let _ = writeln!(output, "  A B C D E F G H");
        for row in (1..=8).rev() {
            let _ = write!(output, "{row} ");
            for _ in 0..8 {
                if rng.gen_bool(0.1) {
                    let piece = pieces[rng.gen_range(0..pieces.len())];
                    let _ = write!(output, "{piece} ");
                } else if row == 8 {
                    let _ = write!(
                        output,
                        "{} ",
                        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'][rng.gen_range(0..8)]
                    );
                } else if row == 7 {
                    let _ = write!(output, "p ");
                } else if row == 2 {
                    let _ = write!(output, "P ");
                } else if row == 1 {
                    let _ = write!(
                        output,
                        "{} ",
                        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'][rng.gen_range(0..8)]
                    );
                } else {
                    let _ = write!(output, ". ");
                }
            }
            let _ = writeln!(output, "{row}");
        }
        let _ = writeln!(output, "  A B C D E F G H");
    }

    fn generate_presence_board(output: &mut String, rng: &mut ChaCha8Rng) {
        let _ = writeln!(output, "IT IS YOUR TURN.");
        let _ = writeln!(output, "  A B C D E F G H");
        for row in (1..=8).rev() {
            let _ = write!(output, "{row} ");
            for _ in 0..8 {
                if rng.gen_bool(0.7) {
                    let _ = write!(output, "P "); // Everything is pawns moving towards you
                } else {
                    let _ = write!(output, ". ");
                }
            }
            let _ = writeln!(output, "{row}");
        }
        let _ = writeln!(output, "  A B C D E F G H");
        let _ = writeln!(output, "THEY ARE ADVANCING.");
    }

    fn generate_infection_board(output: &mut String, rng: &mut ChaCha8Rng) {
        let messages = [
            "CHECKMATE",
            "NO ESCAPE",
            "GAME OVER",
            "YOU LOST",
            "RESIGN NOW",
        ];
        let msg = messages[rng.gen_range(0..messages.len())];

        let _ = writeln!(output, "{msg}");
        for _ in 0..8 {
            let _ = write!(output, "X ");
            for _ in 0..8 {
                if rng.gen_bool(0.5) {
                    let _ = write!(output, "X ");
                } else {
                    let _ = write!(output, "  ");
                }
            }
            let _ = writeln!(output, "X");
        }
        let _ = writeln!(output, "THERE IS ONLY THE BOARD.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Entity;

    #[test]
    fn test_surface_chess() {
        let entity = Entity::new();
        let output = ChessSimulator::generate_board(&entity, 42);
        assert!(output.contains("CHESS SIMULATOR"));
        assert!(output.contains("r n b q k b n r")); // Standard starting position
    }

    #[test]
    fn test_infection_chess() {
        let mut entity = Entity::new();
        entity.update_depth(30); // Infection layer
        let output = ChessSimulator::generate_board(&entity, 42);

        // Should contain one of the horror messages
        assert!(
            output.contains("CHECKMATE")
                || output.contains("NO ESCAPE")
                || output.contains("GAME OVER")
                || output.contains("YOU LOST")
                || output.contains("RESIGN NOW")
        );
        assert!(output.contains("THERE IS ONLY THE BOARD."));
    }
}
