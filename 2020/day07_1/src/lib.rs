use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Bag<'a> {
    amount: u32,
    name: &'a str,
}

fn transform_line(line: &str) -> (&str, Vec<Bag<'_>>) {
    let io_split: Vec<&str> = line.split(" bags contain ").collect();
    let outer = io_split[0];
    let content_split: Vec<&str> = io_split[1].split(", ").collect();

    let mut contents = Vec::new();
    for content in content_split.iter() {
        if *content == "no other bags." {
            contents.push(Bag {
                amount: 0,
                name: "",
            });
            continue;
        }

        let inner_split: Vec<&str> = content.splitn(2, " ").collect();
        let number = inner_split[0]
            .parse::<u32>()
            .expect("could not parse amount");

        let inner_decomposition: Vec<&str> = inner_split[1].split(" bag").collect();
        contents.push(Bag {
            amount: number,
            name: inner_decomposition[0],
        });
    }
    (outer, contents)
}

#[test]
fn transform_line_test() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let expected = (
        "light red",
        vec![
            Bag {
                amount: 1,
                name: "bright white",
            },
            Bag {
                amount: 2,
                name: "muted yellow",
            },
        ],
    );
    let result = transform_line(input);

    assert_eq!(expected, result);
}

#[test]
fn transform_line_empty_test() {
    let input = "dotted black bags contain no other bags.";
    let expected = (
        "dotted black",
        vec![Bag {
            amount: 0,
            name: "",
        }],
    );
    let result = transform_line(input);

    assert_eq!(expected, result);
}

////////////////////////////////////////////////////////////////////////
fn transform_input(input: &str) -> HashMap<&str, Vec<Bag>> {
    println!("starting to transform input");
    let lines: Vec<&str> = input.trim_end().split('\n').collect();
    let mut transformed = HashMap::new();
    for line in lines.iter() {
        let (k, v) = transform_line(line);
        transformed.insert(k, v);
        println!(
            "transformed {} out of {} lines",
            transformed.len(),
            lines.len()
        );
    }
    transformed
}

#[test]
fn transform_example_input() {
    let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let result = transform_input(input);
    let mut expected = HashMap::new();
    expected.insert(
        "light red",
        vec![
            Bag {
                amount: 1,
                name: "bright white",
            },
            Bag {
                amount: 2,
                name: "muted yellow",
            },
        ],
    );
    expected.insert(
        "dark orange",
        vec![
            Bag {
                amount: 3,
                name: "bright white",
            },
            Bag {
                amount: 4,
                name: "muted yellow",
            },
        ],
    );
    expected.insert(
        "bright white",
        vec![Bag {
            amount: 1,
            name: "shiny gold",
        }],
    );
    expected.insert(
        "muted yellow",
        vec![
            Bag {
                amount: 2,
                name: "shiny gold",
            },
            Bag {
                amount: 9,
                name: "faded blue",
            },
        ],
    );
    expected.insert(
        "shiny gold",
        vec![
            Bag {
                amount: 1,
                name: "dark olive",
            },
            Bag {
                amount: 2,
                name: "vibrant plum",
            },
        ],
    );
    expected.insert(
        "dark olive",
        vec![
            Bag {
                amount: 3,
                name: "faded blue",
            },
            Bag {
                amount: 4,
                name: "dotted black",
            },
        ],
    );
    expected.insert(
        "vibrant plum",
        vec![
            Bag {
                amount: 5,
                name: "faded blue",
            },
            Bag {
                amount: 6,
                name: "dotted black",
            },
        ],
    );
    expected.insert(
        "faded blue",
        vec![Bag {
            amount: 0,
            name: "",
        }],
    );
    expected.insert(
        "dotted black",
        vec![Bag {
            amount: 0,
            name: "",
        }],
    );
    assert_eq!(expected, result);
}

////////////////////////////////////////////////////////////////////////
pub fn solve(input: &str, search: &str) -> usize {
    let map = transform_input(input);

    let mut cache = HashMap::new();
    let mut i = 0;
    while cache.len() < map.len() - 1 && i < map.len() {
        i += 1;
        dbg!(cache.len());
        dbg!(&cache);
        for (k, v) in &map {
            dbg!(k);
            if *k == search {
                continue;
            }
            if dbg!(cache.contains_key(k)) {
                continue;
            }
            if v.iter()
                .filter(|bag| {
                    bag.name == search
                        || match cache.get(&bag.name) {
                            Some(true) => true,
                            Some(false) => false,
                            None => false,
                        }
                })
                .collect::<Vec<_>>()
                .len()
                > 0
            {
                cache.insert(k, true);
            }

            if v.len() == 1 && v[0].amount == 0 {
                cache.insert(k, false);
            }
        }
    }
    dbg!(&cache);
    cache
        .iter()
        .fold(0, |acc, (_, &v)| acc + if v { 1 } else { 0 })
}

#[test]
fn solve_example_input() {
    let input = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let result = solve(input, "shiny gold");
    let expected = 4;

    assert_eq!(expected, result);
}
