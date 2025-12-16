use std::collections::{HashMap, VecDeque};

pub fn solve_puzzle_a(input: &str) -> usize {
    let graph = Graph::from(input);

    graph.num_paths(graph.root, graph.target)
}

/// (from, to)
type Edge = (usize, usize);

#[derive(Debug)]
struct Graph {
    root: usize,
    target: usize,
    edges: Vec<Edge>,
}

impl Graph {
    fn num_paths(&self, start: usize, target: usize) -> usize {
        let mut num_paths = 0;
        let mut queue = VecDeque::new();
        queue.push_back(vec![start]);

        while let Some(path) = queue.pop_front() {
            let node = *path.last().unwrap();
            if node == target {
                num_paths += 1;
                continue;
            }

            let start = self.edges.partition_point(|(from, _)| *from < node);
            let end = self.edges.partition_point(|(from, _)| *from <= node);
            for (_, to) in self.edges.get(start..end).unwrap() {
                let mut new_path = path.clone();
                new_path.push(*to);
                queue.push_back(new_path);
            }
        }

        num_paths
    }
}

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        let mut node_names = HashMap::new();
        let mut next_id = 0;
        let mut edges = Vec::new();
        let mut root = 0;
        let mut target = 0;

        for line in value.lines() {
            let (n, out) = line.split_once(':').unwrap();
            let n_id = *node_names.entry(n).or_insert_with(|| {
                let n_id = next_id;
                next_id += 1;
                if n == "you" {
                    root = n_id;
                } else if n == "out" {
                    target = n_id;
                }

                n_id
            });
            for o in out.split_whitespace() {
                let o_id = *node_names.entry(o).or_insert_with(|| {
                    let o_id = next_id;
                    next_id += 1;
                    if o == "you" {
                        root = o_id;
                    } else if o == "out" {
                        target = o_id;
                    }

                    o_id
                });
                edges.push((n_id, o_id));
            }
        }

        edges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        Self {
            root,
            target,
            edges,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve_puzzle_a;

    #[test]
    fn example() {
        let input = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out\n";

        assert_eq!(solve_puzzle_a(input), 5);
    }
}
