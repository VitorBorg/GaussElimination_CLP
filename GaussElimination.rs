// Codigo convertido para Rust, que foi adaptado de um código escrito em C
// Link da implementação original: https://www.codewithc.com/c-program-for-gauss-elimination-method/

use std::io;

fn main() {
    let mut n: usize;
    let mut a: Vec<Vec<f32>> = Vec::new();
    let mut x: Vec<f32> = Vec::new();
    let mut sum: f32;

    println!("\nEnter the order of matrix: ");
    n = read_usize().unwrap();
    println!("\nEnter the elements of augmented matrix row-wise:\n");

    for i in 0..n {
        let mut row: Vec<f32> = Vec::new();
        for j in 0..n + 1 {
            print!("A[{}][{}]: ", i + 1, j + 1);
            let elem = read_f32().unwrap();
            row.push(elem);
        }
        a.push(row);
    }

    for j in 0..n {
        for i in j + 1..n {
            let c = a[i][j] / a[j][j];
            for k in 0..n + 1 {
                a[i][k] = a[i][k] - c * a[j][k];
            }
        }
    }

    x.resize(n, 0.0);
    x[n - 1] = a[n - 1][n] / a[n - 1][n - 1];

    for i in (0..n - 1).rev() {
        sum = 0.0;
        for j in i + 1..n {
            sum += a[i][j] * x[j];
        }
        x[i] = (a[i][n] - sum) / a[i][i];
    }

    println!("\nThe solution is: ");
    for i in 0..n {
        println!("\nx{}={}\t", i + 1, x[i]);
    }
}

fn read_usize() -> Result<usize, std::num::ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>()
}

fn read_f32() -> Result<f32, std::num::ParseFloatError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f32>()
}
