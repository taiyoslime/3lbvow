use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

use super::helper;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CellStatus {
    Unknown,
    Correct,
    Present,
    Absent,
}

impl Default for CellStatus {
    fn default() -> Self {
        CellStatus::Unknown
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cell {
    pub status: CellStatus,
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

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub answer: String,
    pub board: Board,
    pub current_col: usize,
    pub current_row: usize,
    pub game_status: GameStatus,
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
        match action {
            Action::Reset => {
                let answer = helper::generate_new_answer();
                Self {
                    answer: answer.into(),
                    ..Default::default()
                }
                .into()
            }
            Action::PressChar(ch) => {
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
            Action::PressEnter => {
                if self.current_col == 5 {
                    let mut word = String::new();
                    for cell in self.board[self.current_row].iter() {
                        word.push(cell.letter.to_ascii_lowercase()) // TODO
                    }

                    if helper::is_valid_word(&word) {
                        let mut board = self.board.clone();

                        let mut used = [false; 5];
                        let mut solved = true;
                        for i in 0..5 {
                            if word[i..(i + 1)] == self.answer[i..(i + 1)] {
                                board[self.current_row][i].status = CellStatus::Correct;
                                used[i] = true;
                            } else {
                                solved = false;
                            }
                        }

                        if solved {
                            return Self {
                                board,
                                game_status: GameStatus::Clear,
                                ..Default::default()
                            }
                            .into();
                        }

                        for (i, x) in word.chars().enumerate() {
                            for (j, y) in self.answer.chars().enumerate() {
                                if x == y && !used[j] {
                                    board[self.current_row][i].status = CellStatus::Present;
                                    used[j] = true;
                                    break;
                                }
                            }
                            if board[self.current_row][i].status == CellStatus::Unknown {
                                board[self.current_row][i].status = CellStatus::Absent;
                            }
                        }

                        if self.current_row == 5 {
                            Self {
                                board,
                                game_status: GameStatus::GameOver,
                                ..Default::default()
                            }
                            .into()
                        } else {
                            Self {
                                board,
                                current_row: self.current_row + 1,
                                current_col: 0,
                                ..((*self).clone())
                            }
                            .into()
                        }
                    } else {
                        Self {
                            alert_message: String::from("not in wordlist"),
                            ..((*self).clone())
                        }
                        .into()
                    }
                } else {
                    self
                }
            }

            Action::PressDelete => {
                if self.current_col > 0 {
                    let mut board = self.board.clone();
                    board[self.current_row][self.current_col - 1].letter = Default::default();
                    Self {
                        answer: self.answer.clone(),
                        board,
                        current_row: self.current_row,
                        current_col: self.current_col - 1,
                        game_status: self.game_status,
                        alert_message: String::new(),
                    }
                    .into()
                } else {
                    self
                }
            }
        }
    }
}
