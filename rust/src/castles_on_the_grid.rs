// You are given a square grid with some cells open (.) and some blocked (X). Your playing piece
// can move along any row or column until it reaches the edge of the grid or a blocked cell. Given
// a grid, a start and a goal, determine the minmum number of moves to get to the goal.
use std::collections::VecDeque;
use std::collections::HashSet;

/// The character represeting if a node is passable
const PASSABLE_CHAR: char = 'X';
/// The character representing if a node is impassable
const BLOCKED_CHAR: char = '.';

#[derive(Clone, PartialEq, Hash, Eq)]
struct Node {
    x: u8,
    y: u8,
}

impl Node {
    fn new(x: u8, y: u8) -> Node {
        Node { x, y }
    }
}

// returns a vector of a node's neigbors within the grid
// this fn exclusively works in a square grid with a nonzero size
// where you can only look in the in the cardinal directions for neighbors
// coordinate system runs from top left to bottom right
// [0,0] [1,0]
// [0,1] [1,1]
fn find_traversable_neighbors(grid_size: u8, node: Node) -> Vec<Node> {
    let mut neighbors = vec![];
    if node.x > 0 {
        neighbors.push(Node::new(node.x - 1, node.y));
    }
    if node.x < grid_size {
        neighbors.push(Node::new(node.x + 1, node.y));
    }
    if node.y > 0 {
        neighbors.push(Node::new(node.x, node.y - 1));
    }
    if node.y < grid_size {
        neighbors.push(Node::new(node.x, node.y + 1));
    }

    neighbors
}

// returns a list of neighbors that have not been traversed
fn find_untraversed_neighbors(
    grid_size: u8,
    node: Node,
    traversed_nodes: &[Node],
) -> HashSet<Node> {
    let mut traversable_nodes_set: HashSet<Node> = find_traversable_neighbors(grid_size, node)
        .into_iter()
        .collect();
    let traversed_nodes_set = HashSet::new();
    for n in traversed_nodes {
        traversable_nodes_set.insert(n.clone());
    }

    traversable_nodes_set.difference(&traversable_nodes_set)
}

/// A bfs that searches a set of nodes for an optimal path.
#[allow(dead_code)]
fn bfs(grid: &[Node], start: Node) {
    struct NodeDistance {
        node: Node,
        dist: u8,
    }

    let mut queued_nodes: VecDeque<NodeDistance> = VecDeque::with_capacity(grid.len());

    queued_nodes.push_back(NodeDistance {
        node: start,
        dist: 0,
    });

    while let current_node = queued_nodes.pop_front() {
        // get the current nodes neighbors

        // remove neighbors that have been visited
    }
}

#[allow(dead_code)]
fn castles_on_the_grid(grid: &[bool], start_coord: (u8, u8), goal_coord: (u8, u8)) {}

#[cfg(test)]
pub mod test {
    #[test]
    fn test() {}
}
