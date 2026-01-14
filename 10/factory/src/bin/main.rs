use core::f32;
use std::{fmt::Debug, iter::Sum, ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub, SubAssign}, vec};

fn main() {
    let input = include_str!("input");
    let p2 = input
    .replace(['{', '}', '[', ']', '(', ')'], "")
    .lines()
    .enumerate()
    .map(|(i, line)| {
        let mut words = line.split_whitespace();
        // parsing
        let size = words.next().unwrap().len();
        let joltage: Vec<Rational> = words.next_back().unwrap().split(',')
        .map(|x|x.parse::<i32>().unwrap().into())
        .collect();
        let buttons: Vec<Vec<Rational>> = words.map(|x|{
            let mut b = vec![Rational::ZERO;size];
            x.split(',')
                .map(|wire|wire.parse().unwrap())
                .for_each(|wire: usize| b[wire] = 1.into());
            b
        }).collect();
        // get into matrices
        let buttons = Matrix::new(buttons, Orientation::ColumnMajor);
        let joltage = Matrix::new(vec![joltage], Orientation::ColumnMajor);
        let matrix = buttons.join(&joltage).transpose();
        let rref = matrix.clone()
            .row_echelon()
            .reduced_row_echelon();
        let mut rref_cols = rref.clone().transpose();
        let pivot_columns = &rref.1.clone().unwrap();
        let (augmented_column, homogenious_matrix) = (rref_cols.pop().unwrap(), rref_cols);
        let particular = {
            let mut p: Vec<Rational> = vec![0.into(); buttons.len()];
            pivot_columns.iter().zip(augmented_column).for_each(|(i, v)| p[*i] = v);
            Matrix::new(vec![p], Orientation::ColumnMajor)
        };
        let nullspace_basis = homogenious_matrix.clone().transpose().nullspace();
        if nullspace_basis[0].is_empty() {
            // println!("empty nullspace: {:?}", particular[0].iter().sum::<Rational>());
            return particular[0].iter().sum();
        }
        if nullspace_basis.len() != 2 {
            return Rational::ZERO;
        }
        if ![49, 139, 143, 173].contains(&i) { return Rational::ZERO;}
        // need to check
        // 49
        // 139
        // 143
        // 173
        // 

        // gradient decent
        let loss = |free_variable_choices: &Vec<Rational>| -> Rational {
            let free_variable_choices = Matrix::new(
                vec![free_variable_choices.clone()],
                Orientation::ColumnMajor
            );
            // matrix multiplication and addition
            let button_presses = &(&nullspace_basis * &free_variable_choices) + &particular;
            let negative_button_presses = button_presses[0].iter().filter(|&x|x.is_negative()).collect::<Vec<_>>();
            if negative_button_presses.is_empty() {
                button_presses[0].iter().sum::<Rational>() 
            } else {
                negative_button_presses.into_iter().sum::<Rational>().abs() + 5000.into()
            }
        };
        println!("{:?}", i);
        let mut smallest = f32::MAX;
        for i in -00..25 {
            for j in -00..20{
                let loss = loss(&vec![i.into(), j.into()]).to_float();
                if loss < smallest {
                    smallest = loss;
                }
                print!("{:07.2} ", loss)
            }
            println!()
        }

        let mut position = vec![Rational::ZERO; nullspace_basis.len()];
        // let mut button_presses_with_these_choices = Rational::new(i32::MAX, 1);
        let mut last_natural = Rational::new(i32::MAX, 1);
        let mut visited: Vec<Vec<Rational>> = vec![];
        loop {
            visited.push(position.clone());
            let neighbours: Vec<Vec<Rational>> = {
                let num_of_dimentions = nullspace_basis.len();
                (0..num_of_dimentions).flat_map(|dimention|{
                    let mut up = position.clone();
                    up[dimention] -= 1.into();

                    let mut down = position.clone();
                    down[dimention] += 1.into();

                    [up, down]
                }).collect()
            };
            print!("{:?}:{:?}\t", position, loss(&position).to_float());
            neighbours.iter().for_each(|x|print!("{:?}:{:?}, ", x, loss(x).to_float()));
            println!("\t");
            let best_neighbour:Vec<Rational> = neighbours.into_iter()
            .filter(|x|!visited.contains(&x))
            .min_by(|a, b|loss(a).cmp(&loss(b))).unwrap();
            if loss(&position).denominator == 1 {
                last_natural = loss(&position);
            }
            if loss(&position) < loss(&best_neighbour) {
                // println!("nothing smaller then {:?}", loss(&position));
                println!("{:?}", smallest);
                println!("{:?}", loss(&position));
                println!();
                return last_natural
            }
            position = best_neighbour;
        }
    }).sum::<Rational>();
    
    println!("{:?}", p2)
    // 59498 too high
    // 30368 too high
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    RowMajor,
    ColumnMajor,
}
impl Orientation {
    fn flip(self) -> Self{
        match self {
            Orientation::RowMajor => Orientation::ColumnMajor,
            Orientation::ColumnMajor => Orientation::RowMajor,
        }
    }
}
#[derive(Debug, Clone)]
struct Matrix(Vec<Vec<Rational>>, Option<Vec<usize>>, Orientation);
impl Matrix {
    fn print(self: &Self) {
        let to_print = match self.2 {
            Orientation::RowMajor => self.clone(),
            Orientation::ColumnMajor => self.clone().transpose(),
        };
        to_print
        .iter().for_each(|x| {
            x.iter().for_each(|x|{
                print!("{:?} ", x)
            });
            println!();
        });
        println!();
    }
    fn new(x: Vec<Vec<Rational>>, orientation: Orientation) -> Matrix {
        let column_length = x[0].len();
        if !x.iter().map(|x|x.len()).all(|x| x == column_length) {
            panic!("vectors not equal");
        }
        Matrix(x, None, orientation)
    }
    fn transpose(self) -> Matrix{
        let rows = self.len();
        let cols = self[0].len();

        let transpose: Vec<Vec<_>> = (0..cols).map(|col| {
            (0..rows)
                .map(|row| self[row][col])
                .collect()
        }).collect();
        Matrix(transpose, self.1, self.2.flip())
    }
    // from the pseudocode: 
    // https://en.wikipedia.org/wiki/Gaussian_elimination#Pseudocode
    /// performs gausian elimination
    fn row_echelon(self) -> Self {
        assert_eq!(self.2, Orientation::RowMajor);
        let mut mat = self.0;
        let mut pivot_cols = vec![];
        let mut pivot_row = 0usize;
        let mut col = 0usize;
        while pivot_row < mat.len() && col < mat[0].len() {
            let new_pivot_row = (pivot_row..mat.len())
                .find(|&r| mat[r][col] != Rational::ZERO);
            if new_pivot_row.is_none() {
                col += 1;
                continue;
            }
            // swap
            if pivot_row != new_pivot_row.unwrap() {
                // println!("R{pivot_row} <-> R{}", new_pivot_row.unwrap());
            }
            mat.swap(pivot_row, new_pivot_row.unwrap());
            // eliminate below pivot
            for row in pivot_row+1..mat.len() {
                if mat[row][col] == 0.into() {continue;}
                let factor = mat[row][col] / mat[pivot_row][col];
                // println!("R{row} <- R{row}  - ({:?})R{pivot_row}", factor);
                for i in col..mat[0].len(){
                    mat[row][i] = mat[row][i] - (factor * mat[pivot_row][i]);
                }
            }
            // normalize so pivots are all 1
            let pivot_val = mat[pivot_row][col].clone();
            // if pivot_val != 1.into() {println!("R{pivot_row} <- ({:?})R{pivot_row}", Rational::new(1,1) / pivot_val);}
            for i in col..mat[0].len(){
                mat[pivot_row][i] = mat[pivot_row][i] / pivot_val;
            }
            pivot_row += 1;
            pivot_cols.push(col);
            col += 1;
        }
        Matrix(mat, Some(pivot_cols), Orientation::RowMajor)
    }
    fn pivots(&self) -> Option<&Vec<usize>> {
        self.1.as_ref()
    }
    fn reduced_row_echelon(self) -> Self {
        let mut mat= self.0;
        let piv_cols = self.1.unwrap();
        for (pivot_row, &pivot_col) in piv_cols.iter().enumerate().skip(1).rev(){
            for row in 0..pivot_row {
                // println!("R{row} <- R{row} - ({:?})R{pivot_row}", mat[row][pivot_col]);
                let factor = mat[row][pivot_col];
                for i in pivot_col..mat[0].len() {
                    mat[row][i] = mat[row][i] - (factor * mat[pivot_row][i]);
                }
            }
        };
        Matrix(mat, Some(piv_cols), Orientation::RowMajor)
    }
    fn nullspace(self) -> Matrix {
        assert_eq!(self.2, Orientation::RowMajor);
        // if the matrix has a pivot in every column,
        //  then the matrix is one-to-one and has no nullspace (besides trivial)
        if self.pivots().unwrap().iter().len() == self[0].len() {
            return Matrix::new(vec![vec![]], Orientation::ColumnMajor)
        };

        let mat= self.0;
        let piv_cols = self.1.unwrap();
        let mut nspace: Vec<Vec<Rational>> = vec![];
        let free_cols = (0..mat[0].len()).filter(|x|!piv_cols.contains(x)).collect::<Vec<_>>();
        for &fc in &free_cols {
            let mut v = vec![Rational::ZERO; mat[0].len()];
            
            v[fc] = 1.into();

            for (row_idx, &piv_col_idx) in piv_cols.iter().enumerate() {
                if row_idx < mat.len() {
                    v[piv_col_idx] = -mat[row_idx][fc].clone();
                }
            }
            nspace.push(v);
        }
        Matrix::new(nspace, Orientation::ColumnMajor)
    }
    fn join(&self, rhs: &Self) -> Self {
        let Matrix(lhs,_,  lhs_orientation) = self;
        let Matrix(rhs, _, rhs_orientation) = rhs;
        
        assert_eq!(&lhs_orientation, &rhs_orientation);
        assert_eq!(lhs[0].len(), rhs[0].len());
        
        let orientation = lhs_orientation;
        Matrix::new(
            lhs.clone().into_iter().chain(rhs.clone().into_iter()).collect(),
            *orientation
        )
    }
}
impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = match self.2 {
            Orientation::RowMajor => self.clone(),
            Orientation::ColumnMajor => self.clone().transpose()
        };
        let rhs = match rhs.2 {
            Orientation::RowMajor => rhs.clone().transpose(),
            Orientation::ColumnMajor => rhs.clone(),
        };
        assert_eq!(lhs[0].len(), rhs[0].len());
        let mut out = Matrix::new(vec![vec![Rational::ZERO; rhs.len()]; lhs.len()], Orientation::RowMajor);
        for (i, lfs_row) in lhs.iter().enumerate() {
            for (j, rhs_col) in rhs.iter().enumerate() {
                out[i][j] = lfs_row.iter().zip(rhs_col).map(|(a, b)| *a * *b).sum()
            }
        }
        out
    }
}
impl<'a> Add<&'a Matrix> for &'a Matrix {
    type Output = Matrix;
    
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = match self.2 {
            Orientation::RowMajor => self.clone().transpose(),
            Orientation::ColumnMajor => self.clone()
        };
        let rhs = match rhs.2 {
            Orientation::RowMajor => rhs.clone().transpose(),
            Orientation::ColumnMajor => rhs.clone(),
        };
        assert_eq!(lhs[0].len(), rhs[0].len());
        assert_eq!(lhs.len(), rhs.len());
        let mut lhs = lhs.clone();
        for i in 0..lhs.len() {
            for j in 0..lhs[0].len() {
                lhs[i][j] = lhs[i][j] + rhs[i][j];
            }
        };
        lhs
    }
}
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}
impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}
impl Deref for Matrix {
    type Target = Vec<Vec<Rational>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Clone, Copy)]
