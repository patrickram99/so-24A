use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Helper function to send and receive messages
fn send_message(stream: &mut TcpStream, message: &str) -> std::io::Result<()> {
    stream.write_all(message.as_bytes())
}

fn receive_message(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut buffer = [0; 1024];
    let nbytes = stream.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..nbytes]).to_string())
}

// Get the current time in seconds since UNIX_EPOCH
fn get_time_in_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

// Convert Unix time in seconds to a human-readable "HH:MM:SS" format
fn time_to_hms(unix_time: i64) -> String {
    let time = UNIX_EPOCH + Duration::from_secs(unix_time as u64);
    let datetime = time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let total_seconds = datetime.as_secs();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).expect("Could not bind coordinator listener");
    println!("Coordinator running on {}", address);
    let clients = Arc::new(Mutex::new(HashMap::new()));

    // Accept incoming client connections and manage them in a separate thread
    let clients_ref = Arc::clone(&clients);
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.expect("Failed to accept client connection");
            let peer_addr = stream
                .peer_addr()
                .expect("Connected streams should have a peer address");
            println!("New client ({}) connected.", peer_addr);

            {
                let mut clients = clients_ref.lock().unwrap();
                clients.insert(peer_addr, stream);
            }

            // Trigger time synchronization when a new client connects
            synchronize_clients(&clients_ref);
        }
    });

    // Main thread does nothing but keep the server alive
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

    // Collect times from each client
    for client in clients.values_mut() {
        send_message(client, "REQUEST_TIME").expect("Failed to send time request");
        if let Ok(client_time_str) = receive_message(client) {
            if let Ok(client_time) = client_time_str.trim().parse::<i64>() {
                client_times.push(client_time);
            }
        }
    }

    if client_times.is_empty() {
        return;
    }

    // Calculate the average time difference excluding the coordinator's own time
    let sum_of_diff: i64 = client_times.iter().map(|&t| t - coordinator_time).sum();
    let average_diff = sum_of_diff / client_times.len() as i64;
    let new_master_time = coordinator_time + average_diff;

    // Print the detailed average computation and synchronization details
    println!("Computing new average time:");
    println!("  Coordinator time: {}", time_to_hms(coordinator_time));
    for (i, &t) in client_times.iter().enumerate() {
        println!("  Client[{}] time: {}", i + 1, time_to_hms(t));
    }
    println!("  Average time difference: {} seconds", average_diff);
    println!("  New master time: {}", time_to_hms(new_master_time));

    // Send the new master time to each client
    for client in clients.values_mut() {
        let new_time_str = new_master_time.to_string();
        send_message(client, &new_time_str).expect("Failed to send new master time");
    }

    println!(
        "Synchronized all clients to new master time: {}",
        time_to_hms(new_master_time)
    );
}

