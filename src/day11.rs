use std::collections::{HashMap, VecDeque};

pub fn solve_puzzle_a(input: &str) -> usize {
    let graph = Graph::parse(input, "you", "out", &[]);

    graph.num_paths(graph.root, graph.target)
}

/// (from, to)
type Edge = (usize, usize);

#[derive(Debug)]
struct Graph<'a> {
    root: usize,
    target: usize,
    interest: HashMap<&'a str, usize>,
    edges: Vec<Edge>,
}

impl<'a> Graph<'a> {
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

    fn parse(input: &'a str, root: &str, target: &str, interest: &[&str]) -> Graph<'a> {
        let mut node_names = HashMap::new();
        let mut next_id = 0;
        let mut edges = Vec::new();

        for line in input.lines() {
            let (n, out) = line.split_once(':').unwrap();
            let n_id = *node_names.entry(n).or_insert_with(|| {
                let n_id = next_id;
                next_id += 1;

                n_id
            });
            for o in out.split_whitespace() {
                let o_id = *node_names.entry(o).or_insert_with(|| {
                    let o_id = next_id;
                    next_id += 1;

                    o_id
                });
                edges.push((n_id, o_id));
            }
        }

        edges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let root = *node_names.get(root).unwrap();
        let target = *node_names.get(target).unwrap();

        // let file = std::fs::File::create_new("day11.dot").unwrap();
        // let mut writer = std::io::BufWriter::new(file);
        // writer.write_all(b"digraph G {\n").unwrap();
        // writer
        //     .write_all(b"SVR [style=filled, fillcolor=coral]\n")
        //     .unwrap();
        // writer
        //     .write_all(b"TARGET [style=filled, fillcolor=coral]\n")
        //     .unwrap();
        // writer
        //     .write_all(b"DAC [style=filled, fillcolor=coral]\n")
        //     .unwrap();
        // writer
        //     .write_all(b"FFT [style=filled, fillcolor=coral]\n")
        //     .unwrap();
        // writer
        //     .write_all(b"YOU [style=filled, fillcolor=coral]\n")
        //     .unwrap();
        // let dac = *node_names.get("dac").unwrap();
        // let fft = *node_names.get("fft").unwrap();
        // let you = *node_names.get("you").unwrap();
        // let svr = *node_names.get("svr").unwrap();
        // for (from, to) in edges.iter() {
        //     let from = match *from {
        //         i if i == you => "YOU".to_string(),
        //         i if i == svr => "SVR".to_string(),
        //         i if i == target => "TARGET".to_string(),
        //         i if i == dac => "DAC".to_string(),
        //         i if i == fft => "FFT".to_string(),
        //         _ => format!("{from}"),
        //     };
        //     let to = match *to {
        //         i if i == you => "YOU".to_string(),
        //         i if i == svr => "SVR".to_string(),
        //         i if i == target => "TARGET".to_string(),
        //         i if i == dac => "DAC".to_string(),
        //         i if i == fft => "FFT".to_string(),
        //         _ => format!("{to}"),
        //     };
        //     writer
        //         .write_all(format!("{from} -> {to}\n").as_bytes())
        //         .unwrap();
        // }
        // writer.write_all(b"}\n").unwrap();

        node_names.retain(|k, _| interest.contains(k));

        Self {
            root,
            target,
            interest: node_names,
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
