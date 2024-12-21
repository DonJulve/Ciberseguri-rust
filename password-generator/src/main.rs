use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::process;

fn generate_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn print_usage() {
    println!("Usage: generate_password <length>");
    println!("Generate a random password with the specified length.");
}

fn main() {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Si no hay argumentos o el argumento es '--help', se imprime el uso
    if args.len() == 1 || (args.len() == 2 && args[1] == "--help") {
        print_usage();
        process::exit(0);
    }

    // Si se pasa un argumento de longitud, intentar convertirlo
    let length = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: The length must be a positive integer.");
            print_usage();
            process::exit(1);
        }
    };

    let password = generate_password(length);
    println!("Generated password: {}", password);
}
