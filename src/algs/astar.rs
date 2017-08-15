
// struct Node {
//     x: usize,
//     y: usize,
//     idx: usize,
//     path: usize,
// }

// struct Edge {
//     from: usize,
//     to: usize,
//     cost: f32
// }

// struct Graph {
//     v: Vec[Node],
//     e: Vec[Edge]
// }

// // Use `euclidean distance` for the heuristic
// fn h(start: &Node, goal: &Node) -> f32 {
//     let dx = goal.x - start.x;
//     let dy = goal.y - start.y;

//     sqrt(dx * dx + dy * dy)
// }

// // TODO: Adapt fn to allow for changing the heuristic
// // TODO: Adapt this to not destroy the nodes/graph
// fn A_star(start: Node, goal: Node, g: Graph -> Option[Vec[Node]] {
//     // Keep track of the cost to reach the node
//     let mut cost = HashMap::new();
//     cost.insert(start.idx, 0: f32);

//     // Keep track of the estimated heuristic cost
//     let mut heuristic = HashMap::new();
//     heuristic.insert(start.idx, h(&start, &goal));

//     // Keep track of which nodes I've visited and can visit
//     let mut open = vec!(start.idx);
//     let mut closed = HashSet::new()

//     // Take the cheapest path for the current travel
//     while let Some(current) = open.pop() {
//         // If I've reached the goal, return the path
//         if (current == goal.idx) {
//             return Some(mk_path(g, current));
//         }

//         // Mark the current node as visited
//         closed.insert(current);

//         // TODO: Implement `neighbors`
//         for e in g.neighbors(current) {
//             // Don't do anything if already visited
//             if closed.contains(e.to) { continue; }
//             if !open.contains(e.to) { open.push_back(n); }

//             // Get the cost to travel and ensure it is the lowest possible
//             let score = cost.get(current).unwrap() + e.cost;
//             if score >= cost.get(e.to).unwrap_or(f32::max()) { continue; }

//             // Record the path to this node
//             g.v.get_mut(current).unwrap().path = e.from;
//             cost.entry(e.to).insert(score);
//             heuristic.entry(e.to).insert(score + h(e.to, goal.idx);
//         }

//         // Sort the vector by the heuristic cost
//         // TODO: Test whether I need to protect against 'max'. If they're in `open`, `heuristic` should be set, right?
//         // TODO: Check whether this is the correct algorithm or not
//         open.sort_by(
//             |a, b| heuristic.get(b).unwrap()
//                             .partial_cmp(heuristic.get(a).unwrap())
//                             .unwrap());
//     }
    
//     None
// }

// impl Graph {
    
// }

// // 18005411370
// // $200