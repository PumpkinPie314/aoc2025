fn main() {
    let input = include_str!("test");
    let mut pars = input.split("\n\n").collect::<Vec<_>>();
    let tree_areas = pars.pop().unwrap()
        .lines()
        .map(|x|);
    let present_shapes = pars;
    println!("{:?}", present_shapes);
    println!("{:?}", tree_areas);
}

struct Shape([[bool;3];3]);
impl Shape {
    fn from_str(s: &str) {
        s.
    }
}
