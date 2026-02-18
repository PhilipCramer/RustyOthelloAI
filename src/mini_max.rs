use crate::othello::{calculate_scores, Action, State};
pub fn search(
    state: &State,
    depth: isize,
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
        return search(&next_state, depth - 1, alpha, beta, !maximizing_player);
    }

    if maximizing_player {
        let mut value = isize::MIN;
        let mut best_action = None;
        let mut alpha = alpha;

        for action in actions {
            let next_state = state.do_action(Some(action.clone()));
            let (_, child_value) = search(&next_state, depth - 1, alpha, beta, false);

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
            let (_, child_value) = search(&next_state, depth - 1, alpha, beta, true);

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

fn evaluate_game(state: &State, _maximizing_player: bool) -> isize {
    coin_heuristic(state)
}
fn coin_heuristic(state: &State) -> isize {
    let (black, white) = calculate_scores(state);
    100 * (black - white) / (black + white)
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
