# h2l0l - Hydration Reminder

A Windows system tray application that monitors for League of Legends games and reminds you to drink water after each game.

## Features

- 🎮 Automatically detects when League of Legends is running
- 💧 Sends a notification to drink water when a game ends
- 🖥️ Runs as a system tray icon (no visible window)
- 🔕 Smart detection: waits 15 minutes after a game starts before checking again (to avoid interruptions)
- ⚙️ Configurable polling interval
- 🚪 Easy quit option from tray menu

## Platform Support

This application is designed for **Windows only** and monitors `League of Legends.exe`.

The tray icon appears in the system tray (bottom right) - **Right-click** to open the menu.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system

### Building from Source

1. Clone this repository
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

4. Run the application:

```bash
cargo run --release
```

Or run the compiled binary directly from `target/release/h2l0l.exe`

## Usage

1. Start the application
2. A tray icon will appear in your system tray (blue water droplet 💧)
3. The app will run in the background monitoring for League of Legends
4. When you finish a game, you'll receive a notification reminding you to hydrate
5. To quit: Right-click the tray icon and select "Quit"

## Configuration

You can modify the following constants in `src/config.rs`:

- `POLL_INTERVAL_SECS`: How often to check if the game is running (default: 30 seconds)
- `PROCESS_NAME`: The process name to monitor (default: "League of Legends.exe")

## How It Works

1. The app monitors running processes every 30 seconds (configurable)
2. When League of Legends starts, it waits 15 minutes before resuming checks (to avoid spam during game)
3. When the game closes, it immediately sends a notification
4. The monitoring continues indefinitely until you quit the app

## Dependencies

- `sysinfo` - For monitoring system processes
- `notify-rust` - For sending desktop notifications
- `tray-icon` - For creating the system tray icon
- `image` - For icon creation

## Development

To run in development mode with debug output:

```bash
cargo run
```

To build an optimized release version:

```bash
cargo build --release
```

## Troubleshooting

### The tray icon doesn't appear
- Look for a blue water droplet icon
- Check if the icon is hidden in the overflow area (click the up arrow near the system tray)
- Make sure you have permission to display notifications
- Check Settings → System → Notifications & actions

### The menu doesn't appear when I click the icon
- Make sure you're using right-click (not left-click)
- The menu only appears on right-click
- If the icon is unresponsive, try restarting the application

### The notification doesn't show
- Verify that desktop notifications are enabled for the application
- Check that the League of Legends process name matches your system

### The app doesn't detect the game
- The default process name is "League of Legends.exe"
- If your game has a different process name, you may need to modify `PROCESS_NAME` in `src/config.rs`

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## Credits

Created as a friendly reminder to stay hydrated during gaming sessions! 💧