use std::io;
use std::fs;


fn main() {
    

    let file_path = "data.csv";

    // Check if data.csv already exists
    if !fs::metadata(file_path).is_ok() {
        fs::File::create(file_path).unwrap();
        println!("Created file: {}", file_path);
    } else {
        println!("File {} already exists", file_path);
    }


    // loop to interact with user
    loop{
        let multi_line_string = "
Choose an option below
1 - Create a customer
2 - List customer
3 - Add to customer account
4 - Remover from customer account
            ";

        println!("{}",&multi_line_string);

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if num < 1 || num > 4 {
            println!("Invalid input. Please enter a number from 1 to 4.");
        } else {
            println!("You entered: {}", num);
            // break;
        }

        // println!("{}", input.trim());sd
        
    }

}
