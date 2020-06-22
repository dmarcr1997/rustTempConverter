use std::io;
fn main() {
    let mut choice = 'y';
    loop{
        println!("Hello, Would You Like to convert to Fahrenheit or Celcius(F/C):");
        let mut tempType = String::new();
        io::stdin()
            .read_line(&mut tempType)
            .expect("Failed to read line");
        println!("Enter Your Temperature:");
        let tempType: char = tempType.trim().parse().expect("character");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f32 = temp.trim().parse().expect("Please type a number!");
        println!("Your new Temperature is: {:.2}{}", convert(&tempType, temp), tempType);
        println!("Would You like to convert again(y/n): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: char = choice.trim().parse().expect("character");
        if choice != 'y'{
            break;
        }
    }
    println!("Thank You For using my converter!")
}

fn convert(tempT: &char, temp: f32) -> f32{
    if *tempT == 'C'{
        return (temp - 32.0) * (0.55555555555555555555555555555556);
    }
    else{
        return (temp * (1.8)) + 32.0;
    }
}
