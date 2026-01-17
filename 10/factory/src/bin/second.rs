use nalgebra::{DMatrix, DVector, SVD, Scalar, stack};
fn main() {
    let input = include_str!("test")
    .replace(['{', '}', '[', ']', '(', ')'], "");
    let input: Vec<_> = input
    .lines()
    .map(|line| {
        // parsing
        let mut words = line.split_ascii_whitespace();
        let _lights = words.next().unwrap();
        let joltage: Vec<f64> = words.next_back().unwrap()
        .split(',')
        .map(|x|x.parse().unwrap())
        .collect();
        let nrows = joltage.len();
        let buttons: Vec<Vec<usize>> = words
        .map(|word| {
            word.split(',')
            .map(|x|x.chars().next().unwrap() as usize - 48)
            .collect()
        }).collect();
        // buttons should be column vectors with 0s and 1s. [1,3] -> [0,1,0,1]
        let buttons: Vec<Vec<f64>> = buttons.into_iter()
        .map(|b|{
            let mut button = vec![0.0; nrows];
            for button_link in b {
                button[button_link] = 1.0
            }
            button
        }).collect();
        // to nalgebra
        let buttons = buttons.into_iter().map(|b|DVector::from_vec(b)).collect::<Vec<_>>();
        let buttons = DMatrix::from_columns(&buttons);
        let joltage = DVector::from_vec(joltage);
        let usv_t = SVD::new(buttons.clone(),true, true);
        usv_t.singular_values
        
        
    }).collect();
    for line in &input {
        print!("{}", line);
    }
}