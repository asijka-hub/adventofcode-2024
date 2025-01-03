fn main() {
    let input = include_str!("../input.txt");
    let pairs = input.split("\n")
        .map(|s| s.split_at(s.find(" ").unwrap() + 3))
        .map(|(a, b)| (a.trim_end(),b ))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let (mut x, mut y): (Vec<_>, Vec<_>) = pairs.into_iter().map(|(a, b)| (a, b)).unzip();
    // dbg!(pairs);
    x.sort();
    y.sort();

    let ans = x.iter().zip(y.iter()).fold(0, |acc, (a, b)| acc + (a - b).abs());
    println!("{}", ans);

    // dbg!(x, y);
}
