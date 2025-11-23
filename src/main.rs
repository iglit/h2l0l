use std::{thread, time::Duration};

mod config;
mod monitor;
mod notifier;

fn main() {
    println!("h2l0l hydration reminder started. Monitoring for League of Legends games...");

    let mut was_running = false;

    loop {
        let is_running = monitor::is_process_running(config::PROCESS_NAME);

        if !was_running && is_running {
            // Game just started
            println!("Game detected! Waiting 15 minutes before resuming polling...");
            was_running = true;
            thread::sleep(Duration::from_secs(15 * 60)); // Sleep for 15 minutes
            continue; // After sleep, re-check process state
        }

        if was_running && !is_running {
            // Game just ended
            if let Err(e) = notifier::send_notification(
                "Hydration Reminder",
                "You finished a game! Drink some water.",
            ) {
                eprintln!("Failed to send notification: {:?}", e);
            }
        }

        was_running = is_running;
        thread::sleep(Duration::from_secs(config::POLL_INTERVAL_SECS));
    }
}
