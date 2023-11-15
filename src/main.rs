use rand::Rng;


fn main() {
    let key: u8 = create_random_number();
    println!("A random number has been created between 0 and 100, guess it !");
    println!("So ?");
    while !ask_number(key) { println!("Try again !") }
    println!("See ya soon!");
}

fn ask_number(key: u8) -> bool {
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => {
            println!("Error occured : {}", err);
            return false;
        }
    };

    println!("I wrote {}", input);

    input = input.trim().to_string();

    match input.parse::<u8>() {
        Ok(val) => {
            if val == key {
                println!("Good job chacal !");
                return true;
            }
            if val < key {
                println!("FALSE ! It's greater..");
                return false;
            } else {
                println!("FALSE ! It's smaller..");
                return false;
            }
        }
        Err(_) => {
            println!("Apprend à lire chacal!");
            return false;
        }
    };
}

fn create_random_number() -> u8 {
    let val: u8 = rand::thread_rng().gen_range(0..100);
    return val;
}