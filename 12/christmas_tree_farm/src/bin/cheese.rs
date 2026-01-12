fn main() {
    let input = include_str!("input").replace("\n\n", "!");
    // need to change \n\n to ! because split is only double sided for chars
    let mut paragraphs = input.split('!').rev();
    // parsing
    let regions = paragraphs.by_ref().next().unwrap().lines().map(|line|{
        let mut words = line.split(' ');
        let mut wxh = words.next().unwrap().strip_suffix(':').unwrap().split('x');
        let w: usize = wxh.next().unwrap().parse().unwrap();
        let h: usize = wxh.next().unwrap().parse().unwrap();
        let volume = w * h;
        let num_shapes: [usize; 6] = words.map(|x|x.parse().unwrap())
        .collect::<Vec<_>>().try_into().unwrap();
        ( volume, num_shapes)
    });
    let shape_sizes: [usize ; 6]= paragraphs.rev()
    .map(|x|{
        x.chars().filter(|&c|c=='#').count()
    }).collect::<Vec<_>>().try_into().unwrap();

    let mut num_can_fit = 0;
    for (vol, num_shapes) in regions{
        let shapes_total_volumes = num_shapes.iter().zip(shape_sizes).map(|(&a, b)|a*b).sum::<usize>();
        if vol >= shapes_total_volumes {
            num_can_fit += 1;
        }
    };
    println!("{}", num_can_fit)

    // dbg!(&shapes);
    // dbg!(regions);
}
// 537 too high