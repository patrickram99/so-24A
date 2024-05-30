use rand::{thread_rng, Rng};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn handle_client(mut stream: TcpStream) {
    let before = Instant::now();

    // Simular un tiempo de RTT aleatorio entre 1 y 3 segundos
    let mut rng = thread_rng();
    let simulated_rtt = rng.gen_range(1..=3);
    sleep(Duration::from_secs(simulated_rtt));

    if let Ok(now) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        let secs_today = (now.as_secs() % 86400) + 300; // Añadir 5 minutos
        let timestamp = secs_today.to_string();
        let after = Instant::now();
        let _ = stream.write_all(timestamp.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Failed: {}", e);
            }
        }
    }
}

