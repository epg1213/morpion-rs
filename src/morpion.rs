use std::{fmt, io};

use rand::seq::IndexedRandom;

pub fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);
    let mut chars = buffer.chars();
    chars.next_back();
    chars.as_str().to_string()
}

#[derive(Debug, Clone, PartialEq)]
pub enum Player {
    Cross,
    Circle,
    None
}

impl Player {
    fn swap(&mut self) {
        if *self == Player::Cross {
            *self = Player::Circle;
        } else if *self == Player::Circle {
            *self = Player::Cross;
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::None => write!(f, " "),
            Player::Cross => write!(f, "X"),
            Player::Circle => write!(f, "O")
        }
    }
}

#[derive(Debug)]
pub struct Morpion {
    grid: Vec<Player>,
    autoplay: bool,
    current_player: Player
}

impl Morpion {
    pub fn new() -> Self {
        Self {
            grid: vec![Player::None; 9],
            autoplay: false,
            current_player: Player::Cross
        }
    }

    pub fn set_autoplay(&mut self) {
        self.autoplay=true;
    }

    pub fn get_empty_squares(&self) -> Vec<usize> {
        let mut empty_sq = Vec::<usize>::new();
        for (i, c) in self.grid.iter().enumerate() {
            if *c == Player::None {
                empty_sq.push(i);
            }
        }
        empty_sq
    }

    pub fn place(&mut self, index: usize) -> bool {
        if self.get_empty_squares().contains(&index) {
            if let Some(square) = self.grid.get_mut(index) {
                *square = self.current_player.clone();
                return true
            }
        }
        false
    }

    pub fn place_rand(&mut self) -> bool {
        let index: usize;
        match self.get_empty_squares().choose(&mut rand::rng()) {
            Some(v) => index = *v,
            None => return false
        }
        self.place(index)
    }

    pub fn ask_user(&mut self) {
        loop {
            match get_input().parse::<usize>() {
                Ok(parsed_input) => {
                    if self.get_empty_squares().contains(&parsed_input) {
                        self.place(parsed_input);
                        return;
                    }
                },
                Err(_) => {}
            }
            println!("Please enter a valid index: {:?}", self.get_empty_squares());
        }
    }

    // 0 1 2
    // 3 4 5
    // 6 7 8
    pub fn check_winner(&self) -> Option<Player> {
        for alignment in [ // check every line, column and diagonal
            [0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 4, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8], [2, 4, 6],
        ] {
            if self.grid.get(alignment[0]).unwrap_or(&Player::None) == &Player::Cross &&
            self.grid.get(alignment[1]).unwrap_or(&Player::None) == &Player::Cross &&
            self.grid.get(alignment[2]).unwrap_or(&Player::None) == &Player::Cross {
                return Some(Player::Cross);
            }
            if self.grid.get(alignment[0]).unwrap_or(&Player::None) == &Player::Circle &&
            self.grid.get(alignment[1]).unwrap_or(&Player::None) == &Player::Circle &&
            self.grid.get(alignment[2]).unwrap_or(&Player::None) == &Player::Circle {
                return Some(Player::Circle);
            }
        }
        if self.get_empty_squares().is_empty() {
            return Some(Player::None) // draw
        }
        None // no winner yet
    }

    pub fn next(&mut self) -> Option<Player> { // returns the winner if there is one
        if self.autoplay && self.current_player == Player::Circle {
            self.place_rand();
        } else {
            self.ask_user();
        }
        self.current_player.swap();
        self.check_winner()
    }
}
impl fmt::Display for Morpion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = String::new();
        for (i, c) in self.grid.iter().enumerate() {
            if i%3==2 {
                content.push_str(format!("| {} |\n", c).as_str());
            } else {
                content.push_str(format!("| {} ", c).as_str());
            }
        }
        write!(f, "{}", content)
    }
}

