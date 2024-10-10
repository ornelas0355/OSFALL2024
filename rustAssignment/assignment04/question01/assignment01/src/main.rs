fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    *total = 0; // Ensure total is reset to 0 before calculation
    
    let mut current = low;

    while current <= high {
        *total += current;
        current += step;
    }
}

fn main() {
    let mut result = 0;
    
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result); // Should print 5050
    
    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result); // Should print 30

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result); // Should print 35
}
