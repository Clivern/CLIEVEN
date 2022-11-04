package main

import (
	"fmt"
	"math"
	"strings"
	"time"
)

// Defining a struct
type Person struct {
	Name string
	Age  int
}

// Method for the struct
func (p Person) Introduce() {
	fmt.Printf("Hi, I'm %s and I'm %d years old.\n", p.Name, p.Age)
}

// Defining an interface
type Shape interface {
	Area() float64
}

// Struct implementing the Shape interface
type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

// Function with multiple return values
func divideAndRemainder(a, b int) (int, int) {
	return a / b, a % b
}

func main() {
	// Printing to console
	fmt.Println("Hello, world!")

	// Variables and short declaration
	var x int = 5
	y := 10

	// Basic types
	var (
		integer int     = 42
		float   float64 = 3.14
		boolean bool    = true
		char    rune    = 'A'
	)
	fmt.Println(
		integer,
		float,
		boolean,
		char,
	)

	// Strings
	str1 := "Hello"
	str2 := "World"
	fmt.Println(strings.Join([]string{str1, str2}, " "))

	// Slices
	numbers := []int{1, 2, 3, 4, 5}
	fmt.Println("Slice:", numbers)

	// Maps
	ages := map[string]int{
		"Alice": 30,
		"Bob":   25,
	}
	fmt.Println("Bob's age:", ages["Bob"])

	// Control flow
	if x < 10 {
		fmt.Println("x is less than 10")
	} else {
		fmt.Println("x is 10 or greater")
	}

	// Switch statement
	switch y {
	case 5:
		fmt.Println("y is 5")
	case 10:
		fmt.Println("y is 10")
	default:
		fmt.Println("y is neither 5 nor 10")
	}

	// For loop
	for i := 0; i < 5; i++ {
		fmt.Printf("Loop iteration: %d\n", i)
	}

	// Range loop
	for index, value := range numbers {
		fmt.Printf("Index: %d, Value: %d\n", index, value)
	}

	// Function call with multiple return values
	quotient, remainder := divideAndRemainder(17, 5)
	fmt.Printf("17 divided by 5 is %d with remainder %d\n", quotient, remainder)

	// Using the struct
	person := Person{Name: "Alice", Age: 30}
	person.Introduce()

	// Using the interface
	circle := Circle{Radius: 5}
	fmt.Printf("Circle area: %.2f\n", circle.Area())

	// Error handling
	_, err := time.Parse("2006-01-02", "2023-13-01")
	if err != nil {
		fmt.Println("Error parsing date:", err)
	}

	// Defer statement
	defer fmt.Println("This will be printed last")

	// Goroutine and channel
	ch := make(chan string)
	go func() {
		ch <- "Hello from goroutine!"
	}()
	fmt.Println(<-ch)

	fmt.Println("Main function completed")
}
