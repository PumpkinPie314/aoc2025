use std::{collections::{HashMap, VecDeque}, iter};

fn main() {
    let input = include_str!("input");
    let outputs = input.lines()
    .map(|x|{
        let mut words = x.split_whitespace();
        let me = words.next().unwrap()
        .strip_suffix(":").unwrap();
        let outputs = words.collect::<Vec<_>>();
        (me , outputs)
    }).chain(iter::once(("out", vec![])))
    .collect::<HashMap<_,_>>();

    let mut ways_to_get_to: HashMap<&str, i32> = outputs.clone().into_iter().map(|(k, _)| (k, 0i32)).collect();
    *ways_to_get_to.get_mut("you").unwrap() = 1;
    let mut queue: VecDeque<&str> = vec!["you"].into();
    while !queue.is_empty() {
        let me = queue.pop_front().unwrap();
        for & output in outputs.get(me).unwrap().iter() {
            queue.push_back(output);
            *ways_to_get_to.get_mut(output).unwrap() += 1;
        }
    }
    println!("{:?}", *ways_to_get_to.get("out").unwrap())
}
