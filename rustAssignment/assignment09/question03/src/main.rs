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
    // Hard-coded secret number
    let secret: i32 = 99;
    
    // Track number of guesses
    let mut attempts: i32 = 0;

    // Simulating user input for the guess
    let mut guess: i32 = 12; // Change this value to test different scenarios
    
    // While loop to check guesses
    while check_guess(guess, secret) != 0 {
        attempts += 1;
        if check_guess(guess, secret) == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
        // Modify the guess for the next iteration (for testing purposes)
        guess += 1; // Incrementing to eventually guess correctly
    }

    println!("Correct guess! It took {} guesses.", attempts + 1);
}
