use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let pairs = input
        .split("\n")
        .map(|s| s.split_at(s.find(" ").unwrap() + 3))
        .map(|(a, b)| (a.trim_end(), b))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let (mut x, mut y): (Vec<_>, Vec<_>) = pairs.into_iter().map(|(a, b)| (a, b)).unzip();
    // dbg!(pairs);

    println!("first part: {}", first_part(x.clone(), y.clone()));
    println!("second part: {}", second_part(x.clone(), y.clone()));
}

fn first_part(mut x: Vec<i32>, mut y: Vec<i32>) -> i32 {
    x.sort();
    y.sort();
    x.iter()
        .zip(y.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs())
}

fn second_part(mut x: Vec<i32>, mut y: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for e in &y {
        map.entry(*e).and_modify(|e| *e += 1).or_insert(1);
    }

    // dbg!(&map);

    x.iter()
        .zip(y.iter())
        .map(|(a, b)| (*map.entry(*a).or_default()) * (*a))
        .sum()
}
