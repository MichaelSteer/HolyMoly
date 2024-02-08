pub struct Application {
    name: String,
    version: String,
    running: bool,
}

#[allow(dead_code)] // TODO: REMOVE
impl Application {
    pub fn new(name: String, version: String) -> Self {
        Application {
            name,
            version,
            running: false,
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            println!("Starting...");
            self.running = true;
        } else {
            println!("Already running...");
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            println!("Stopping...");
            self.running = false;
        } else {
            println!("Oh No");
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn apploop(&self) {
        if self.running {
            println!("Main Loop...");
        } else {
            println!("Not running...");
        }
    }

    pub fn print_version(&self) {
        println!("App: {} Version: {}", self.name, self.version);
    }
}