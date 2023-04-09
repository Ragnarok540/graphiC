//! Tetris logic.

use std::fs::read_to_string;

/// Horizontal size of the board.
const H_SIZE: usize = 10;
/// Vertical size of the board.
const V_SIZE: usize = 20;

/// Stores information for a single `Gameboard` cell
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub value: u8,
    //pub falling: bool,
    //pub locked: bool,
}

/// Stores game board information.
#[derive(Debug, PartialEq)]
pub struct Tetris {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[Cell; H_SIZE]; V_SIZE],
    /// Current Tetromino.
    pub tetromino: Tetromino,
    /// Position of the current Tetromino.
    pub position: [f64; 2],
}

impl Tetris {
    /// Creates a new game board.
    pub fn new() -> Tetris {
        Tetris {
            cells: [[Cell::default(); H_SIZE]; V_SIZE],
            tetromino: Tetromino::new('z'),
            position: [0.0, 4.0],
        }
    }
 
    /// Horizontal move.
    pub fn h_move(&mut self, dir: char) {
        match dir {
            'l' => self.position[1] = self.position[1] - 1.0,
            'r' => self.position[1] = self.position[1] + 1.0,
            _ => {}
        }
    }

    /// Vertical move.
    pub fn v_move(&mut self) {
        self.position[0] = self.position[0] + 1.0;
    }
}

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
     North,
     East,
     South,
     West,
}

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn counterclockwise(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn rotate(x: usize, y: usize, dir: Direction) -> usize {
        match dir {
            Direction::North => y * 4 + x,
            Direction::West => 12 + y - (x * 4),
            Direction::South => 15 - (y * 4) - x,
            Direction::East => 3 - y + (x * 4),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tetromino {
    pub body: Vec2d<char>,
    pub t_type: char,
}

impl Tetromino {
    pub fn new(t_type: char) -> Self {
        let body = match t_type {
            'o' => Tetromino::o_tetromino(),
            'i' => Tetromino::i_tetromino(),
            't' => Tetromino::t_tetromino(),
            'l' => Tetromino::l_tetromino(),
            'j' => Tetromino::j_tetromino(),
            's' => Tetromino::s_tetromino(),
            'z' => Tetromino::z_tetromino(),
            '.' => Tetromino::empty_tetromino(),
            _ => Tetromino::empty_tetromino(),
        };
        Self {body, t_type}
    }

    pub fn empty_tetromino() -> Vec2d<char> {
        Vec2d::new(vec!['.'; 16], 4, 4)
    }

    pub fn o_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            '.', 'X', 'X', '.',
            '.', 'X', 'X', '.',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn i_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            'X', 'X', 'X', 'X',
            '.', '.', '.', '.',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn t_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            '.', '.', 'X', '.',
            '.', 'X', 'X', 'X',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn l_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            '.', '.', 'X', '.',
            'X', 'X', 'X', '.',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn j_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            '.', 'X', '.', '.',
            '.', 'X', 'X', 'X',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn s_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            '.', '.', 'X', 'X',
            '.', 'X', 'X', '.',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }

    pub fn z_tetromino() -> Vec2d<char> {
        let v = [
            '.', '.', '.', '.',
            'X', 'X', '.', '.',
            '.', 'X', 'X', '.',
            '.', '.', '.', '.',
        ];
        Vec2d::new(v.to_vec(), 4, 4)
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    col: usize,
    row: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, col: usize, row: usize) -> Self {
        assert!(vec.len() == col * row);
        Self { vec, col, row }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.col * row;
        &self.vec[i..(i + self.col)]
    }

    pub fn index(&self, col: usize, row: usize) -> &T {
        let i = self.col * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, col: usize, row: usize) -> &mut T {
        let i = self.col * row;
        &mut self.vec[i + col]
    }

    pub fn vec_index(&self, index: usize) -> &T {
        &self.vec[index]
    }

    pub fn vec_index_mut(&mut self, index: usize) -> &mut T {
        &mut self.vec[index]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.row {
            if i != 0 {
                str.push_str(",\n");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}

// pub struct Game {
    // matrix: Vec2d<u32>,
    // current_piece: Tetromino,
    // current_direction: Direction,
    // current_x: usize,
    // current_y: usize,
    // height: usize,
    // width: usize,
    // game_over: bool,
    // waiting_time: f64,
// }

// impl Game {
    // pub fn new(width: usize, height: usize) -> Game {
    //     Game {
    //         matrix: Vec2d::new(vec![0; width * height], width, height),
    //         current_piece: Tetromino::new('z'),
    //         current_direction: Direction::North,
    //         current_x: width / 2,
    //         current_y: 0,
    //         height,
    //         width,
    //         game_over: false,
    //         waiting_time: 0.0
    //     }
    // }


    // pub fn update(&mut self, delta_time: f64) {
    //     self.waiting_time += delta_time;

    //     if self.game_over {
    //         if self.waiting_time > RESTART_TIME {
    //             self.restart();
    //         }
    //         return;
    //     }

    //     //let fit = self.does_piece_fit();

    //     //println!("fits {}", fit);


    //     if self.waiting_time > MOVING_PERIOD {
    //         //self.update_snake(None);

    //         self.waiting_time = 0.0;
    //     }
    // }

    // fn generate_piece(&mut self) {
    //     self.current_piece = Tetromino::new('z');
    //     self.current_direction = Direction::North;
    // }

    // fn does_piece_fit(&mut self) -> bool {
    //     for i in 0..=3 {
    //         for j in 0..=3 {
    //             // index in piece
    //             let idx = Direction::rotate(i, j, self.current_direction);
    //             //index in field
    //             let idf = (self.current_y + i) * self.width + (self.current_x + j);
    //             println!("index 1: {}", idx);
    //             println!("index 2: {}", idf);
    //             //if self.current_piece.body.vec_index(idx) == 'X' && self.matrix.vec_index(idf) != '.' {
    //             //    return false
    //             //}
    //         }
    //     }
    //     return true
    // }
