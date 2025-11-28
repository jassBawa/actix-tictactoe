use crate::game::game_state::{Board, Player};

pub struct GameEngine;

impl GameEngine {
    pub fn make_move(board: &mut Board, position: usize, player: Player) -> Result<(), GameError> {
        if position >= 9 {
            return Err(GameError::InvalidPosition);
        }

        let row = position / 3;
        let col = position % 3;

        if board[row][col] != None {
            return Err(GameError::PositionOccupied);
        }

        board[row][col] = Some(player);

        Ok(())
    }

    pub fn check_winner(board: &Board) -> Option<Player> {
        for row in board.iter() {
            if row[0].is_some() && row[0] == row[1] && row[1] == row[2] {
                return row[0];
            }
        }

        // check for columns
        for col in 0..3 {
            if board[0][col].is_some()
                && board[0][col] == board[1][col]
                && board[1][col] == board[2][col]
            {
                return board[0][col];
            }
        }

        // check diagonals
        if board[0][0].is_some() && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
            return board[0][0];
        }
        if board[0][2].is_some() && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
            return board[0][2];
        }

        None
    }

    pub fn is_board_full(board: &Board) -> bool {
        board
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
    }
}

#[derive(Debug)]
pub enum GameError {
    InvalidPosition,
    PositionOccupied,
    NotPlayerTurn,
    GameFinished,
    InvalidPlayer,
}
