use std::{collections::{HashMap}, iter};

fn main() {
    let input = include_str!("input");
    let outputs_from: HashMap<&str, Vec<&str>> = input.lines()
    .map(|x|{
        let mut words = x.split_whitespace();
        let me = words.next().unwrap()
        .strip_suffix(":").unwrap();
        let outputs = words.collect::<Vec<_>>();
        (me , outputs)
    }).chain(iter::once(("out", vec![])))
    .collect::<HashMap<_,_>>();

    // println!("p1: {:?}", way_to_get_from_to(&links, "you", "out"));
    let mut memo = HashMap::new();
    println!("p1: {:?}",way_to_get_from_to(&outputs_from, &mut memo, "you", "out"));
    println!("p2: {:?}", 
        way_to_get_from_to(&outputs_from, &mut memo, "svr", "fft")
        * way_to_get_from_to(&outputs_from, &mut memo, "fft", "dac")
        * way_to_get_from_to(&outputs_from, &mut memo,  "dac", "out")
    );
}
fn way_to_get_from_to<'a>(outputs_from: &HashMap<&'a str, Vec<&'a str>>, memo: &mut HashMap<(&'a str, &'a str), u64>, from: &'a str, to: &'a str) -> u64 {
    if outputs_from.get(from).unwrap().contains(&to) {return 1;} 
    if memo.contains_key(&(from, to)) {return *memo.get(&(from, to)).unwrap();}
    let ways: u64 = outputs_from.get(from).unwrap().iter().map(|from|way_to_get_from_to(outputs_from, memo, from, to)).sum();
    memo.insert((from, to).clone(), ways);
    return ways;
}