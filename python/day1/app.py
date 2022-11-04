# Python Basics

# Importing necessary libraries
import math

# Defining a class
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    # Method for the class
    def introduce(self):
        print(f"Hi, I'm {self.name} and I'm {self.age} years old.")

# Defining a function with multiple return values
def divide_and_remainder(a, b):
    return a // b, a % b

# Main function
def main():
    # Printing to console
    print("Hello, world!")

    # Variables and basic types
    x = 5
    y = 10

    integer = 42
    float_num = 3.14
    boolean = True
    char = 'A'
    
    print(integer, float_num, boolean, char)

    # Strings
    str1 = "Hello"
    str2 = "World"
    print(f"{str1} {str2}")

    # Lists (similar to slices in Go)
    numbers = [1, 2, 3, 4, 5]
    print("List:", numbers)

    # Dictionaries (similar to maps in Go)
    ages = {
        "Alice": 30,
        "Bob": 25,
    }
    print("Bob's age:", ages["Bob"])

    # Control flow (if-else)
    if x < 10:
        print("x is less than 10")
    else:
        print("x is 10 or greater")

    # Switch statement equivalent using if-elif-else
    if y == 5:
        print("y is 5")
    elif y == 10:
        print("y is 10")
    else:
        print("y is neither 5 nor 10")

    # For loop
    for i in range(5):
        print(f"Loop iteration: {i}")

    # Looping through a list with index
    for index, value in enumerate(numbers):
        print(f"Index: {index}, Value: {value}")

    # Function call with multiple return values
    quotient, remainder = divide_and_remainder(17, 5)
    print(f"17 divided by 5 is {quotient} with remainder {remainder}")

    # Using the class
    person = Person(name="Alice", age=30)
    person.introduce()

    # Error handling with try-except
    try:
        invalid_date = "2023-13-01"  # Invalid date format
        year, month, day = map(int, invalid_date.split('-'))
        if month < 1 or month > 12:
            raise ValueError("Invalid month")
        print(f"Parsed date: {year}-{month}-{day}")
        
    except ValueError as e:
        print("Error parsing date:", e)

# Entry point of the program
if __name__ == "__main__":
    main()

