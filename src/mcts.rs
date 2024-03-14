use crate::othello::{State, Action, simulate_game};
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Node {
    state: State,
    action: Option<Action>,
    untried_actions: Vec<Action>,
    visits: usize,
    wins: usize,
}

impl Node {
    pub fn new (state: State, action: Option<Action>, untried_actions: Vec<Action>) -> Node {
        Node {
            state,
            action,
            untried_actions,
            visits: 0,
            wins: 0,
        }
    }

    pub fn update_node(&mut self, result: (char, bool))  {
        self.visits += 1;
        if result.0 == self.state.next_turn && result.1 {
            self.wins += 1;
        }
    }
    // Calculates and returns the Upper Confidence Bound (UCB) for the Node
    fn calculate_ucb(&self, total_count: usize) -> f64 {
        (self.wins as f64 / self.visits as f64) + (2.0 * (total_count as f64).ln() / self.visits as f64).sqrt()
    }

}

#[derive()]
pub struct MCTS {
    pub size: usize,
    nodes: Vec<Node>,
    tree: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    state_map: HashMap<State, usize>,
}

impl MCTS {
    pub fn new(node: Node) -> Self {
        let mut map = HashMap::new();
        map.insert(node.state, 0 as usize);
        Self {
            tree: vec![Vec::new()],
            parents: vec![None],
            state_map: map.to_owned(),
            size: 1,
            nodes: vec![node],
        }
    }

    // Performs a Monte Carlo Tree Search from the given state for the given number of iterations
    // It returns the best action found or an error if no action was found
    pub fn search(&mut self, from: State, iterations: u64) -> Result<Action, ()> {
        if let Some(root) = self.state_map.get(&from).cloned() {
            for _i in 0..iterations {
                let node_index = self.select(root.clone()).clone();
                let node_index = self.expand(node_index.clone()).clone();
                for index in self.tree.get(node_index).expect("No child nodes to simulate").clone().iter() {
                    let result = &self.simulate(*index);
                    self.backpropagate(*index, result.clone());
                }

            }
            Ok(self.get_best_choice(root)?)
        }
        else {
            self.add_node(from.clone(), None, None);
            return self.search(from, iterations)
        }
    }

    // Adds a new node to the MCTS with the given state, action, and parent
    fn add_node(&mut self, state: State, action: Option<Action>, parent: Option<usize>){
        let new_node = Node::new(state, action, state.get_actions());
        self.state_map.insert(state, self.size);
        self.tree.push(Vec::new());
        self.parents.push(parent);
        self.nodes.push(new_node.clone());
        self.size += 1;
    }

    // Selects a node from the MCTS using the Upper Confidence Bound (UCB) formula
    fn select(&self, root_index: usize) -> usize {
        let mut max_ucb = 0.0;
        let mut max_index = 0 as usize;
        let mut node_index = root_index;
        loop {
            if self.tree.get(node_index).expect("Empty child selection").len() == 0  {
                return node_index;
            }
            else {
                for index in self.tree.get(node_index).unwrap().iter() {
                    let node = self.nodes.get(*index).expect("selected child doesnt exist").clone();
                    let node_ucb = node.calculate_ucb(self.nodes.get(root_index).unwrap().visits as usize);
                    if node_ucb > max_ucb {
                        max_ucb = node_ucb;
                        max_index = index.clone();
                    }
                }
                node_index = max_index;
            } 
            max_ucb = 0.0;
            max_index = 0;
        }
    }

    // Expands the given node in the MCTS by adding all its untried actions as new nodes
    fn expand(&mut self, node_index: usize) -> usize {
        let mut node = self.nodes.get_mut(node_index).expect("No node to expand").clone();
        if node.untried_actions.len() == 0 {
            self.add_node(node.state.clone().do_action(None),
                None, 
                Some(node_index.clone())
            );
            self.tree.get_mut(node_index).expect("No node").push(self.size - 1);
        } else {
            for (_i, action) in node.untried_actions.iter().enumerate() {
                self.add_node(
                    node.state.clone().do_action(Some(action.clone())), 
                    Some(action.clone()), 
                    Some(node_index.clone())
                );
                self.tree.get_mut(node_index).expect("No node").push(self.size - 1);
            }
            while node.untried_actions.len() > 0 {
                node.untried_actions.pop();
            }
        }
        node_index
    }

    // Simulates a game from the given node and returns the result
    fn simulate(&mut self, node_index: usize) -> (char, bool) {
        if let Some(node) = self.nodes.get_mut(node_index) {
            let mut node_state = node.state.clone();
            let win = simulate_game(&mut node_state);
            node.update_node((node.state.next_turn, win));
            return (node_state.next_turn, win);
        }
        ('_', false)
    }

    // Updates the nodes in the MCTS from the given child node to the root based on the result of a simulated game
    fn backpropagate(&mut self, child_index: usize, result: (char, bool)) {
        let mut current_node: &mut Node;
        let mut parent_index: Option<usize>  = self.parents.get(child_index).unwrap().clone(); 
        while parent_index.is_some() {
            current_node = self.nodes.get_mut(parent_index.unwrap()).expect("Parent doesn't exist");
            current_node.update_node(result);
            let tmp = parent_index.clone();
            parent_index = *self.parents.get(tmp.unwrap()).expect("Error fetching parent of parent");
        }
    }

    // Selects the best action from the given node in the MCTS based on the number of visits
    // It returns the best action found or an error if no action was found
    fn get_best_choice(&self, from_index: usize) -> Result<Action, ()> {
        let mut best_index = 0;
        let mut max_visits = 0;
        for index in self.tree.get(from_index).expect("Empty list of children when getting best choice").iter().clone() {
            let node = self.nodes.get(*index).expect("MCST, choice: node index doesnt exists");
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
        }
        else {
            Ok(best_action.clone())
        }
    }
}

