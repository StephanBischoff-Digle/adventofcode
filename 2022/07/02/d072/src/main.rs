use std::{collections::HashMap, fs};

#[derive(Debug)]
enum INode {
    Dir(HashMap<String, usize>),
    File(usize),
}

impl INode {
    fn dir(parent: usize, id: usize) -> Self {
        let hs = [(String::from("."), id), (String::from(".."), parent)].into();
        Self::Dir(hs)
    }

    fn file(size: usize) -> Self {
        Self::File(size)
    }
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<INode>,
}

impl FileSystem {
    fn build(input: String) -> Self {
        let mut nodes = vec![INode::dir(0, 0)];

        let mut pos = 0;
        let mut line_index = 1;
        let lines = input.lines().collect::<Vec<_>>();
        while line_index < lines.len() {
            let line = lines[line_index].trim();
            if line.starts_with("$ ls") {
                line_index += 1;
                let mut ls_l = lines[line_index].trim();
                while !ls_l.starts_with("$") {
                    if ls_l.starts_with("dir") {
                        let name = ls_l.split(" ").nth(1).unwrap().to_owned();
                        let my_idx = nodes.len();
                        nodes.push(INode::dir(pos, my_idx));
                        match nodes[pos] {
                            INode::Dir(ref mut content) => _ = content.insert(name, my_idx),
                            _ => (),
                        }
                    } else {
                        let split = ls_l.split(" ").collect::<Vec<_>>();
                        let size = split[0].parse().unwrap();
                        let name = split[1].to_owned();
                        let my_idx = nodes.len();
                        nodes.push(INode::file(size));
                        match nodes[pos] {
                            INode::Dir(ref mut content) => _ = content.insert(name, my_idx),
                            _ => (),
                        }
                    }

                    line_index += 1;
                    if line_index >= lines.len() {
                        break;
                    }
                    ls_l = lines[line_index].trim();
                }
            } else {
                let name = line[5..].to_owned();
                if let INode::Dir(map) = &nodes[pos] {
                    pos = *map.get(&name).unwrap();
                }
                line_index += 1;
            }
        }

        Self { nodes }
    }

    fn solve(&self) -> usize {
        let mut sizes = HashMap::new();
        let mut more_passes = 1;

        while more_passes > 0 {
            more_passes = 0;
            self.nodes.iter().enumerate().for_each(|(idx, node)| {
                if !sizes.contains_key(&idx) {
                    if let INode::Dir(content) = node {
                        let mut size = 0;
                        let mut finished = true;
                        for (k, c) in content {
                            if [".".to_owned(), "..".to_owned()].contains(k) {
                                continue;
                            }

                            match self.nodes[*c] {
                                INode::Dir(_) => match sizes.get(c) {
                                    Some(s) => size += s,
                                    None => {
                                        more_passes += 1;
                                        finished = false;
                                        break;
                                    }
                                },
                                INode::File(s) => size += s,
                            }
                        }
                        if finished {
                            sizes.insert(idx, size);
                        }
                    }
                }
            });
        }

        let total_space = 70_000_000;
        let needed_space = 30_000_000;
        let used_space = sizes.get(&0).unwrap();
        let remaining = total_space - used_space;
        let delta = needed_space - remaining;

        sizes
            .into_values()
            .filter(|&val| val > delta)
            .min()
            .unwrap()
    }
}

fn main() {
    let Ok(input) = fs::read_to_string("input.txt") else {
        eprintln!("Failed to read input.txt");
        return;
    };

    let filesystem = FileSystem::build(input);
    println!("{}", filesystem.solve());
}
