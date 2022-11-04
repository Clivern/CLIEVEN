defmodule Day1 do
  # Defining a struct
  defstruct name: "John Doe", age: 30

  # Function with pattern matching and guards
  def greet(name) when is_binary(name) do
    "Hello, #{name}!"
  end

  def greet(_), do: "Hello, stranger!"

  # Private function
  defp private_helper, do: "I'm private"

  # Main function
  def main do
    # Printing to console
    IO.puts("Hello, world!")

    # Variables and pattern matching
    x = 5
    y = 10
    {a, b} = {1, 2}

    # Basic types
    integer = 42
    float = 3.14
    atom = :an_atom
    boolean = true
    string = "Elixir"

    # Lists and tuples
    list = [1, 2, 3, 4, 5]
    tuple = {:ok, "success"}

    # Maps
    map = %{name: "Alice", age: 30}

    # Keyword lists
    keyword_list = [a: 1, b: 2]

    # Control flow
    if x < 10 do
      IO.puts("x is less than 10")
    else
      IO.puts("x is 10 or greater")
    end

    # Case statement
    case y do
      5 -> IO.puts("y is 5")
      10 -> IO.puts("y is 10")
      _ -> IO.puts("y is neither 5 nor 10")
    end

    # Enum functions
    Enum.each(1..5, fn i -> IO.puts("Iteration #{i}") end)

    # List comprehension
    squares = for n <- 1..5, do: n * n

    # Pipe operator
    1..10
    |> Enum.filter(&(&1 > 5))
    |> Enum.map(&(&1 * 2))
    |> IO.inspect(label: "Filtered and doubled")

    # Using the struct
    person = %Day1{name: "Alice", age: 25}
    IO.inspect(person, label: "Person struct")

    # Pattern matching with maps
    %{name: name} = person
    IO.puts("Name from struct: #{name}")

    # Anonymous functions
    add = fn a, b -> a + b end
    IO.puts("5 + 7 = #{add.(5, 7)}")

    # Error handling
    try do
      raise "An error occurred"
    rescue
      e in RuntimeError -> IO.puts("Caught error: #{e.message}")
    end

    # Processes and message passing
    pid = spawn(fn -> receive do
      {:hello, msg} -> IO.puts("Got hello message: #{msg}")
    end end)
    send(pid, {:hello, "world"})

    # Calling the public function
    IO.puts(greet("Elixir"))

    # Modules and aliasing
    alias String, as: S
    IO.puts(S.upcase("elixir"))
  end
end

# Run the main function
Day1.main()
