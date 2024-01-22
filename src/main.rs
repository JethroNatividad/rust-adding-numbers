// Program tha prompts how many to add, then prompts n amount of that times and add all numbers.
// Inputs: count, n_number
// Process: add all
// Outputs: The total is n

fn add_numbers(numbers: Vec<i64>) -> i64 {
    // make a total var
    let mut total: i64 = 0;
    // loop numbers
    for i in numbers {
        // add to total
        total += i;
    }
    // return total
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(vec![1, 2, 3]), 6);
        assert_eq!(add_numbers(vec![10, 10, 0, 10]), 30);
        assert_eq!(add_numbers(vec![99999, 0, 1]), 100000);
    }
}

fn main() {
    println!("Hello, world!");
}
