fn main() {
    // Step 1: Create an array of 10 integer numbers
    let numbers: [i32; 10] = [15, 3, 6, 10, 8, 30, 22, 18, 5, 25];

    // Step 2: Iterate through the array and check for even/odd, Fizz, Buzz, and FizzBuzz
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // Step 3: Use a while loop to find the sum of all numbers
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Step 4: Find the largest number in the array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}

// Step 5: is_even function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

