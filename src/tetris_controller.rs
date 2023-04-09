//! Tetris controller.

use piston::GenericEvent;

use crate::Tetris;

/// Handles events for Sudoku game.
pub struct TetrisController {
    /// Stores the tetris state.
    pub tetris: Tetris,
}

impl TetrisController {
    /// Creates a new tetris controller.
    pub fn new(tetris: Tetris) -> TetrisController {
        TetrisController {
            tetris: tetris,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], height: f64, width: f64, e: &E) {
        use piston::input::{Button, Key};
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left => self.tetris.h_move('l'),
                Key::Right => self.tetris.h_move('r'),
                Key::Down => self.tetris.v_move(),
                Key::Up => self.tetris.tetromino.rotate_tetromino('r'),
                Key::Space => self.tetris.tetromino.rotate_tetromino('l'),
                _ => {}
            }
        }
    }
}
