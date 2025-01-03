fn main() {
    let input = include_str!("../input.txt");
    let data = input
        .split("\n")
        .map(|a| {
            a.split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    // dbg!(data);

    println!("first: {}", solve_first(&data));
    // println!("first: {}", solve_second(&data));

}

fn solve_first(data: &Vec<Vec<i32>>) -> u32 {
    data.iter().fold(0, |acc, row| {
        let asc = row[0] < row[1];
        let mut prev = row[0];
        let mut diff = 1;

        for e in row[1..].iter() {
            if prev == *e || (*e - prev).abs() > 3 || (prev < *e) != asc {
                diff -= 1;
                break
            }
            prev = *e;
        }
        acc + diff
    })
}