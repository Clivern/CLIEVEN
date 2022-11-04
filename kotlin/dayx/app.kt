import kotlin.math.PI

// Data class
data class Person(val name: String, var age: Int)

// Enum class
enum class Color {
    RED, GREEN, BLUE
}

// Interface
interface Shape {
    fun area(): Double
}

// Class implementing an interface
class Circle(private val radius: Double) : Shape {
    override fun area() = PI * radius * radius
}

// Extension function
fun String.addExclamation() = "$this!"

// Higher-order function
fun operation(x: Int, y: Int, op: (Int, Int) -> Int): Int = op(x, y)

fun main() {
    // Printing to console
    println("Hello, world!")

    // Variables and immutability
    val immutable = 5
    var mutable = 10
    mutable += 5

    // Basic types
    val integer: Int = 42
    val float: Float = 3.14f
    val double: Double = 3.14
    val boolean: Boolean = true
    val character: Char = 'A'

    // Strings
    val string = "Kotlin"
    println("Hello, $string!")

    // Null safety
    var nullableString: String? = null
    println(nullableString?.length ?: "String is null")

    // Lists and Arrays
    val list = listOf(1, 2, 3, 4, 5)
    val mutableList = mutableListOf(1, 2, 3)
    val array = arrayOf(1, 2, 3, 4, 5)

    // Control flow
    if (immutable < 10) {
        println("Less than 10")
    } else {
        println("10 or greater")
    }

    // When expression (switch-case equivalent)
    when (mutable) {
        5 -> println("It's 5")
        10 -> println("It's 10")
        else -> println("It's neither 5 nor 10")
    }

    // Loops
    for (i in 0 until 5) {
        println("Loop iteration: $i")
    }

    var counter = 0
    while (counter < 3) {
        println("While loop: $counter")
        counter++
    }

    // Using the data class
    val person = Person("Alice", 30)
    println(person)
    person.age++

    // Using the enum
    val color = Color.RED
    println("Color: $color")

    // Using the interface and class
    val circle = Circle(5.0)
    println("Circle area: ${circle.area()}")

    // Collections operations
    val evenNumbers = list.filter { it % 2 == 0 }
    println("Even numbers: $evenNumbers")

    // Lambda expressions
    val sum = { x: Int, y: Int -> x + y }
    println("Sum: ${sum(5, 7)}")

    // Using the higher-order function
    val result = operation(10, 5) { a, b -> a * b }
    println("Operation result: $result")

    // Extension function usage
    println("Kotlin".addExclamation())

    // Exception handling
    try {
        throw Exception("An error occurred")
    } catch (e: Exception) {
        println("Caught exception: ${e.message}")
    } finally {
        println("This is the finally block")
    }

    // Coroutines (basic example)
    kotlinx.coroutines.runBlocking {
        kotlinx.coroutines.launch {
            delay(1000L)
            println("World!")
        }
        print("Hello, ")
    }

    // Companion object
    class MyClass {
        companion object {
            fun create(): MyClass = MyClass()
        }
    }
    val myObject = MyClass.create()

    // Object declaration (Singleton)
    object Singleton {
        val property = "I'm a singleton"
    }
    println(Singleton.property)
}
