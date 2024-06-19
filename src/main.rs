use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("######################");
    println!("Adivinhe um numero...");
    println!("######################");

    loop {
        let mut guess = String::new();
        println!("Informe um número digitando no console ...");

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha no console!");

        println!("O número que você pensou é : {}",guess);

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número é pequeno demais!"),
            Ordering::Greater => println!("O número é grande demais!"),
            Ordering::Equal => {
                println!("!!!!!!!!!!!!!!");
                println!("Você acertou!");
                println!("!!!!!!!!!!!!!!!");
                break;
            }
        }
    }
}
