use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Devinez le nombre !");
    loop {
        println!("Veuillez entrer un nombre !");

        let mut assumption = String::new();

        io::stdin()
            .read_line(&mut assumption)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let assumption: u32 = match assumption.trim().parse() {
            Ok(nomber) => nomber,
            Err(_) => continue,
        };

        println!("Votre nombre : {}", assumption);
        match assumption.cmp(&secret_number) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
