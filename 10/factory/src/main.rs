/*--- Part Two ---

All of the machines are starting to come online! Now, it's time to worry about the joltage requirements.

Each machine needs to be configured to exactly the specified joltage levels to function properly. Below the buttons on each machine is a big lever that you can use to switch the buttons from configuring the indicator lights to increasing the joltage levels. (Ignore the indicator light diagrams.)

The machines each have a set of numeric counters tracking its joltage levels, one counter per joltage requirement. The counters are all initially set to zero.

So, joltage requirements like {3,5,4,7} mean that the machine has four counters which are initially 0 and that the goal is to simultaneously configure the first counter to be 3, the second counter to be 5, the third to be 4, and the fourth to be 7.

The button wiring schematics are still relevant: in this new joltage configuration mode, each button now indicates which counters it affects, where 0 means the first counter, 1 means the second counter, and so on. When you push a button, each listed counter is increased by 1.

So, a button wiring schematic like (1,3) means that each time you push that button, the second and fourth counters would each increase by 1. If the current joltage levels were {0,1,2,3}, pushing the button would change them to be {0,2,2,4}.

You can push each button as many times as you like. However, your finger is getting sore from all the button pushing, and so you will need to determine the fewest total presses required to correctly configure each machine's joltage level counters to match the specified joltage requirements.

Consider again the example from before:

[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

Configuring the first machine's counters requires a minimum of 10 button presses. One way to do this is by pressing (3) once, (1,3) three times, (2,3) three times, (0,2) once, and (0,1) twice.

Configuring the second machine's counters requires a minimum of 12 button presses. One way to do this is by pressing (0,2,3,4) twice, (2,3) five times, and (0,1,2) five times.

Configuring the third machine's counters requires a minimum of 11 button presses. One way to do this is by pressing (0,1,2,3,4) five times, (0,1,2,4,5) five times, and (1,2) once.

So, the fewest button presses required to correctly configure the joltage level counters on all of the machines is 10 + 12 + 11 = 33.

Analyze each machine's joltage requirements and button wiring schematics. What is the fewest button presses required to correctly configure the joltage level counters on all of the machines?
 */
use std::{fmt::Debug, iter::{self, Sum}, ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub}, vec};

fn main() {
    let input = include_str!("input");
    let _p2 = input
        .replace(['{', '}', '[', ']', '(', ')'], "")
        .lines()
        // .skip(113)
        // .take(1)
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
            let nullspace_basis = homogenious_matrix.clone().transpose().nullspace();
            let basis_dimentions = nullspace_basis.len();
            if basis_dimentions == 0 {
                return particular.into_iter().sum::<Rational>();
            }
            // gradient descent 
            let mut free_variables = vec![Rational::ZERO; nullspace_basis.len()];
            let mut button_presses;
            loop {
                button_presses = nullspace_basis.mul(&free_variables);
                for i in 0..button_presses.len() {
                    button_presses[i] = button_presses[i] + particular[i]
                }

                let mut neighbour_free_vars = vec![free_variables.clone(); &nullspace_basis.len()*2];
                for dim in 0..basis_dimentions {
                    neighbour_free_vars[dim * 2 + 0][dim] = neighbour_free_vars[dim * 2 + 0][dim] + 1.into();
                    neighbour_free_vars[dim * 2 + 1][dim] = neighbour_free_vars[dim * 2 + 1][dim] - 1.into();
                }
                let neighbour_button_presses = neighbour_free_vars.clone().into_iter().map(|x|nullspace_basis.mul(&x)).collect::<Vec<_>>();
                if button_presses.iter().any(|x|x.numerator.is_negative()) {
                    // choose the least negative one
                    free_variables = neighbour_button_presses.into_iter().max_by(|a, b| {
                        let a: Rational = a.into_iter()
                            .filter(|x|x.numerator.is_negative())
                            .cloned()
                            .sum();
                        let b: Rational = b.into_iter()
                            .filter(|x|x.numerator.is_negative())
                            .cloned()
                            .sum();
                        a.cmp(&b)
                    }).unwrap();
                    println!("{:?}", free_variables);
                } else {
                    // base case
                    if button_presses.iter().copied().sum::<Rational>() <= neighbour_button_presses.iter().map(|x|x.iter().copied().sum()).min().unwrap() {
                        break;
                    }
                    // choose smallest positive naighbour
                    let best_neighbour_idx = neighbour_button_presses.iter().enumerate().filter(|(i, x)|{
                        (**x).iter().all(|x|(*x).numerator.is_positive())
                    }).max_by(|(_,xa), (_,xb)|{
                        let a: Rational = xa.iter().copied().sum();
                        let b: Rational = xb.iter().copied().sum();
                        a.cmp(&b)
                    }).map(|(i,_)| i).unwrap();
                    free_variables = neighbour_free_vars.swap_remove(best_neighbour_idx)
                }
            }


            
            return button_presses.into_iter().sum()
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
struct Matrix(Vec<Vec<Rational>>, Option<Vec<usize>>, Orientation); // my include pivot columns after refed
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
    fn mul(&self, v: &Vec<Rational>) -> Vec<Rational>  {
        dbg!(&self, &v);
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
        dbg!(&self);
        // if the matrix has a pivot in every column,
        //  then the matrix is one-to-one and has no nullspace (besides trivial)
        if self.pivots().unwrap().iter().len() == self[0].len() {
            println!("early return!");
            return Matrix::new(vec![vec![]])
        };

        let mat= self.0;
        let piv_cols = self.1.unwrap();
        let mut nspace: Vec<Vec<Rational>> = vec![];
        let free_cols = (0..mat[0].len()).filter(|x|!piv_cols.contains(x)).collect::<Vec<_>>();
        for &fc in &free_cols {
            let mut v = vec![Rational::ZERO; mat[0].len()];
            for row in 0..mat.len() {
                v[piv_cols[row]] = -mat[row][fc];
            }
            v[fc] = 1.into();
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
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.numerator  / self.denominator;
        let b = other.numerator / other.denominator;
        a.cmp(&b)
    }
}
impl PartialOrd for Rational {    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.numerator.partial_cmp(&other.numerator) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.denominator.partial_cmp(&other.denominator)
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
