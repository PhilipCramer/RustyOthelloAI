use crate::othello::{Action, State};

pub fn minimax(
    state: &State,
    depth: i32,
    alpha: isize,
    beta: isize,
    maximizing_player: bool,
) -> (Option<Action>, isize) {
    if depth == 0 || state.remaining_moves == 0 {
        return (None, evaluate_game(state, maximizing_player));
    }

    let actions = state.get_actions();

    if actions.is_empty() {
        let next_state = state.do_action(None);
        return minimax(&next_state, depth - 1, alpha, beta, !maximizing_player);
    }

    if maximizing_player {
        let mut value = isize::MIN;
        let mut best_action = None;
        let mut alpha = alpha;

        for action in actions {
            let next_state = state.do_action(Some(action.clone()));
            let (_, child_value) = minimax(&next_state, depth - 1, alpha, beta, false);

            if child_value > value {
                value = child_value;
                best_action = Some(action);
            }
            alpha = alpha.max(value);

            if beta <= alpha {
                break;
            }
        }
        return (best_action, value);
    } else {
        let mut value = isize::MAX;
        let mut best_action = None;
        let mut beta = beta;

        for action in actions {
            let next_state = state.do_action(Some(action.clone()));
            let (_, child_value) = minimax(&next_state, depth - 1, alpha, beta, true);

            if child_value < value {
                value = child_value;
                best_action = Some(action);
            }
            beta = beta.min(value);

            if beta <= alpha {
                break;
            }
        }
        return (best_action, value);
    }
}

fn evaluate_game(state: &State, maximizing_player: bool) -> isize {
    todo!()
}
fn coin_heuristic(state: &State, maximizing_player: bool) -> isize {
    let mut w_score: isize = 0;
    let mut b_score: isize = 0;
    for row in state.board.rows {
        let (w, b) = row.count_colors();
        w_score += w;
        b_score += b;
    }
    todo!()
}
fn corner_heuristic(state: &State, maximizing_player: bool) -> isize {
    todo!()
}
fn mobility_heuristic(state: &State, maximizing_player: bool) -> isize {
    todo!()
}
fn stability_heuristic(state: &State, maximizing_player: bool) -> isize {
    todo!()
}
