use std::{sync::mpsc, thread, time::Duration};
use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

mod config;
mod monitor;
mod notifier;

fn main() {
    println!("h2l0l hydration reminder started as tray icon...");

    // Initialize macOS application
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::NSApplication;
        use cocoa::base::nil;
        let _app = NSApplication::sharedApplication(nil);
    }

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

    // Create the menu
    let menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    menu.append(&quit_item).unwrap();

    // Create the tray icon
    // Note: On macOS, this will appear in the menu bar (top-right)
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("h2l0l - Hydration Reminder")
        .with_title("💧") // Water droplet emoji for visibility in menu bar
        .build()
        .expect("Failed to create tray icon");

    println!(
        "Tray icon created in menu bar (look for 💧 at top-right). Click it and select Quit to exit."
    );

    // Main event loop for menu events
    let menu_channel = MenuEvent::receiver();

    // Keep the app running and process events
    loop {
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

        // Pump macOS event loop to keep the tray icon responsive
        #[cfg(target_os = "macos")]
        unsafe {
            use cocoa::appkit::{NSApplication, NSEventMask};
            use cocoa::base::nil;
            use cocoa::foundation::{NSDate, NSDefaultRunLoopMode};

            let app = NSApplication::sharedApplication(nil);
            let distant_past = NSDate::distantPast(nil);
            let event = app.nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSEventMask::NSAnyEventMask.bits(),
                distant_past,
                NSDefaultRunLoopMode,
                true,
            );
            if event != nil {
                app.sendEvent_(event);
            }
        }

        // Sleep briefly to avoid spinning the CPU
        thread::sleep(Duration::from_millis(100));
    }
}
