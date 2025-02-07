use std::io;

//Function to calculate coins for a specific denomination
fn calculate_change(mut cents: i32, coin_value: i32) -> i32 {

    let mut coins = 0;
    while cents >= coin_value {
        coins += 1;
        cents -= coin_value;
    }
    coins
}

fn main() {

    //Prompt for a positive amount
    let mut input = String::new();

    loop {

        //Reset the input string
        input.clear();

        println!("Change owed: ");

        //Read input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //Parse input, handling potential errors
        match input.trim().parse::<i32>()
        {
            Ok(cents) if cents >= 0 => {
                
                // Optimized coin change calculation using an array of denominations.
                let denominations = [25, 10, 5, 1];
                let mut remaining = cents; // assuming `cents` is your original amount
                let mut total_coins = 0;

                for &coin in &denominations {
                    let coin_count = calculate_change(remaining, coin);
                    total_coins += coin_count;
                    remaining %= coin;
                }

                println!("The total coins are: {}", total_coins);

                //Exit the loop after successful calculation
                break;
            },

            Ok(_) => println!("Please enter a non-negative amount"),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}
