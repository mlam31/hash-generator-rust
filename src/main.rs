use std::io::{BufReader, Read};
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

fn file_hash(filepath: &str) -> std::io::Result<String>{
    // ./hash-generator --file <filepath>
    let path = Path::new(filepath);
    if path.exists(){
        println!("> Le fichier {} existe", filepath);
        let file = File::open(filepath)?;
        let mut hasher = Sha256::new();
        let mut reader = BufReader::new(file);

        let mut buffer = [0u8; 4096];
        loop {
            let n = reader.read(&mut buffer)?;
            if n==0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        let result = hasher.finalize();
        let hash_hex = format!("{:x}", result);
        println!("> Hash: {}", hash_hex);
        Ok(hash_hex)
    } else {
        println!(">Le fichier {} n'existe pas", filepath);
        Err(io::Error::new(io::ErrorKind::NotFound, "Fichier introuvable"))
    }
    
}

fn compare_hash(filepath1: &str, filepath2: &str) -> bool{
    // ./hash-generator --compare <filepath1> <filepath2>
    let file1_hash = file_hash(filepath1).unwrap();
    let file2_hash = file_hash(filepath2).unwrap();
    if file1_hash == file2_hash {
        println!("✅Les 2 fichiers possèdent le même hash");
        return true
    } else {
        println!("❌Les 2 fichiers ne possèdent pas le même hash");
        return false
    }
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
            println!("{}", filename);
            let _ = file_hash(filename.trim());
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
