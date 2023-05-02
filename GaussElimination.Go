# Codigo convertido para Go, que foi adaptado do usuario Maihj do github (implementado em c): https://github.com/Maihj
# Link do repositorio: https://github.com/Maihj/Algorithms/blob/master/Gaussian-elimination/gauss-eli.c

package main

import (
	"fmt"
)

func main() {
	var n, c int
	var a, b, l, x [100]float32

	fmt.Println("\n----------------------------------------------------------------------")
	fmt.Println(" This algorithm is used to solve 'Ax = B' with Gauss elimination.")
	fmt.Println("----------------------------------------------------------------------\n")
	fmt.Print("Please enter the dimension of the matrix A: ")
	fmt.Scanf("%d", &n)

	fmt.Print("Please enter A: ")
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			fmt.Scanf("%f", &a[i*100+j])
		}
	}

	fmt.Print("Please enter B: ")
	for i := 0; i < n; i++ {
		fmt.Scanf("%f", &b[i])
	}

	for k := 0; k < n-1; k++ {
		if a[k*100+k] == 0 {
			fmt.Println("Can't solve this linear equations by Gaussian-elimination.")
			return
		}
		for i := k + 1; i < n; i++ {
			l[i*100+k] = a[i*100+k] / a[k*100+k]
			c++
		}
		for i := k + 1; i < n; i++ {
			for j := k + 1; j < n; j++ {
				a[i*100+j] = a[i*100+j] - l[i*100+k]*a[k*100+j]
				c++
			}
			b[i] = b[i] - l[i*100+k]*b[k]
			c++
			a[i*100+k] = 0
		}
	}

	fmt.Printf("After converting, A(%d*%d) and B(%d*1) can be:\n", n, n, n)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			fmt.Printf("%.2f  ", a[i*100+j])
		}
		fmt.Printf("%.2f\n", b[i])
	}

	fmt.Print("The solution to x is: ")
	x[n-1] = b[n-1] / a[(n-1)*100+n-1]
	c++
	for i := n - 2; i >= 0; i-- {
		var result float32 = 0
		for j := i + 1; j < n; j++ {
			result = result + a[i*100+j]*x[j]
			c++
		}
		x[i] = (b[i] - result) / a[i*100+i]
		c++
	}

	for i := 0; i < n; i++ {
		fmt.Printf("%.2f  ", x[i])
	}
	fmt.Printf("\nThe number of times in multiplication and division is: %d\n", c)
}
