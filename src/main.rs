use std::io;
use std::io::Write;

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
    use super::add_numbers;
    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(vec![1, 2, 3]), 6);
        assert_eq!(add_numbers(vec![10, 10, 0, 10]), 30);
        assert_eq!(add_numbers(vec![99999, 0, 1]), 100000);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // count, "How many times to add? "
    // initialize numbers vec
    // loop count times
    // insert to numbers vec
    // total = get_numbers(vec)
    // print "The total is {}"
}
