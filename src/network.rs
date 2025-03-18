
pub struct InternetConnection {
    pub is_connected: bool,
}

impl InternetConnection {
    pub fn new() -> Self {
        InternetConnection { is_connected: true }
    }

    pub fn toggle(&mut self) {
        self.is_connected = !self.is_connected;
        if self.is_connected {
            println!("Internetforbindelse genoprettet!");
        } else {
            println!("Internetforbindelse afbrudt!");
        }
    }
}
