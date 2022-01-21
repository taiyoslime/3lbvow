use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

use std::collections::HashMap;

use super::helper;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AlphaStatus {
    Unknown,
    Correct,
    Present,
    Absent,
}

impl Default for AlphaStatus {
    fn default() -> Self {
        AlphaStatus::Unknown
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cell {
    pub status: AlphaStatus,
    pub letter: char,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Progress,
    Clear,
    GameOver,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Progress
    }
}

pub type Board = [[Cell; 5]; 6];

pub type AlphabetsStatus = HashMap<char, AlphaStatus>;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub answer: String,
    pub board: Board,
    pub current_col: usize,
    pub current_row: usize,
    pub game_status: GameStatus,
    pub alphabets_status: AlphabetsStatus,
    pub alert_message: String,
}

pub enum Action {
    Reset,
    PressChar(char),
    PressEnter,
    PressDelete,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match (self.game_status, action) {
            (_, Action::Reset) => {
                Self {
                    answer: helper::generate_new_answer(),
                    alphabets_status: helper::generate_new_alphabets_status(), 
                    ..Default::default()
                }
                .into()
            }
            (GameStatus::Progress, Action::PressChar(ch)) => {
                if self.current_col < 5 {
                    let mut board = self.board.clone();
                    board[self.current_row][self.current_col] = {
                        Cell {
                            letter: ch,
                            ..Default::default()
                        }
                    };

                    Self {
                        board,
                        current_col: self.current_col + 1,
                        alert_message: String::new(),
                        ..((*self).clone())
                    }
                    .into()
                } else {
                    self
                }
            }

            (GameStatus::Progress, Action::PressEnter) => {
                if self.current_col == 5 {
                    let mut word = String::new();
                    for cell in self.board[self.current_row].iter() {
                        word.push(cell.letter)
                    }

                    if helper::is_valid_word(&word) {
                        let mut board = self.board.clone();
                        let mut alphabets_status = self.alphabets_status.clone();

                        let mut used = [false; 5];
                        let mut solved = true;

                        // TODO
                        for i in 0..5 {
                            if word[i..(i + 1)] == self.answer[i..(i + 1)] {
                                board[self.current_row][i].status = AlphaStatus::Correct;
                                alphabets_status.insert(word.chars().nth(i).unwrap(), AlphaStatus::Correct);
                                used[i] = true;
                            } else {
                                solved = false;
                            }
                        }

                        if solved {
                            return Self {
                                board,
                                alphabets_status,
                                game_status: GameStatus::Clear,
                                alert_message: String::from("Congratz"),
                                ..Default::default()
                            }
                            .into();
                        }

                        for (i, x) in word.chars().enumerate() {
                            if board[self.current_row][i].status == AlphaStatus::Correct {
                                continue;
                            }
                            for (j, y) in self.answer.chars().enumerate() {
                                if x == y && !used[j] {
                                    board[self.current_row][i].status = AlphaStatus::Present;
                                    alphabets_status.insert(x, AlphaStatus::Present);
                                    used[j] = true;
                                    break;
                                }
                            }
                            if board[self.current_row][i].status == AlphaStatus::Unknown {
                                board[self.current_row][i].status = AlphaStatus::Absent;
                                alphabets_status.insert(x, AlphaStatus::Absent);
                            }
                        }

                        if self.current_row == 5 {
                            Self {
                                board,
                                alphabets_status,
                                game_status: GameStatus::GameOver,
                                alert_message: String::from("Game Over"),
                                ..Default::default()
                            }
                            .into()
                        } else {
                            Self {
                                board,
                                alphabets_status,
                                current_row: self.current_row + 1,
                                current_col: 0,
                                ..((*self).clone())
                            }
                            .into()
                        }
                    } else {
                        Self {
                            alert_message: String::from("Not in wordlist"),
                            ..((*self).clone())
                        }
                        .into()
                    }
                } else {
                    self
                }
            }

            (GameStatus::Progress, Action::PressDelete) => {
                if self.current_col > 0 {
                    let mut board = self.board.clone();
                    board[self.current_row][self.current_col - 1].letter = Default::default();
                    Self {
                        board,
                        current_col: self.current_col - 1,
                        alert_message: String::new(),
                        ..((*self).clone())
                    }
                    .into()
                } else {
                    self
                }
            }

            (_, _) => self
        }
    }
}
