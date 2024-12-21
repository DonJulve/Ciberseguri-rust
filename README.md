# Ciberseguri-rust

Este repositorio contiene ejemplos varios de herramientas de ciberseguridad implementadas en Rust.

## Contenido

- **Keylogger**: Implementación de un keylogger que crea un archivo de log con las teclas presionadas por el usuario y las envía por correo a un servicio de smtp cada minuto.

- **Packet sniffer**: Implementación de un sniffer de paquetes que captura paquetes de la red y los imprime en consola al cabo de un rato, se puede filtrar por protocolo (TCP, UDP, ICMP) y seleccionar la interfaz de red a analizar. También se puede ver la dirección IP de origen.

- **Password generator**: Implementación de un generador de contraseñas que genera contraseñas aleatorias de una longitud dada.

- **Port scanner**: Implementación de un escáner de puertos que escanea los puertos de una dirección IP y muestra los puertos abiertos (por defecto, los puertos del 1 al 1024).

## Tecnologías empleadas

- **Rust**: Lenguaje de programación en el que están implementadas las herramientas.

## Como usar

Compilar el programa con el comando:

```bash
cargo build --release
```

Al lanzar cualquier programa sin parámetros o al poner --help, se mostrará la ayuda del programa (menos el keylogger ya que ese solo tienes que meter tus datos de configuración dentro del programa antes de compilarlo).