struct Rational{
    numerator: i32,
    denominator: i32
}
impl Rational {
    const ZERO: Rational= Rational {
        numerator: 0,
        denominator: 1,
    };
    fn new(n: i32, d: i32) -> Self{
        Rational {
            numerator: n,
            denominator: d,
        }
    }
    fn simplify(&self) -> Self{
        let gcd = gcd(self.numerator, self.denominator);
        if gcd == 0 {return Rational::ZERO};
        let sign_fliper = if self.denominator.is_negative() { -1 } else { 1};
        Rational {
            numerator: self.numerator / gcd * sign_fliper,
            denominator: self.denominator / gcd * sign_fliper,
        }
    }
    fn is_negative(&self) -> bool {
        self.numerator.is_negative()
    }
    fn abs(self) -> Self {
        Rational { numerator: self.numerator.abs(), denominator: self.denominator }
    }
    fn to_float(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.numerator as f64 / self.denominator as f64;
        let b = other.numerator  as f64 / other.denominator  as f64;
        a.partial_cmp(&b).unwrap()
    }
}
impl PartialOrd for Rational {    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_float().partial_cmp(&other.to_float())
    }
}
impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Rational { numerator: value, denominator: 1 }
    }
}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    } 
}

impl Eq for Rational{}
impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}  ", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}
impl Mul for Rational{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator
        ).simplify()
    }
}
impl Div for Rational{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Rational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator
        ).simplify()
    }
}
impl Neg for Rational{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}
impl Add for Rational{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let an = self.numerator;
        let ad = self.denominator;
        let bn = rhs.numerator;
        let bd = rhs.denominator;
        Rational {
            numerator: an * bd + bn * ad,
            denominator: ad * bd,
        }.simplify()
    }
}
impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self + -rhs).simplify()
    }
}
impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl Sum for Rational {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, |acc, x| acc + x)
    }
}
impl<'a> Sum<&'a Rational> for Rational {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, |acc, x| acc + *x)
    }
}
// https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn gcd(a: i32, b: i32) -> i32{
    let mut a = a;
    let mut b = b;
    while a != 0 {
        let remainder = b % a;
        b = a;
        a = remainder;
    }
    b.abs()
}
