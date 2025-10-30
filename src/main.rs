use std::path::Path;
use std::{fs::File, io};
use sha2::{Sha256, Digest};

fn input_hash(input: &str) {
    // ./hash-generator --input <user-input>
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    println!(">Input {}", input);
    println!(">Hash: {:x}", result)

}

fn file_hash(filepath: &str){
    // ./hash-generator --file <filepath>
    let path = Path::new(filepath);
    if path.exists(){
        let file = File::open(filepath);
        println!("Le fichier existe")
    } else {
        println!("Le fichier n'existe pas")
    };
}

fn compare_hash(file1: &str, file2: &str){
    // ./hash-generator hash1.txt hash2.txt
}


fn main() {
    println!("Hash Generator (SHA-256)");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture");
        let input = input.trim();
        if input.starts_with("./hash-generator --input") {
            let text = &input["./hash-generator --input".len()..];
            input_hash(text);
            continue;
        }

        if input.starts_with("./hash-generator --file"){
            let filename = &input["./hash-generator --file".len()..];
            file_hash(filename);
            continue;
        }
        
        if input.starts_with("./hash-generator --compare"){
            let rest = &input["./hash-generator --compare".len()..];
            let files: Vec<&str> = rest.split_whitespace().collect();
            if files.len() == 2 {
                compare_hash(files[0], files[1]);
                continue;
            }
        }

        if input == "quit" || input == "exit" {
            println!("Bye  bye");
            break;
        }
    }
}
