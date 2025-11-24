use std::{sync::mpsc, thread, time::Duration};
use tray_icon::{
    Icon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem},
};

mod config;
mod monitor;
mod notifier;

/// Creates a simple water droplet icon for the system tray
fn create_water_icon() -> Icon {
    // Create a 32x32 RGBA image with a blue water droplet
    let width = 32;
    let height = 32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];

    // Draw a simple circle/droplet shape in blue
    let center_x = width / 2;
    let center_y = height / 2;
    let radius = 10;

    for y in 0..height {
        for x in 0..width {
            let dx = x as i32 - center_x as i32;
            let dy = y as i32 - center_y as i32;
            let distance_sq = dx * dx + dy * dy;

            if distance_sq <= radius * radius {
                let idx = ((y * width + x) * 4) as usize;
                rgba[idx] = 64; // R - dark blue
                rgba[idx + 1] = 164; // G
                rgba[idx + 2] = 223; // B - light blue
                rgba[idx + 3] = 255; // A - fully opaque
            }
        }
    }

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}

fn main() {
    println!("h2l0l hydration reminder started as tray icon...");

    // Channel to signal when to quit
    let (tx, rx) = mpsc::channel();

    // Spawn monitoring thread
    thread::spawn(move || {
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

            // Check if we should quit
            match rx.try_recv() {
                Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Stopping monitoring thread...");
                    break;
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }

            thread::sleep(Duration::from_secs(config::POLL_INTERVAL_SECS));
        }
    });

    // Create the menu with more items for better visibility
    let menu = Menu::new();
    let status_item = MenuItem::new("h2l0l - Hydration Reminder", false, None);
    let separator = tray_icon::menu::PredefinedMenuItem::separator();
    let quit_item = MenuItem::new("Quit", true, None);

    menu.append(&status_item).unwrap();
    menu.append(&separator).unwrap();
    menu.append(&quit_item).unwrap();

    // Create the water droplet icon
    let icon = create_water_icon();

    // Create the tray icon for Windows system tray
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("h2l0l - Hydration Reminder")
        .with_icon(icon)
        .with_menu_on_left_click(false) // Right-click only for menu
        .build()
        .expect("Failed to create tray icon");

    println!(
        "Tray icon created in system tray (look for blue droplet at bottom-right). Right-click it and select Quit to exit."
    );

    // Main event loop for menu events and tray icon clicks
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    // Keep the app running and process events
    loop {
        // Process any pending tray icon click events
        while let Ok(_event) = tray_channel.try_recv() {
            // The menu should show automatically on clicks
            // This just helps process events on Windows
            println!("Tray icon clicked - menu should appear");
        }

        // Process any pending menu events
        while let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_item.id() {
                println!("Quit selected, shutting down...");
                let _ = tx.send(());
                // Give monitoring thread time to clean up
                thread::sleep(Duration::from_millis(100));
                println!("Application closed.");
                return;
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}
