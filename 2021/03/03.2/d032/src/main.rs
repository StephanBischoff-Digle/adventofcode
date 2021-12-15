use std::collections::HashMap;
use std::fs;

fn gamma_eps(lst: &[&str]) -> (i32, i32) {
    let mut acc: HashMap<usize, u32> = HashMap::new();

    // Count bits
    for line in lst {
        line.chars().enumerate().for_each(|(idx, c)| {
            let entry = acc.entry(idx).or_default();
            *entry += match c {
                '1' => 1,
                _ => 0,
            }
        });
    }

    let l = lst
        .len()
        .try_into()
        .expect("Convert number of input line unto u32");
    let mut gamma = 0;
    let mut mask = 0;
    for (k, v) in acc.iter() {
        let d = if 2 * v < l { 0 } else { 1 };
        gamma += d << (acc.len() - k - 1);
        mask += 1 << (acc.len() - k - 1);
    }

    let epsilon = gamma ^ mask;
    (gamma, epsilon)
}

fn filter<'a, F>(mask_gen: F, lst: &[&'a str]) -> &'a str
where
    F: FnOnce(&Vec<&str>) -> i32 + Copy,
{
    let mut f_lst = lst.to_owned();
    let mut l: i32 = (lst[0].len() - 1).try_into().expect("Parse usize unto i32");
    let mut idx = 0;
    while f_lst.len() > 1 {
        let mask = mask_gen(&f_lst);
        let d = if (mask & (1 << l)) == 0 { 0 } else { 1 };
        f_lst = f_lst
            .iter()
            .filter(|s| {
                s.chars()
                    .nth(idx)
                    .expect("Char indexing")
                    .to_string()
                    .parse::<i32>()
                    .expect("Parse 0 or 1.")
                    == d
            })
            .cloned()
            .collect();
        idx += 1;
        l -= 1;
    }

    f_lst[0]
}

fn filter_lst<'a>(lst: &[&'a str]) -> (&'a str, &'a str) {
    (
        filter(|x| gamma_eps(x).0, lst),
        filter(|x| gamma_eps(x).1, lst),
    )
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Read from input.txt");
    let input: Vec<&str> = input.trim_end().split('\n').collect();

    let (g, e) = filter_lst(&input);
    let gamma = u32::from_str_radix(g, 2).expect("Parse binary into u32.");
    let epsilon = u32::from_str_radix(e, 2).expect("Parse binary into u32.");
    println!("{}", gamma * epsilon);
}
