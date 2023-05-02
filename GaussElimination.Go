// Codigo convertido para GO, que foi adaptado de um código escrito em C
// Link da implementação original: https://www.codewithc.com/c-program-for-gauss-elimination-method/

package main

import (
	"fmt"
	"strconv"
)

func main() {
	var n int
	var sum float64
	fmt.Print("\nEnter the order of matrix: ")
	fmt.Scanf("%d", &n)

	a := make([][]float64, n)
	for i := range a {
		a[i] = make([]float64, n+1)
	}

	fmt.Print("\nEnter the elements of augmented matrix row-wise:\n")
	for i := 0; i < n; i++ {
		for j := 0; j < n+1; j++ {
			fmt.Printf("A[%d][%d]: ", i+1, j+1)
			a[i][j] = readFloat()
		}
	}

	for j := 0; j < n; j++ {
		for i := j + 1; i < n; i++ {
			c := a[i][j] / a[j][j]
			for k := 0; k < n+1; k++ {
				a[i][k] = a[i][k] - c*a[j][k]
			}
		}
	}

	x := make([]float64, n)
	x[n-1] = a[n-1][n] / a[n-1][n-1]

	for i := n - 2; i >= 0; i-- {
		sum = 0
		for j := i + 1; j < n; j++ {
			sum += a[i][j] * x[j]
		}
		x[i] = (a[i][n] - sum) / a[i][i]
	}

	fmt.Println("\nThe solution is:")
	for i := 0; i < n; i++ {
		fmt.Printf("\nx%d=%f\t", i+1, x[i])
	}
}

func readFloat() float64 {
	var input string
	fmt.Scanf("%s", &input)
	value, err := strconv.ParseFloat(input, 64)
	if err != nil {
		fmt.Println("Error reading float:", err)
		return 0.0
	}
	return value
}
