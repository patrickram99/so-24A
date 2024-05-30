use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::net::TcpStream;
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

// Adjust the client's clock and convert the time to a human-readable format
fn adjust_clock(new_master_time: i64, current_time: &mut i64) {
    let offset = new_master_time - *current_time;
    *current_time = new_master_time;

    let time = UNIX_EPOCH + Duration::from_secs(*current_time as u64);
    let datetime = time_to_hms(time);
    println!(
        "Adjusted clock by {} seconds. New simulated time: {}",
        offset, datetime
    );
}

// Convert Unix time in seconds to a human-readable "HH:MM:SS" format
fn time_to_hms(time: SystemTime) -> String {
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
    let coordinator_addr = "127.0.0.1:7878";
    let mut stream =
        TcpStream::connect(coordinator_addr).expect("Could not connect to coordinator");

    // Initialize the client's clock with a random offset between -600 to 600 seconds (±10 minutes)
    let mut rng = thread_rng();
    let random_offset = rng.gen_range(-600..=600);
    let mut simulated_time = get_time_in_seconds() + random_offset;
    let initial_time_str = time_to_hms(UNIX_EPOCH + Duration::from_secs(simulated_time as u64));
    println!("Initial simulated time: {}", initial_time_str);

    loop {
        match receive_message(&mut stream) {
            Ok(message) => {
                if message.trim() == "REQUEST_TIME" {
                    // Send simulated current time to coordinator
                    let current_time_str = simulated_time.to_string();
                    send_message(&mut stream, &current_time_str)
                        .expect("Failed to send current time to coordinator");
                } else {
                    // Attempt to parse the message as the new master time
                    if let Ok(master_time) = message.trim().parse::<i64>() {
                        adjust_clock(master_time, &mut simulated_time);
                    } else {
                        println!("Received unexpected message: {}", message);
                    }
                }
            }
            Err(e) => {
                println!("Failed to receive message: {}", e);
                break;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
