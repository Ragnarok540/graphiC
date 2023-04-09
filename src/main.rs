//#![deny(missing_docs)]

use piston::{WindowSettings, EventLoop, RenderEvent};
use piston::event_loop::{EventSettings, Events};
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};

mod tetris;
mod tetris_view;
mod tetris_controller;

pub use crate::tetris::{Tetris, Tetromino, Direction};
pub use crate::tetris_controller::TetrisController;
pub use crate::tetris_view::{TetrisView, TetrisViewSettings};

const BACK_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Tetris", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let tetris = Tetris::new();
    let mut tetris_controller = TetrisController::new(tetris);
    let tetris_view_settings = TetrisViewSettings::new();
    let tetris_view = TetrisView::new(tetris_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        tetris_controller.event(tetris_view.settings.position, tetris_view.settings.height, tetris_view.settings.width, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear(BACK_COLOR, g);
                tetris_view.draw(&tetris_controller, glyphs, &c, g);
            });
        }
    }

    println!("{}", settings.get_exit_on_esc());
    
    let mv = Tetromino::new('i');
    let mut mv_2 = Tetromino::new('.');
    
    for i in 0..=3 {
        for j in 0..=3 {
            let idx = Direction::rotate(i, j, Direction::South);
            *mv_2.body.vec_index_mut(idx) = *mv.body.index(i, j);
        }
    }

    println!("Display:\n{}", mv_2.body);
    println!("Debug:\n{:?}", mv_2.body);
    println!("index: {}", mv_2.body.index(1, 2));
    println!("index: {:?}", mv_2.body.row(1));
}
