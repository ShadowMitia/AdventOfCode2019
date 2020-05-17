use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

// https://www.redblobgames.com/pathfinding/a-star/introduction.html#astar
mod AStar {
    use std::collections::HashMap;

    use std::fmt;
    use std::fmt::{Debug, Formatter};

    pub fn manhattan_distance(p1: &Node, p2: &Node) -> i32 {
        i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y)
    }

    #[derive(Debug, Eq, Copy, Clone, PartialEq, Hash)]
    pub struct Node {
        x: i32,
        y: i32,
    }

    impl Node {
        pub fn new(x: i32, y: i32) -> Self {
            Node { x, y }
        }
    }

    pub struct Graph<'a, F>
    where
        F: Fn(&Node, &Node) -> i32,
    {
        nodes: &'a Vec<Node>,
        neighbors: &'a HashMap<Node, Vec<Node>>,
        cost: F,
    }

    impl<'a, F> Debug for Graph<'a, F>
    where
        F: Fn(&Node, &Node) -> i32,
    {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
                f,
                "Graph{{ nodes {:#?}  neighbors {:#?} }}",
                self.nodes, self.neighbors
            )
        }
    }

    impl<'a, F> Graph<'a, F>
    where
        F: Fn(&Node, &Node) -> i32,
    {
        pub fn new(nodes: &'a Vec<Node>, neighbors: &'a HashMap<Node, Vec<Node>>, cost: F) -> Self {
            Graph {
                nodes,
                neighbors,
                cost,
            }
        }

        pub fn shortest_path(
            &self,
            goal: &Node,
            start: &Node,
            heuristic: &dyn Fn(&Node, &Node) -> i32,
        ) -> Vec<Node>
        where
            F: Fn(&Node, &Node) -> i32,
        {
            let mut frontier = Vec::new();
            frontier.push((start, 0));

            let mut came_from: HashMap<&Node, Option<&Node>> = HashMap::new();
            came_from.insert(start, None);

            let mut cost_so_far: HashMap<&Node, i32> = HashMap::new();
            cost_so_far.insert(start, 0);

            while frontier.len() > 0 {
                let (current, _) = frontier[0];
                frontier.remove(0);

                if current == goal {
                    break;
                }

                for neighbors in self.neighbors.get(current) {
                    for next in neighbors {
                        let new_cost =
                            cost_so_far.get(current).unwrap() + (self.cost)(current, next);

                        if cost_so_far.get(next) == None
                            || new_cost < *cost_so_far.get(next).unwrap()
                        {
                            cost_so_far.insert(next, new_cost);
                            let priority = new_cost + heuristic(goal, next);
                            frontier.push((next, priority));
                            frontier.sort_by(|n1, n2| n1.1.cmp(&n2.1));
                            came_from.insert(next, Some(current));
                        }
                    }
                }
            }

            let mut res = Vec::new();
            res.push(goal.clone());
            let mut current = goal;
            loop {
                match came_from.get(current) {
                    Some(Some(node)) => {
                        res.push(*node.clone());
                        current = node;
                    }
                    _ => break,
                }
            }
            res.iter().rev().cloned().collect::<Vec<Node>>()
        }
    }

    mod test {
        use super::*;

        #[test]
        fn test_shortest_path() {
            let nodes = vec![Node { x: 0, y: 0 }, Node { x: 10, y: 10 }];
            let mut neighbors = HashMap::new();
            neighbors.insert(nodes[0], vec![nodes[1]]);
            let cost = manhattan_distance;

            let graph = Graph::new(&nodes, &neighbors, &cost);

            assert_eq!(graph.shortest_path(&nodes[1], &nodes[0], &cost), nodes);

            let nodes = vec![
                Node { x: 0, y: 0 },
                Node { x: 100, y: 100 },
                Node { x: 10, y: 10 },
            ];
            let mut neighbors = HashMap::new();
            neighbors.insert(nodes[0], vec![nodes[1], nodes[2]]);
            neighbors.insert(nodes[2], vec![nodes[1]]);

            let cost = manhattan_distance;

            let graph = Graph::new(&nodes, &neighbors, &cost);

            assert_eq!(
                graph.shortest_path(&nodes[1], &nodes[0], &cost),
                vec![nodes[0], nodes[1]]
            );

            let nodes = vec![
                Node { x: 0, y: 0 },
                Node { x: 50, y: 50 },
                Node { x: 10, y: -10 },
                Node { x: 100, y: 40 },
                Node { x: 200, y: 40 },
            ];
            let mut neighbors = HashMap::new();
            neighbors.insert(nodes[0], vec![nodes[1], nodes[2]]);
            neighbors.insert(nodes[1], vec![nodes[4]]);
            neighbors.insert(nodes[2], vec![nodes[3]]);
            neighbors.insert(nodes[3], vec![nodes[4]]);
            let cost = manhattan_distance;

            let graph = Graph::new(&nodes, &neighbors, &cost);

            assert_eq!(
                graph.shortest_path(&nodes[4], &nodes[0], &cost),
                vec![nodes[0], nodes[1], nodes[4]]
            );
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::convert::Into<AStar::Node> for Point {
    fn into(self) -> AStar::Node {
        AStar::Node::new(self.x, self.y)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut file = File::open("day3/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let inputs: Vec<&str> = contents.lines().collect();

    let wire1 = parse_wire(inputs[0]);
    let wire2 = parse_wire(inputs[1]);

    println!(
        "Answer for day3 - part 1 {}",
        get_closest_intersection_distance(&wire1, &wire2)
    );

    println!(
        "Answer for day3 - part 2 {}",
        get_closest_intersection_on_wire_cumulative(&wire1, &wire2)
    );
}

fn get_closest_intersection_distance(wire1: &Vec<Point>, wire2: &Vec<Point>) -> i32 {
    let origin_point = Point::new(0, 0);

    let mut intersections = Vec::new();

    for i in 1..wire1.len() {
        for j in 1..wire2.len() {
            let p1 = &wire1[i - 1];
            let p2 = &wire1[i];
            let p3 = &wire2[j - 1];
            let p4 = &wire2[j];

            match get_segment_intersection(p1, p2, p3, p4) {
                Some(p) => intersections.push(p),
                None => continue,
            }
        }
    }

    let mut intersections: Vec<i32> = intersections
        .iter()
        .filter(|&p| *p != origin_point)
        .map(|p| manhattan_distance(&origin_point, &p))
        .collect();

    intersections.sort();
    intersections[0]
}

fn index(i: usize, j: usize, width: usize, height: usize) -> usize {
    width * j + i
}

fn get_closest_intersection_on_wire_cumulative(wire1: &Vec<Point>, wire2: &Vec<Point>) -> i32 {
    let origin_point = Point::new(0, 0);

    let mut nodes = wire1.clone();

    let mut intersections = Vec::new();

    let mut other_nodes = wire2.clone();

    for i in 1..wire1.len() {
        for j in 1..wire2.len() {
            let p1 = &wire1[i - 1];
            let p2 = &wire1[i];

            let p3 = &wire2[j - 1];
            let p4 = &wire2[j];

            match get_segment_intersection(p1, p2, p3, p4) {
                Some(p) => {
                    let pos = nodes.iter_mut().position(|n| n == p2).unwrap();
                    nodes.insert(pos, p);
                    intersections.push(p);

                    let pos = other_nodes.iter().position(|n| n == p4).unwrap();
                    other_nodes.insert(pos, p);
                }
                None => {
                    continue;
                }
            }
        }
    }

    nodes.remove(0);
    other_nodes.remove(0);

    intersections = intersections
        .iter()
        .filter(|&&node| node.x != 0 && node.y != 0)
        .cloned()
        .collect();

    println!("{:?}", intersections);

    let mut neighbors: HashMap<AStar::Node, Vec<AStar::Node>> = HashMap::new();
    let mut other_neighbiors: HashMap<AStar::Node, Vec<AStar::Node>> = HashMap::new();

    for i in 0..nodes.len() - 1 {
        neighbors.insert(nodes[i].into(), vec![nodes[i + 1].into()]);
    }
    neighbors.insert(nodes[nodes.len() - 1].into(), vec![]);

    for i in 0..other_nodes.len() - 1 {
        other_neighbiors.insert(other_nodes[i].into(), vec![other_nodes[i + 1].into()]);
    }
    other_neighbiors.insert(other_nodes[other_nodes.len() - 1].into(), vec![]);

    let nodes: Vec<AStar::Node> = nodes.iter().map(|&n| n.into()).collect();
    let other_nodes: Vec<AStar::Node> = other_nodes.iter().map(|&n| n.into()).collect();

    let graph = AStar::Graph::new(&nodes, &neighbors, &AStar::manhattan_distance);
    let graph2 = AStar::Graph::new(&other_nodes, &other_neighbiors, &AStar::manhattan_distance);

    println!("{:#?}", graph);

    let mut steps = Vec::new();

    for intersection in intersections {
        let path = graph.shortest_path(&intersection.into(), &nodes[0], &AStar::manhattan_distance);

        let mut dist = 0;
        for node_index in 0..path.len() - 1 {
            dist += AStar::manhattan_distance(&path[node_index], &path[node_index + 1]);
        }

        println!("{:?}", path);

        ///////////////////////////////////////////////////////

        let path = graph2.shortest_path(
            &intersection.into(),
            &other_nodes[0],
            &AStar::manhattan_distance,
        );

        println!("{:?}", path);

        let mut dist2 = 0;
        for node_index in 0..path.len() - 1 {
            dist2 += AStar::manhattan_distance(&path[node_index], &path[node_index + 1]);
        }

        steps.push((dist, dist2));
    }

    let mut shortest_dir = i32::MAX;

    for (step1, step2) in steps {
        if step1 + step2 <= shortest_dir {
            shortest_dir = step1 + step2;
        }
    }

    shortest_dir
}

fn parse_wire(wire: &str) -> Vec<Point> {
    let splitted_wire = wire.split(",").collect::<Vec<&str>>();

    let mut points = Vec::new();
    points.push(Point::new(0, 0));

    for pos in splitted_wire {
        let direction = pos.chars().nth(0).unwrap();
        let distance = pos
            .chars()
            .skip(1)
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        let start = points.iter().last().unwrap();

        let new_start = match (direction, distance) {
            ('R', val) => Point::new(start.x + val, start.y),
            ('U', val) => Point::new(start.x, start.y + val),
            ('D', val) => Point::new(start.x, start.y - val),
            ('L', val) => Point::new(start.x - val, start.y),
            _ => panic!("Shouldn't happen"),
        };

        points.push(new_start);
    }

    points
}

// ref : page 8 http://nguyen.univ-tln.fr/share/GeomAlgo/trans_inter.pdf
// Shouldn't work in general case, works fine for us, since we only have integer values
// TODO: make this more robust, and make work with points made of f32
///
///
/// Computes the intersection point between two segments
///
/// # Explanation
///
/// To compute this we solve a linear equation of two equations with two unknowns
/// We can use the [Cramer's Rule](https://en.wikipedia.org/wiki/Cramer%27s_rule) to see if the system has solutions (i.e. has an intersection)
/// Then we can compute the ratio of the distance of the intersection point on one of the segments
///
/// # Refs
/// - page 8 http://nguyen.univ-tln.fr/share/GeomAlgo/trans_inter.pdf
/// - [https://en.wikipedia.org/wiki/Line_segment_intersection](https://en.wikipedia.org/wiki/Line_segment_intersection)
fn get_segment_intersection(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> Option<Point> {
    // compute determinant (imagine two-bt-two matrix)
    let det: f32 = ((p2.x - p1.x) * (p3.y - p4.y) - (p3.x - p4.x) * (p2.y - p1.y)) as f32;
    if det == 0.0 {
        return None;
    } else {
        // This computes the ratio of the intersection point on each segment
        // TODO: figure out what this formula is? Could be this : https://en.wikipedia.org/wiki/Cramer%27s_rule#General_case but unsure
        let t1: f32 = ((p3.x - p1.x) * (p3.y - p4.y) - (p3.x - p4.x) * (p3.y - p1.y)) as f32 / det;
        let t2: f32 = ((p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y)) as f32 / det;

        if t1 > 1.0 || t1 < 0.0 || t2 > 1.0 || t2 < 0.0 {
            return None;
        } else {
            if t1 == 1.0 {
                return Some(p2.clone());
            } else if t1 == 0.0 {
                return Some(p1.clone());
            } else if t2 == 0.0 {
                return Some(p3.clone());
            } else if t2 == 1.0 {
                return Some(p4.clone());
            }

            return Some(Point::new(
                (p1.x as f32 + t1 * (p2.x - p1.x) as f32) as i32,
                (p1.y as f32 + t1 * (p2.y - p1.y) as f32) as i32,
            ));
        }
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y)
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_wire() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let output = vec![
            Point::new(0, 0),
            Point::new(75, 0),
            Point::new(75, -30),
            Point::new(158, -30),
            Point::new(158, 53),
            Point::new(146, 53),
            Point::new(146, 4),
            Point::new(217, 4),
            Point::new(217, 11),
            Point::new(145, 11),
        ];

        assert_eq!(parse_wire(input), output);
    }

    #[test]
    fn test_get_intersection() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(0, 10);

        let p3 = Point::new(0, 0);
        let p4 = Point::new(10, 0);

        assert_eq!(
            get_segment_intersection(&p1, &p2, &p3, &p4),
            Some(Point::new(0, 0))
        );

        let p1 = Point::new(20, 10);
        let p2 = Point::new(40, 10);

        let p3 = Point::new(30, 20);
        let p4 = Point::new(30, 0);

        assert_eq!(
            get_segment_intersection(&p1, &p2, &p3, &p4),
            Some(Point::new(30, 10))
        );

        let p1 = Point::new(20, -10);
        let p2 = Point::new(40, -10);

        let p3 = Point::new(30, -20);
        let p4 = Point::new(30, 0);

        assert_eq!(
            get_segment_intersection(&p1, &p2, &p3, &p4),
            Some(Point::new(30, -10))
        );
    }

    #[test]
    fn test_get_closest_intersection_distance() {
        let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(get_closest_intersection_distance(&wire1, &wire2), 159);

        let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(get_closest_intersection_distance(&wire1, &wire2), 135);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 10);
        assert_eq!(manhattan_distance(&p1, &p2), 20);

        let p1 = Point::new(-200, 20);
        let p2 = Point::new(4000, 18);
        assert_eq!(manhattan_distance(&p1, &p2), 4202);
    }

    #[test]
    fn test_get_closest_intersection_on_wire_cumulative() {
        let wire1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let wire2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(
            get_closest_intersection_on_wire_cumulative(&wire1, &wire2),
            610
        );

        let wire1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let wire2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(
            get_closest_intersection_on_wire_cumulative(&wire1, &wire2),
            410
        );
    }
}
