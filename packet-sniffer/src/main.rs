extern crate pcap;

use pcap::{Capture, Device};
use std::process::{exit};
use std::env;
use std::net::{Ipv4Addr};

fn print_usage() {
    println!("Uso: packet-sniffer INTERFACE [FILTER]");
    println!("\nOpciones:");
    println!("  INTERFACE: Nombre de la interfaz de red a usar.");
    println!("  FILTER: Filtro para capturar paquetes (opcional). Si está vacío, no se aplica filtro.");
    println!("  --help: Muestra este mensaje de ayuda.");
}

fn main() {
    // Obtener los argumentos de la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Mostrar uso si se solicita --help
    if args.len() == 2 && args[1] == "--help" {
        print_usage();
        return;
    }

    // Verificar si se especificó la interfaz
    let device_name = if args.len() > 1 {
        &args[1]
    } else {
        // Si no se proporciona interfaz, mostrar el uso y salir
        println!("Debe especificar una interfaz.");
        print_usage();
        exit(1);
    };

    // Obtener las interfaces de red disponibles
    let devices = Device::list().unwrap();
    if devices.is_empty() {
        println!("No se encontraron dispositivos.");
        exit(1);
    }

    // Imprimir los dispositivos disponibles
    println!("Dispositivos disponibles:");
    for device in &devices {
        println!("{}", device.name);
    }
    println!();

    // Verificar que la interfaz proporcionada sea válida
    let device = devices.iter()
        .find(|dev| dev.name == *device_name)
        .expect("No se encontró el dispositivo");

    println!("Usando la interfaz: {}", device_name);

    // Intentar abrir la interfaz
    let mut capture = match Capture::from_device(device.clone()).unwrap().open() {
        Ok(capture) => capture,
        Err(e) => {
            println!("Error al abrir la interfaz: {}", e);
            exit(1);
        }
    };

    // Obtener el filtro desde los argumentos
    let filter = if args.len() > 2 {
        &args[2]
    } else {
        ""
    };

    // Establecer el filtro si es proporcionado
    if !filter.is_empty() {
        match capture.filter(filter, true) {
            Ok(_) => println!("Filtro establecido: {}", filter),
            Err(e) => {
                println!("Error al establecer el filtro: {}", e);
                exit(1);
            }
        }
    } else {
        println!("No se ha establecido ningún filtro.");
    }

    // Capturar paquetes
    println!("Capturando paquetes...");
    loop {
        match capture.next() {
            Ok(packet) => {
                // Imprimir detalles del paquete capturado
                let size = packet.len();
                println!("Paquete capturado, tamaño: {} bytes", size);

                // Acceder a los primeros bytes del paquete
                let data = packet.data;

                // Verificar si es un paquete Ethernet (Ethernet header)
                if data.len() >= 14 { // Al menos el encabezado de Ethernet tiene 14 bytes
                    let eth_type = ((data[12] as u16) << 8) | (data[13] as u16); // Tipo del paquete (Ethernet)

                    // Verificar si es un paquete IPv4 (Ethernet II, tipo 0x0800)
                    if eth_type == 0x0800 {
                        println!("Paquete IPv4 capturado.");

                        // Obtener la dirección IP de origen (bytes 26 a 29 para IPv4)
                        if data.len() >= 34 { // Comprobamos que los datos del paquete sean suficientes
                            let ip_src = Ipv4Addr::new(data[26], data[27], data[28], data[29]);
                            println!("IP de origen: {}", ip_src);

                            // Obtener el protocolo IP (TCP, UDP, ICMP)
                            let protocol = data[23];
                            match protocol {
                                6 => println!("Protocolo: TCP"),   // TCP
                                17 => println!("Protocolo: UDP"),  // UDP
                                1 => println!("Protocolo: ICMP"),  // ICMP
                                _ => println!("Protocolo desconocido"),
                            }
                        }
                    }
                    // Verificar si es un paquete ARP (Ethernet II, tipo 0x0806)
                    else if eth_type == 0x0806 {
                        println!("Paquete ARP capturado.");
                    }
                    else {
                        println!("Paquete de otro tipo capturado.");
                    }
                }

                // Imprimir un salto de línea entre cada paquete
                println!();
            },
            Err(e) => {
                println!("Error al capturar el paquete: {}", e);
                exit(1);
            }
        }
    }
}
