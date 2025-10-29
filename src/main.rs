use std::{io, result};
use hex_literal::hex;
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};

fn input_to_hash(input: &str) -> [u8;32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    result.into()
}

fn main(){
    let guess_hash = [22, 89, 158, 100, 105, 172, 4, 38, 148, 109, 151, 245, 192, 127, 175, 192, 11, 211, 137, 36, 104, 35, 226, 95, 99, 202, 16, 107, 61, 184, 114, 201]; //bonjour
    println!("Le hash du mot que vous devez trouver est: {:?}", guess_hash);

    let mut input = String::new();
    println!("Saisissez un mot");
    io::stdin() // accès à l’entrée standard
        .read_line(&mut input) // lit une ligne et la met dans `input`
        .expect("Erreur lors de la lecture");
    
    loop {
        let mut user_hash = input_to_hash(&input);
        if user_hash == guess_hash {
            println!("Les 2 hash correspondent !");
            break
        }
        else {
            println!("\nEssayez un autre mot ");
            println!("Le hash du mot {} est {:?}", input, user_hash);
            io::stdin() // accès à l’entrée standard
                .read_line(&mut input) // lit une ligne et la met dans `input`
                .expect("Erreur lors de la lecture");
        }
    }
    println!("{:?}", guess_hash)
}