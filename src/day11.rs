use std::collections::HashMap;

pub fn solve_puzzle_a(input: &str) -> usize {
    let graph = Graph::parse(input, "you", "out", &[]);

    graph.num_paths(graph.root, graph.target)
}

pub fn solve_puzzle_b(input: &str) -> usize {
    let graph = Graph::parse(input, "svr", "out", &["dac", "fft"]);
    let dac = *graph.interest.get("dac").unwrap();
    let fft = *graph.interest.get("fft").unwrap();

    let root_to_fft = graph.num_paths(graph.root, fft);
    let fft_to_dac = graph.num_paths(fft, dac);
    let dac_to_target = graph.num_paths(dac, graph.target);
    let root_to_dac = graph.num_paths(graph.root, dac);
    let dac_to_fft = graph.num_paths(dac, fft);
    let fft_to_target = graph.num_paths(fft, graph.target);

    (root_to_dac * dac_to_fft * fft_to_target) + (root_to_fft * fft_to_dac * dac_to_target)
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
        let mut memo = HashMap::new();

        fn recurse(
            graph: &Graph,
            start: usize,
            target: usize,
            memo: &mut HashMap<usize, usize>,
        ) -> usize {
            if start == target {
                return 1;
            }

            if let Some(&num_paths) = memo.get(&start) {
                return num_paths;
            }

            let mut num_paths = 0;
            let edges_start = graph.edges.partition_point(|(from, _)| *from < start);
            let edges_end = graph.edges.partition_point(|(from, _)| *from <= start);
            for (_, to) in graph.edges.get(edges_start..edges_end).unwrap() {
                num_paths += recurse(graph, *to, target, memo);
            }

            memo.insert(start, num_paths);

            num_paths
        }

        recurse(self, start, target, &mut memo)
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
    use super::{solve_puzzle_a, solve_puzzle_b};

    #[test]
    fn example() {
        let input = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out\n";

        assert_eq!(solve_puzzle_a(input), 5);
    }

    #[test]
    fn example2() {
        let input = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out\n";

        assert_eq!(solve_puzzle_b(input), 2);
    }
}
