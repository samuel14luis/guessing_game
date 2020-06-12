use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Por favor ingresa un número.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al leer la línea");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("Acertaste!");
                break;
            },
        }
    }
}
