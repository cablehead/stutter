use rand::{thread_rng, Rng};
use std::io::Write;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = thread_rng();
    let mut buffer = vec![0; 20]; // Buffer up to 20 bytes, adjust as needed.

    loop {
        let chunk_size = rng.gen_range(5..=20);
        match io::stdin().read(&mut buffer[..chunk_size]) {
            Ok(0) => {
                // EOF reached or no more data to read.
                break;
            }
            Ok(n) => {
                print!("{}", String::from_utf8_lossy(&buffer[..n]));
                io::stdout().flush().unwrap();
                // Sleep for a random duration between 5ms and 100ms.
                thread::sleep(Duration::from_millis(rng.gen_range(20..=100)));
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                break;
            }
        }
    }
}
