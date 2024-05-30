use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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

// Convertir el tiempo Unix en segundos a un formato legible por humanos "HH:MM:SS"
fn time_to_hms(unix_time: i64) -> String {
    let time = UNIX_EPOCH + Duration::from_secs(unix_time as u64);
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
    let address = "127.0.0.1:7878";
    let listener =
        TcpListener::bind(address).expect("No se pudo enlazar el coordinador al listener");
    println!("Coordinador ejecutándose en {}", address);
    let clients = Arc::new(Mutex::new(HashMap::new()));

    // Aceptar conexiones entrantes de clientes y gestionarlas en un hilo separado
    let clients_ref = Arc::clone(&clients);
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.expect("Falló al aceptar la conexión del cliente");
            let peer_addr = stream
                .peer_addr()
                .expect("Las transmisiones conectadas deberían tener una dirección de pares");
            println!("Nuevo cliente ({}) conectado.", peer_addr);

            {
                let mut clients = clients_ref.lock().unwrap();
                clients.insert(peer_addr, stream);
            }

            // Disparar la sincronización de tiempo cuando un nuevo cliente se conecta
            synchronize_clients(&clients_ref);
        }
    });

    // El hilo principal no hace nada más que mantener el servidor vivo
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn synchronize_clients(clients: &Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    let mut clients = clients.lock().unwrap();
    if clients.is_empty() {
        return;
    }

    let coordinator_time = get_time_in_seconds();
    let mut client_times = vec![];

    // Recoger los tiempos de cada cliente
    for client in clients.values_mut() {
        send_message(client, "REQUEST_TIME").expect("Falló al enviar la solicitud de tiempo");
        if let Ok(client_time_str) = receive_message(client) {
            if let Ok(client_time) = client_time_str.trim().parse::<i64>() {
                client_times.push(client_time);
            }
        }
    }

    if client_times.is_empty() {
        return;
    }

    // Calcular la diferencia de tiempo promedio excluyendo el propio tiempo del coordinador
    let sum_of_diff: i64 = client_times.iter().map(|&t| t - coordinator_time).sum();
    let average_diff = sum_of_diff / client_times.len() as i64;
    let new_master_time = coordinator_time + average_diff;

    // Imprimir el cálculo detallado del promedio y los detalles de sincronización
    println!("Calculando nuevo tiempo promedio:");
    println!(
        "  Tiempo del coordinador: {}",
        time_to_hms(coordinator_time)
    );
    for (i, &t) in client_times.iter().enumerate() {
        println!("  Tiempo del cliente[{}]: {}", i + 1, time_to_hms(t));
    }
    println!("  Diferencia de tiempo promedio: {} segundos", average_diff);
    println!("  Nuevo tiempo maestro: {}", time_to_hms(new_master_time));

    // Enviar el nuevo tiempo maestro a cada cliente
    for client in clients.values_mut() {
        let new_time_str = new_master_time.to_string();
        send_message(client, &new_time_str).expect("Falló al enviar el nuevo tiempo maestro");
    }

    println!(
        "Todos los clientes sincronizados al nuevo tiempo maestro: {}",
        time_to_hms(new_master_time)
    );
}

