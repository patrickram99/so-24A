use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Función auxiliar para enviar y recibir mensajes
fn send_message(stream: &mut TcpStream, message: &str) -> std::io::Result<()> {
    stream.write_all(message.as_bytes())
}

fn receive_message(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut buffer = [0; 1024];
    let nbytes = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..nbytes]).to_string())
}

// Obtener el tiempo actual en segundos desde UNIX_EPOCH
fn get_time_in_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("El tiempo retrocedió")
        .as_secs() as i64
}

// Ajustar el reloj del cliente y convertir el tiempo a un formato legible por humanos
fn adjust_clock(new_master_time: i64, current_time: &mut i64) {
    let offset = new_master_time - *current_time;
    *current_time = new_master_time;

    let time = UNIX_EPOCH + Duration::from_secs(*current_time as u64);
    let datetime = time_to_hms(time);
    println!(
        "Reloj ajustado por {} segundos. Nuevo tiempo simulado: {}",
        offset, datetime
    );
}

// Convertir el tiempo Unix en segundos a un formato legible "HH:MM:SS"
fn time_to_hms(time: SystemTime) -> String {
    let datetime = time
        .duration_since(UNIX_EPOCH)
        .expect("El tiempo retrocedió");
    let total_seconds = datetime.as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    let coordinator_addr = "127.0.0.1:7878";
    let mut stream =
        TcpStream::connect(coordinator_addr).expect("No se pudo conectar con el coordinador");

    // Inicializar el reloj del cliente con un desplazamiento aleatorio entre -600 y 600 segundos (±10 minutos)
    let mut rng = thread_rng();
    let random_offset = rng.gen_range(-600..=600);
    let mut simulated_time = get_time_in_seconds() + random_offset;
    let initial_time_str = time_to_hms(UNIX_EPOCH + Duration::from_secs(simulated_time as u64));
    println!("Tiempo simulado inicial: {}", initial_time_str);

    loop {
        match receive_message(&mut stream) {
            Ok(message) => {
                if message.trim() == "REQUEST_TIME" {
                    // Enviar tiempo actual simulado al coordinador
                    let current_time_str = simulated_time.to_string();
                    send_message(&mut stream, &current_time_str)
                        .expect("Falló al enviar el tiempo actual al coordinador");
                } else {
                    // Intentar analizar el mensaje como el nuevo tiempo maestro
                    if let Ok(master_time) = message.trim().parse::<i64>() {
                        adjust_clock(master_time, &mut simulated_time);
                    } else {
                        println!("Mensaje inesperado recibido: {}", message);
                    }
                }
            }
            Err(e) => {
                println!("Falló al recibir mensaje: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
