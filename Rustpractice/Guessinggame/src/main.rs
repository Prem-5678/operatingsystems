// Function to check the guess and return 0 if correct, 1 if too high, -1 if too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Hardcoded secret number
    let secret: i32 = 42;

    // Variables to track the number of guesses and the guessing range
    let mut attempts = 0;
    let mut low = 1;
    let mut high = 100;

    // Start the guessing loop
    loop {
        // Guess is set to the midpoint of the current range
        let guess = (low + high) / 2;
        attempts += 1;

        // Call the check_guess function
        let result = check_guess(guess, secret);

        // Use if-else to determine the outcome
        if result == 0 {
            println!("Your guess {} is correct!", guess);
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Your guess {} is too high!", guess);
            high = guess - 1;  // Adjust the upper bound
        } else {
            println!("Your guess {} is too low!", guess);
            low = guess + 1;   // Adjust the lower bound
        }
    }

    // After the loop ends, print how many guesses it took
    println!("It took you {} guesses to find the correct number.", attempts);
}
