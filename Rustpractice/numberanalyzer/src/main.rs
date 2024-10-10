fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers = [12, 15, 7, 3, 10, 25, 30, 22, 18, 5];

    // For loop to iterate through the array and check each number
    for &num in numbers.iter() {
        // Check if the number is even or odd
        if is_even(num) {
            print!("{} is even", num);
        } else {
            print!("{} is odd", num);
        }

        // Check if the number is divisible by 3, 5, or both (FizzBuzz)
        if num % 3 == 0 && num % 5 == 0 {
            println!(" - FizzBuzz");
        } else if num % 3 == 0 {
            println!(" - Fizz");
        } else if num % 5 == 0 {
            println!(" - Buzz");
        } else {
            println!(); // Just move to the next line
        }
    }

    // While loop to find the sum of all numbers in the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("The sum of all numbers is: {}", sum);

    // Loop to find the largest number in the array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in the array is: {}", largest);
}
