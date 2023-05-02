# Codigo convertido para Rust, que foi adaptado do usuario Maihj do github (implementado em c): https://github.com/Maihj
# Link do repositorio: https://github.com/Maihj/Algorithms/blob/master/Gaussian-elimination/gauss-eli.c


use std::io::{self, Write};

fn main() {
    let mut n: usize;
    let mut a = vec![vec![0f32; 100]; 100];
    let mut b = vec![0f32; 100];
    let mut l = vec![vec![0f32; 100]; 100];
    let mut x = vec![0f32; 100];
    let mut c = 0;

    println!("\n----------------------------------------------------------------------\n This algorithm is used to solve 'Ax = B' with Gauss elimination. \n----------------------------------------------------------------------\n");
    print!("Please enter the dimension of the matrix A: ");
    io::stdout().flush().unwrap();
    n = read_usize();

    print!("Please enter A: ");
    io::stdout().flush().unwrap();
    for i in 0..n {
        for j in 0..n {
            a[i][j] = read_f32();
        }
    }

    print!("Please enter B: ");
    io::stdout().flush().unwrap();
    for i in 0..n {
        b[i] = read_f32();
    }

    for k in 0..(n - 1) {
        if a[k][k] == 0f32 {
            println!("Can't solve this linear equations by Gaussian-elimination.");
            return;
        }
        for i in (k + 1)..n {
            l[i][k] = a[i][k] / a[k][k];
            c += 1;
        }
        for i in (k + 1)..n {
            for j in (k + 1)..n {
                a[i][j] = a[i][j] - l[i][k] * a[k][j];
                c += 1;
            }
            b[i] = b[i] - l[i][k] * b[k];
            c += 1;
            a[i][k] = 0f32;
        }
    }

    println!("After converting, A({}*{}) and B({}*1) can be:", n, n, n);
    for i in 0..n {
        for j in 0..n {
            print!("{:.2}  ", a[i][j]);
        }
        println!("{:.2}", b[i]);
    }

    println!("The solution to x is: ");
    x[n - 1] = b[n - 1] / a[n - 1][n - 1];
    c += 1;
    for i in (0..(n - 1)).rev() {
        let mut result = 0f32;
        for j in (i + 1)..n {
            result = result + a[i][j] * x[j];
            c += 1;
        }
        x[i] = (b[i] - result) / a[i][i];
        c += 1;
    }

    for i in 0..n {
        print!("{:.2}  ", x[i]);
    }
    println!("\nThe number of times in multiplication and division is: {}", c);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Failed to parse input")
}

fn read_f32() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Failed to parse input")
}