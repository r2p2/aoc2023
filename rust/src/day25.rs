use std::collections::HashSet;

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Connection<'a> {
    a: &'a str,
    b: &'a str,
}
type Connections<'a> = Vec<Connection<'a>>;

fn parse(input: &str) -> Connections {
    let mut connections = Vec::new();

    input.lines().for_each(|line| {
        let mut it = line.split(':');
        let src = it.next().unwrap();
        it.next().unwrap().split_whitespace().for_each(|dst| {
            connections.push(Connection { a: src, b: dst });
        })
    });

    connections
}

#[derive(Clone)]
struct Node<'a> {
    id: &'a str,
    joined: usize,
}
impl<'a> Node<'a> {
    fn new(id: &'a str) -> Node {
        Node { id, joined: 1 }
    }
}

struct Map<'a> {
    nodes: Vec<Node<'a>>,
    connections: Vec<(usize, usize)>,
}
impl<'a> Map<'a> {
    fn from(connections: &Connections<'a>) -> Map<'a> {
        let mut map = Map {
            nodes: Vec::new(),
            connections: Vec::new(),
        };

        // add nodes
        connections.iter().for_each(|c| {
            if let None = map.find_node(c.a) {
                map.nodes.push(Node::new(c.a));
            }
            if let None = map.find_node(c.b) {
                map.nodes.push(Node::new(c.b));
            }
        });

        // add connections
        connections.iter().for_each(|c| {
            let src_idx = map.find_node(c.a).unwrap();
            let dst_idx = map.find_node(c.b).unwrap();

            map.connections.push((src_idx, dst_idx));
            map.connections.push((dst_idx, src_idx));
        });

        map
    }
    fn count_active_nodes(&self) -> usize {
        let mut seen = HashSet::new();
        self.connections.iter().for_each(|(s, d)| {
            seen.insert(*s);
            seen.insert(*d);
        });
        seen.len()
    }
    fn count_edges(&self) -> usize {
        self.connections.len()
    }
    fn find_node(&self, id: &'a str) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .find_map(|(i, n)| if n.id == id { Some(i) } else { None })
    }
    fn contract(&mut self) {
        let contract_idx = rand::random::<usize>() % self.count_edges(); // FIXME: modulo
                                                                         // considered problematic
        let (src_idx, dst_idx) = self.connections[contract_idx];
        self.connections.remove(contract_idx);

        self.nodes[src_idx].joined += self.nodes[dst_idx].joined;

        self.connections.iter_mut().for_each(|(src, dst)| {
            if *src == dst_idx {
                *src = src_idx;
            } else if *dst == dst_idx {
                *dst = src_idx;
            }
        });
        self.connections.retain(|(a, b)| a != b); // remove self-referential
    }
}

pub fn task1(input: &str) -> usize {
    let connections = parse(input);

    // We hope we don't need to try it that often!
    (0..100_000_000).into_par_iter().find_map_first(|_| {
        let mut map = Map::from(&connections);
        while map.count_active_nodes() > 2 {
            map.contract();
        }
        let (src_idx, dst_idx) = map.connections.first().unwrap();
        let solution = map.nodes[*src_idx].joined * map.nodes[*dst_idx].joined;
        println!("{}:{}", solution, map.count_edges() / 2);
        if map.count_edges() / 2 == 3 {
            return Some(solution);
        }
        return None;
    }).unwrap()
}

#[test]
fn task1_test() {
    let data = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;
    assert_eq!(task1(data), 54);
}
