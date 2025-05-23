use crate::othello::{simulate_game, Action, Color, State};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    state: State,
    action: Option<Action>,
    untried_actions: Vec<Action>,
    visits: usize,
    score: isize,
}

impl Node {
    pub fn new(state: State, action: Option<Action>, untried_actions: Vec<Action>) -> Node {
        Node {
            state,
            action,
            untried_actions,
            visits: 0,
            score: 0,
        }
    }

    pub fn update_node(&mut self, result: (Color, isize)) {
        self.visits += 1;
        if result.0 == self.state.next_turn {
            self.score += result.1;
        } else {
            self.score -= result.1
        }
    }
    // Calculates and returns the Upper Confidence Bound (UCB) for the Node
    fn calculate_ucb(&self, total_count: usize, explore: f32) -> f32 {
        (self.score as f32 / self.visits as f32)
            + explore * (2.0 * (total_count as f32).ln() / self.visits as f32).sqrt()
    }
}

#[derive()]
pub struct MCTS {
    pub size: usize,
    color: Color,
    expl: f32,
    nodes: Vec<Node>,
    tree: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    state_map: HashMap<State, usize>,
}

impl MCTS {
    pub fn new(col: &str, explore: f32) -> Self {
        let ai_color: Color;
        match col {
            b if b == "false".to_string() => ai_color = Color::BLACK,
            _ => ai_color = Color::WHITE,
        };
        Self {
            tree: Vec::new(),
            color: ai_color,
            expl: explore.clone(),
            parents: Vec::new(),
            state_map: HashMap::new(),
            size: 0,
            nodes: Vec::new(),
        }
    }

    // Performs a Monte Carlo Tree Search from the given state for the given number of iterations
    // It returns the best action found or an error if no action was found
    pub fn search(
        &mut self,
        from: State,
        iterations: usize,
        send_status: fn(usize, usize, &Color),
    ) -> Result<Action, ()> {
        if let Some(root) = self.state_map.get(&from).cloned() {
            for i in 0..iterations {
                if i % 1000 == 0 {
                    _ = send_status(i, iterations, &self.color);
                }
                let selected_node = self.select(root);
                let expanded_node = self.expand(selected_node);
                let result: (Color, isize) = self.simulate(expanded_node);
                self.backpropagate(expanded_node, result);
            }
            return self.get_best_choice(root);
        } else {
            self.add_node(from.clone(), None, None);
            return self.search(from, iterations, send_status);
        }
    }

    // Adds a new node to the MCTS with the given state, action, and parent
    fn add_node(&mut self, state: State, action: Option<Action>, parent: Option<usize>) {
        let new_node = Node::new(state, action, state.get_actions());
        self.state_map.insert(state, self.size);
        self.tree.push(Vec::new());
        self.parents.push(parent);
        self.nodes.push(new_node.clone());
        self.size += 1;
    }

    // Selects a node from the MCTS using the Upper Confidence Bound (UCB) formula
    fn select(&self, root_index: usize) -> usize {
        let mut max_ucb = std::f32::MIN;
        let mut max_index = 0 as usize;
        let mut node_index = root_index;
        let mut depth = 0;
        loop {
            // Failsafe to avoid tree becoming too deep
            if depth > 100 {
                return node_index;
            }
            let children = &self.tree[node_index];
            if children.is_empty() {
                return node_index;
            }
            if !self.nodes[node_index].untried_actions.is_empty() {
                return node_index;
            }
            let parent_visits = self.nodes[node_index].visits;
            for &child_index in children {
                let child = &self.nodes[child_index];
                let ucb = child.calculate_ucb(parent_visits, self.expl);

                if ucb > max_ucb {
                    max_ucb = ucb;
                    max_index = child_index;
                }
            }
            if max_index == node_index {
                return node_index;
            }
            node_index = max_index;
            max_ucb = std::f32::MIN;
            depth += 1;
        }
    }

    // Expands the given node in the MCTS by adding all its untried actions as new nodes
    fn expand(&mut self, node_index: usize) -> usize {
        // Get the node not a clone of it
        let untried_actions = self.nodes[node_index].untried_actions.clone();

        if untried_actions.is_empty() {
            // No actions to try add skip node
            let new_state = self.nodes[node_index].state.clone().do_action(None);
            self.add_node(new_state, None, Some(node_index));
            self.tree[node_index].push(self.size - 1);

            // Return the new node's index
            return self.size - 1;
        } else {
            // Pick one random action to expand (not all at once)
            let mut rng = rand::thread_rng();
            let action_index = rng.gen_range(0..untried_actions.len());
            let action = untried_actions[action_index].clone();

            // Remove this action from untried_actions in the original node
            self.nodes[node_index].untried_actions.remove(action_index);

            // Create a new node with this action
            let new_state = self.nodes[node_index]
                .state
                .clone()
                .do_action(Some(action.clone()));
            self.add_node(new_state, Some(action), Some(node_index));
            self.tree[node_index].push(self.size - 1);

            // Return the new node's index
            return self.size - 1;
        }
    }

    // Simulates a game from the given node and returns the result
    fn simulate(&mut self, node_index: usize) -> (Color, isize) {
        if let Some(node) = self.nodes.get_mut(node_index) {
            let mut node_state = node.state.clone();
            let mut score = simulate_game(&mut node_state);
            if self.color != node.state.next_turn {
                score *= -1;
            }
            node.update_node((node.state.next_turn, score));
            return (node_state.next_turn, score);
        }
        panic!("Node not found");
    }

    // Updates the nodes in the MCTS from the given child node to the root based on the result of a simulated game
    fn backpropagate(&mut self, child_index: usize, result: (Color, isize)) {
        let mut current_node: &mut Node;
        let mut parent_index: Option<usize> = self.parents.get(child_index).unwrap().clone();
        while parent_index.is_some() {
            current_node = self
                .nodes
                .get_mut(parent_index.unwrap())
                .expect("Parent doesn't exist");
            current_node.update_node(result);
            let tmp = parent_index.clone();
            parent_index = *self
                .parents
                .get(tmp.unwrap())
                .expect("Error fetching parent of parent");
        }
    }

    // Selects the best action from the given node in the MCTS based on the number of visits
    // It returns the best action found or an error if no action was found
    fn get_best_choice(&self, from_index: usize) -> Result<Action, ()> {
        let mut best_index = 0;
        let mut max_visits = 0;
        for index in self
            .tree
            .get(from_index)
            .expect("Empty list of children when getting best choice")
            .iter()
            .clone()
        {
            let node = self
                .nodes
                .get(*index)
                .expect("MCST, choice: node index doesnt exists");
            if node.visits > max_visits {
                best_index = index.clone();
                max_visits = node.visits;
            }
        }
        let best_node = self.nodes.get(best_index).unwrap().clone();
        if best_node.action.is_none() {
            return Err(());
        };
        let best_action = best_node.action.unwrap().clone();
        let from_state = self.nodes.get(from_index).unwrap().clone().state;
        if from_state.next_turn != best_action.color {
            return Err(());
        } else {
            Ok(best_action.clone())
        }
    }
}
