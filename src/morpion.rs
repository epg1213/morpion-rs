use std::{fmt, io};

use rand::seq::IndexedRandom;

fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer);
    let mut chars = buffer.chars();
    chars.next_back();
    chars.as_str().to_string()
}

#[derive(Debug)]
pub struct Morpion {
    grid: Vec<char>,
    autoplay: bool,
    current_player: char
}
// ' ', 'X', 'O'
// 0 1 2
// 3 4 5
// 6 7 8

//  0  1  2  3
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15

impl Morpion {
    pub fn new(autoplay: bool) -> Self {
        Self {
            grid: vec![' '; 9],
            autoplay: autoplay,
            current_player: 'X'
        }
    }

    pub fn get_empty_squares(&self) -> Vec<usize> {
        let mut empty_sq = Vec::<usize>::new();
        for (i, c) in self.grid.iter().enumerate() {
            if *c == ' ' {
                empty_sq.push(i);
            }
        }
        empty_sq
    }

    pub fn place(&mut self, character: char, index: usize) -> bool {
        if self.get_empty_squares().contains(&index) {
            if let Some(square) = self.grid.get_mut(index) {
                *square = character;
                return true
            }
        }
        false
    }

    pub fn place_rand(&mut self, character: char) -> bool {
        let index: usize;
        match self.get_empty_squares().choose(&mut rand::rng()) {
            Some(v) => index = *v,
            None => return false
        }
        self.place(character, index)
    }

    pub fn check_winner(&self) -> Option<char> {
        if self.get_empty_squares().is_empty() {
            return Some(' ')
        }
        for alignment in [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 4, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8], [2, 4, 6],
        ] {
            if self.grid.get(alignment[0]).unwrap_or(&' ') == &'X' &&
            self.grid.get(alignment[1]).unwrap_or(&' ') == &'X' &&
            self.grid.get(alignment[2]).unwrap_or(&' ') == &'X' {
                return Some('X');
            }
            if self.grid.get(alignment[0]).unwrap_or(&' ') == &'O' &&
            self.grid.get(alignment[1]).unwrap_or(&' ') == &'O' &&
            self.grid.get(alignment[2]).unwrap_or(&' ') == &'O' {
                return Some('O');
            }
        }
        None
    }
    pub fn next(&mut self) -> Option<char> {
        if self.autoplay && self.current_player == 'O' {
            self.place_rand('O');
        } else {
            let mut parsed_input: usize;
            loop {
                match get_input().parse::<usize>() {
                    Ok(v) => {
                        parsed_input = v.into();
                        if self.get_empty_squares().contains(&parsed_input) {
                            break;
                        }
                    },
                    Err(_) => {}
                }
                println!("Please enter a valid index: {:?}", self.get_empty_squares());
            }
            self.place(self.current_player, parsed_input);
        }
        if self.current_player == 'X' {
            self.current_player = 'O';
        } else {
            self.current_player = 'X';
        }
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

