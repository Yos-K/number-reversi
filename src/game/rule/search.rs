use crate::game::{
    assets::{
        board::Board, square::Square
    }, 
    rule::{
        position::{Position, ReversibleCandidates}, 
        turn::Turn
    }
};

/// Search for the positions of the pieces that are able to be reversed by the piece placed on the specified position by the current turn.
///
/// # Arguments
/// * `board` - The current board of the Othello game.
/// * `position` - The position to place the piece for the current turn.
/// * `turn` - The current turn.
/// * `direction` - The direction to search for the positions of the pieces that are able to be reversed.
/// * `result` - The result of the previous search in the same direction.
///
/// # Returns
/// A vector of `Position`s of the pieces that are able to be reversed.
pub fn search(board: &Board, position: Position, turn: Turn, direction: (isize, isize), result: Option<Vec<Position>>) -> Vec<Position> {
    let mut result = match result {
        Some(r) => r,
        None => vec![],
    };
    
    if board.is_put(position) {
        if result.len() == 0 {
            return vec![];
        }
    };
    
    let next = position.next_position(direction);
    if next.0 < 0 || next.0 >= 8 || next.1 < 0 || next.1 >= 8 {
        return vec![];
    }

    let next = Position{x: next.0 as usize, y: next.1 as usize};
    match board.squares[next.x][next.y] {
        Square::Put(p) => {
            if p.color == turn.color {
                if result.len() == 0 {
                    vec![]
                } else {
                    result.push(next);
                    result
                }
            } else {
                result.push(next);
                search(board, next.clone(), turn, direction, Some(result))
            }
        },
        _ => vec![]
    }
}
pub fn search_up(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (0, -1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_up_right(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (1, -1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_right(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (1, 0), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_down_right(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (1, 1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_down(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (0, 1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_down_left(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (-1, 1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_left(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (-1, 0), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}
pub fn search_up_left(board: &Board, position: Position, turn: Turn) -> Option<ReversibleCandidates> {
    let mut positions = search(board, position, turn, (-1, -1), None);
    let opposite = positions.pop();
    match opposite {
        Some(p) => Some(ReversibleCandidates::new(p, positions)),
        None => None,
    }
}

#[cfg(test)]
mod board_test {
    mod search_test {
        use crate::game::{
            assets::{
                board::Board, 
                piece::Piece
            }, 
            rule::{
                position::{
                    Position, 
                    ReversibleCandidates
                }, 
                color::Color, 
                turn::Turn, 
                search::{
                    search_right, 
                    search_up, 
                    search_up_right
                }
            }
        };

        #[test]
        fn when_next_piece_is_same_colors_then_return_empty_vec() {
            let board = Board::set_initial_state();
            let position = Position {x: 2, y: 3};
            let turn = Turn{color: Color::Black};
            let result = search_right(
                &board,
                position,
                turn,
            );
            assert_eq!(result, None);
        }
        #[test]
        fn when_next_piece_is_different_color_then_search_next_up(){
            let board = Board::set_initial_state();
            let position = Position {x: 3, y: 5};
            let turn = Turn{color: Color::Black};
            let result = search_up(
                &board,
                position,
                turn,
            );
            assert_eq!(result, Some(ReversibleCandidates::new(Position{x: 3, y: 3}, vec![Position {x: 3, y: 4}])));
        }
        #[test]
        fn when_next_piece_is_different_color_then_search_next_up_right(){
            let board = Board::set_initial_state();
            let board = board.put_piece(Position {x: 3, y: 5}, Piece::new(Color::White, 1));
            let position = Position {x: 2, y: 6};
            let turn = Turn{color: Color::Black};
            let result = search_up_right(
                &board,
                position,
                turn,
            );
            assert_eq!(result, Some(ReversibleCandidates::new(Position{x: 4, y: 4}, vec![Position {x: 3, y: 5}])));
        }
    }
}