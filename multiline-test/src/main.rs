/*A calculator program in Rust
This is a tool that will use several functions and then call them to perform a calculation in sequence.
The program will have a function that will take in two numbers and return the sum.
*/

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

//subtract
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

//multiply
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//divide
fn divide(a: i32, b: i32) -> i32 {
    a / b
}

//main function that uses all calculator functions
fn main() {
    let a = 10;
    let b = 5;

    let sum = sum(a, b);
    let subtract = subtract(a, b);
    let multiply = multiply(a, b);
    let divide = divide(a, b);

    println!("The sum of {} and {} is {}", a, b, sum);
    println!("The difference of {} and {} is {}", a, b, subtract);
    println!("The product of {} and {} is {}", a, b, multiply);
    println!("The quotient of {} and {} is {}", a, b, divide);
}
