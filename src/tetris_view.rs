//! Tetris view.

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use crate::tetris_controller::TetrisController;
pub use crate::tetris::Tetromino;

/// Stores tetris view settings.
pub struct TetrisViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of the tetris board along the vertical edge.
    pub height: f64,
    /// Size of the tetris board along horizontal edge.
    pub width: f64,
    /// Size of tetris blocks.
    pub block_size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Text color.
    pub text_color: Color,
    /// Yellow color.
    pub yellow_color: Color,
    /// Cyan color.
    pub cyan_color: Color,
    /// Magenta color.
    pub magenta_color: Color,
    /// Orange color.
    pub orange_color: Color,
    /// Blue color.
    pub blue_color: Color,
    /// Green color.
    pub green_color: Color,
    /// Red color.
    pub red_color: Color,
    /// Black color.
    pub black_color: Color,
}

impl TetrisViewSettings {
    /// Creates new tetris view settings.
    pub fn new() -> TetrisViewSettings {
        TetrisViewSettings {
            position: [17.0; 2],
            height: 400.0,
            width: 200.0,
            block_size: 20.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            text_color: [0.0, 0.0, 0.1, 1.0],
            yellow_color: [1.0, 1.0, 0.0, 1.0],
            cyan_color: [0.0, 1.0, 1.0, 1.0],
            magenta_color: [1.0, 0.0, 1.0, 1.0],
            orange_color: [1.0, 0.647, 0.0, 1.0],
            blue_color: [0.0, 0.0, 1.0, 1.0],
            green_color: [0.0, 1.0, 0.0, 1.0],
            red_color: [1.0, 0.0, 0.0, 1.0],
            black_color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Stores visual information about tetris.
pub struct TetrisView {
    /// Stores tetris view settings.
    pub settings: TetrisViewSettings,
}

impl TetrisView {
    /// Creates a new tetris view.
    pub fn new(settings: TetrisViewSettings) -> TetrisView {
        TetrisView {
            settings: settings,
        }
    }

    /// Draw tetris.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &TetrisController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G
    ) where C: CharacterCache<Texture = G::Texture> {
        use graphics::{Image, Line, Rectangle, Transformed};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0] + settings.board_edge_radius,
            settings.position[1] + settings.board_edge_radius,
            settings.width,
            settings.height,
        ];

        // Draw board background.
        Rectangle::new(settings.background_color).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        // Draw test blocks.
        // self.draw_block(settings.blue_color, 1.0, 1.0, c, g);
        // self.draw_block(settings.red_color, 10.0, 1.0, c, g);
        // self.draw_block(settings.green_color, 1.0, 20.0, c, g);
        // self.draw_block(settings.magenta_color, 10.0, 20.0, c, g);

        // Draw tetromino.
        let tetromino = &controller.tetris.tetromino;
        let pos = controller.tetris.position;

        for i in 0..=3 {
            for j in 0..=3 {
                if *tetromino.body.index(j, i) == 'X' {
                    let i_x = i as f64 + pos[0];
                    let j_y = j as f64 + pos[1];
                    self.draw_block(settings.blue_color, j_y, i_x, c, g);
                }
            }
        }

        // Draw loaded and invalid cell backgrounds
        // for i in 0..9 {
        //     for j in 0..9 {
        //         if controller.gameboard.cells[i][j].loaded {
        //             Self::color_cell(
        //                 settings,
        //                 [j, i],
        //                 settings.loaded_cell_background_color,
        //                 c,
        //                 g,
        //             );
        //         } else if controller.gameboard.cells[i][j].invalid {
        //             Self::color_cell(
        //                 settings,
        //                 [j, i],
        //                 settings.invalid_cell_background_color,
        //                 c,
        //                 g,
        //             );
        //         }
        //     }
        // }

        // Draw board edge.
        let boarder_rect = [
            settings.position[0],
            settings.position[1],
            settings.width + settings.board_edge_radius * 2.0,
            settings.height + settings.board_edge_radius * 2.0,
        ];

        Rectangle::new_border(
            settings.board_edge_color,
            settings.board_edge_radius,
        ).draw(boarder_rect, &c.draw_state, c.transform, g);

        // Draw characters.
        // let text_image = Image::new_color(settings.text_color);
        // let cell_size = settings.size / 9.0;
        // for j in 0..9 {
        //     for i in 0..9 {
        //         if let Some(ch) = controller.gameboard.char([i, j]) {
        //             let pos = [
        //                 settings.position[0] + i as f64 * cell_size + 15.0,
        //                 settings.position[1] + j as f64 * cell_size + 34.0
        //             ];
        //             if let Ok(character) = glyphs.character(34, ch) {
        //                 let ch_x = pos[0] + character.left();
        //                 let ch_y = pos[1] - character.top();
        //                 let text_image = text_image.src_rect([
        //                     character.atlas_offset[0],
        //                     character.atlas_offset[1],
        //                     character.atlas_size[0],
        //                     character.atlas_size[1],
        //                 ]);
        //                 text_image.draw(character.texture,
        //                                 &c.draw_state,
        //                                 c.transform.trans(ch_x, ch_y),
        //                                 g);
        //             }
        //         }
        //     }
        // }

    }

    pub fn draw_block<G: Graphics>(
        &self,
        color: Color,
        x: f64,
        y: f64,
        c: &Context,
        g: &mut G
    ) {
        use graphics::Rectangle;
        let ref settings = self.settings;
        let board_rect = [
            x * settings.block_size,
            y * settings.block_size,
            settings.block_size,
            settings.block_size,
        ];

        Rectangle::new(color).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }

}
