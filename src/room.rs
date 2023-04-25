struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    fn new(name: String) -> Room {
        Room {
            name,
            devices: Vec::new()
        }
    }

    fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    fn get_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}
