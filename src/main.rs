use std::io;
fn main() {
     loop{
        println!("Type 'c' for Celsius to Fahrenheit");
        println!("Type 'f' to convert Fahrenheit to Celsius");
        println!("Type 'exit' to quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim();
        if choice == "exit"{
             println!("Exiting program");
            break;
        }
        println!("Enter the temperature value:");

        let mut value = String::new();
        io::stdin().read_line(&mut value).expect("Failed to read number");

        let temp: f64 = value.trim().parse().expect("Enter a number");

        if choice == "c"{
        let result = (temp * 9.0/5.0) + 32.0;
        println!("{}°C is {}°F", temp, result);

     }else if choice == "f"{
         let result = (temp - 32.0) * 5.0/9.0;
         println!("{}°F is {}°C", temp, result);
     }else{
         println!("Invalid choice!");
        
        }
        
        }


}