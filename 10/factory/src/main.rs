
use std::{fmt::Debug, iter::{self, Sum}, ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub}, vec};

fn main() {
    let input = include_str!("test");
    let _p2 = input
        .replace(['{', '}', '[', ']', '(', ')'], "")
        .lines()
        // .skip(113)
        .take(1)
        .map(|line| {
            let mut words = line.split_whitespace();
            let size = words.next().unwrap().len();
            let joltage: Vec<Rational> = words.next_back().unwrap().split(',')
            .map(|x|x.parse::<i16>().unwrap().into())
            .collect();
            let buttons: Vec<Vec<Rational>> = words.map(|x|{
                let mut b = vec![Rational::ZERO;size];
                x.split(',')
                    .map(|wire|wire.parse().unwrap())
                    .for_each(|wire: usize| b[wire] = 1.into());
                b
            }).collect();
            let matrix = Matrix::new(
                buttons.into_iter()
                    .chain(iter::once(joltage)).collect()// augmented
            ).transpose();
            // matrix.print();
            let rref = matrix
                .row_echelon()
                .reduced_row_echelon();
            // rref.print();
            let mut rref_cols = rref.transpose();
            // rref_cols.print();
            let (augmented_column, homogenious_matrix) = (rref_cols.pop().unwrap(), rref_cols);
            let mut particular =  vec![Rational::ZERO; homogenious_matrix.len()];
            for (piv_row, &piv_col) in homogenious_matrix.pivots().unwrap().iter().enumerate(){
                particular[piv_col] = augmented_column[piv_row];
            };
            let basis = homogenious_matrix.clone().transpose().nullspace();
            let basis_dimentions = basis.len();
            if basis_dimentions == 0 {
                return particular.into_iter().sum();
            }
            // gradient descent 
            let mut free_variables = vec![0i16; basis_dimentions];
            loop {
                let neighbours = vec![free_variables.clone(); basis_dimentions*2];

            }


            
            return Rational::ZERO;
        })
        .collect::<Vec<_>>();
    
    // println!("{:?}", p2)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    RowMajor,
    ColumnMajor,
}
impl Orientation {
    fn flip(self) -> Self{
        if self == Orientation::ColumnMajor{
            Orientation::RowMajor
        } else {
            Orientation::ColumnMajor
        }
    }
}
#[derive(Debug, Clone)]
struct Matrix(Vec<Vec<Rational>>, Option<Vec<usize>>, Orientation); // includes pivot columns
impl Matrix {
    fn print(self: &Self) {
        self.clone()
        // .transpose()
        .iter().for_each(|x| {
            x.iter().for_each(|x|{
                print!("{:?} ", x)
            });
            println!();
        });
        println!();
    }
    fn new(x: Vec<Vec<Rational>>) -> Matrix {
        let column_length = x[0].len();
        if !x.iter().map(|x|x.len()).all(|x| x == column_length) {
            panic!("vectors not equal");
        }
        Matrix(x, None, Orientation::ColumnMajor)
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
    fn mul(self, v: Vec<Rational>) -> Vec<Rational>  {
        assert_eq!(self.len(), v.len());
        let mut product = vec![Rational::ZERO; self[0].len()];
        for col in 0..self.len() {
            let scale = v[col];
            for i in 0..self[0].len() {
                product[i] = product[i] + (scale * self[col][i]);
            }
        }
        product
    }
    // i was copying this pseudocode: 
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
        if self.last().unwrap().iter().all(|x|*x== Rational::ZERO) {return Matrix::new(vec![])};
        let mat= self.0;
        let piv_cols = self.1.unwrap();
        let mut nspace: Vec<Vec<Rational>> = vec![];
        let free_cols = (0..mat[0].len()).filter(|x|!piv_cols.contains(x)).collect::<Vec<_>>();
        for &fc in &free_cols {
            let mut v = vec![Rational::ZERO; mat[0].len()];
            for row in 0..mat.len() {
                v[piv_cols[row]] = -mat[row][fc];
            }
            v[fc] = Rational::new(1, 1);
            nspace.push(v);
        }
        Matrix::new(nspace)
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
    numerator: i16,
    denominator: i16
}
impl Rational {
    const ZERO: Rational= Rational {
        numerator: 0,
        denominator: 1,
    };
    fn new(n: i16, d: i16) -> Self{
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
}
impl From<i16> for Rational {
    fn from(value: i16) -> Self {
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
impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self + -rhs).simplify()
    }
}
impl Sum for Rational {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Rational::ZERO, |acc, x| acc + x)
    }
}
// https://en.wikipedia.org/wiki/Euclidean_algorithm#Implementations
fn gcd(a: i16, b: i16) -> i16{
    let mut a = a;
    let mut b = b;
    while a != 0 {
        let remainder = b % a;
        b = a;
        a = remainder;
    }
    b.abs()
}
