use notify_rust::Notification;
use crate::note;

fn run_notification(note: note::Note) -> Result<(), Box<dyn std::error::Error>> {
    notify_rust::Notification::new().summary(note.description.as_str()).show()?;
    Ok(())
}
