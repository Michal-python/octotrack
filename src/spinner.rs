use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

const CHARS: [char; 4] = ['/', '-', '\\', '|'];

pub fn spawn_spinner_task(running: Arc<Mutex<AtomicBool>>) {
    tokio::spawn(async move {
        let mut idx = 0;
        let mut stdout = io::stdout();
        while running.lock().await.load(Ordering::SeqCst) {
            write!(
                stdout,
                "\r{} 🌐 Fetching data from GitHub API...",
                CHARS[idx]
            )
            .unwrap();
            stdout.flush().unwrap();
            idx = (idx + 1) % 4;
            sleep(Duration::from_millis(100)).await;
        }
    });
}
