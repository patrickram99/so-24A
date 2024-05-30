use std::io::{self, Read};
use std::net::TcpStream;
use std::time::{Duration, SystemTime};

fn get_server_time() -> Result<f64, io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut buffer = [0; 128];

    let bytes_read = stream.read(&mut buffer)?;
    let server_time_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    server_time_str
        .trim()
        .parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn format_time(seconds: f64) -> String {
    let hours = ((seconds as u64) / 3600) % 24;
    let minutes = ((seconds as u64) % 3600) / 60;
    let secs = seconds % 60.0;
    format!(
        "{:02} horas, {:02} minutos y {:.0} segundos",
        hours, minutes, secs
    )
}

fn main() {
    let start = SystemTime::now();
    match get_server_time() {
        Ok(server_time) => {
            let end = SystemTime::now();
            let rtt = end.duration_since(start).unwrap().as_secs_f64() / 2.0; // RTT calculado como la mitad del tiempo total de ida y vuelta
            let adjusted_time = server_time + rtt; // Ajustar tiempo según el RTT simulado

            println!("Hora del servidor: {}", format_time(server_time));
            println!("Tiempo de ida y vuelta (RTT)/2: {:.3} segundos", rtt);
            println!("Hora ajustada: {}", format_time(adjusted_time));
        }
        Err(e) => {
            eprintln!("Error al obtener la hora del servidor: {}", e);
        }
    }
}

