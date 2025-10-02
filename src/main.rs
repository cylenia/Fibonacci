use std::io;

fn main() {
    println!("Please enter the number you would like in the Fibonacci sequence.");

    let mut position: u32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number.");
                continue;
            }
        };

        if input < 4 {
            println!("Use a larger number. {} isn't very interesting!", input);
            continue;
        };

        break input;
    };

    let nth = position;
    position -= 1;
    let mut nums: [u128; 2] = [0, 1];

    while position != 0 {
        let x = nums[1];
        match nums[0].checked_add(nums[1]) {
            Some(sum) => nums[1] = sum,
            None => {
                println!("Overflow occurred! Try a smaller number.");
                return;
            }
        }
        nums[0] = x;
        position -= 1;
    }

    println!("The {}th fibonacci number is {}", nth, nums[1]);
}
