use std::time::SystemTime;

pub struct Note {
    active: bool,
    pub title: String,
    pub description: String,
    time_created: SystemTime,
    time_deactivated: Option<SystemTime>
}

impl Note {
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn new(title: String, description: String) -> Self {
        Self { 
            active: true, 
            title: title, 
            description: description, 
            time_created: SystemTime::now(), 
            time_deactivated: None 
        }
    }
}
