use std::ops::Deref;

fn main() {
    let input = include_str!("test");
    let mut paragraphs = input.split("\n\n").collect::<Vec<_>>();
    let mut regions = paragraphs.pop().unwrap()
        .lines()
        .map(|x|{
            let mut words = x.split_ascii_whitespace();
            let mut dimentions = words.next().unwrap()
                .strip_suffix(':').unwrap()
                .split('x')
                .map(|x|x.parse::<u8>().unwrap());
            let width = dimentions.next().unwrap();
            let height = dimentions.next().unwrap();
            let shapes_included: [u8; 6] = words
                .map(|x|x.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
                .try_into().unwrap();
            Region {
                width,
                height,
                shapes_included,
                region: [0u64; 64]
            }
        }).collect::<Vec<_>>();
    let present_shapes: [Shape; 6]  = paragraphs.into_iter()
        .map(|x|{
            let mut rows = x.lines().skip(1)
            .map(|line| line.chars().map(|c|{
                match c {
                    '#' => '1',
                    '.' => '0',
                    '\n' => '\n',
                    _ => unreachable!()
                }
            }).collect::<String>())
            .map(|binary_string|u64::from_str_radix(&binary_string, 2).unwrap());
            Shape {
                rows: [
                    rows.next().unwrap(),
                    rows.next().unwrap(),
                    rows.next().unwrap(),
                ]
            }
        }).collect::<Vec<_>>()
        .try_into()
        .unwrap();

    regions.iter_mut().for_each(|x|{
        x.add(present_shapes[0], 0, 0, false);
    });
    
    println!("{:?}", present_shapes);
    for region in regions {
        let Region{width, height, shapes_included:_, region} = region;
        for row in region.iter().take(height.into()) {
            let s = &format!("{:064b}", row)[(64usize-(width as usize))..64]
                .chars()
                .map(|c| if c == '1' {'#'} else {'.'}).rev().collect::<String>();
            println!("{}", s);
        }
        println!()
    }
}
#[derive(Debug, Clone, Copy)]
struct Shape {
    rows: [u64; 3]
}
#[derive(Debug, Clone, Copy)]
struct Region {
    width: u8,
    height: u8,
    shapes_included: [u8 ; 6],
    region: [u64; 64],
}
impl Region {
    fn add(&mut self, shape: Shape, x_offset: u8, turns: u8, flip: bool) {
        assert!(turns <= 3);
        

    }
}