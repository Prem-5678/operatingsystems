fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    *total = 0; // Reset total to 0 before starting
    let mut current = low;

    while current <= high {
        *total += current;
        current += step;
    }
}

fn main() {
    let mut result = 0;
    
    // Sum from 0 to 100 with step 1
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    // Sum from 0 to 10 with step 2
    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    // Sum from 5 to 15 with step 3
    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
