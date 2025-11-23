use notify_rust::Notification;

/// Sends a desktop notification with the given summary and body.
/// Returns Result<(), notify_rust::error::Error>.
pub fn send_notification(summary: &str, body: &str) -> Result<(), notify_rust::error::Error> {
    Notification::new().summary(summary).body(body).show()?;
    Ok(())
}
