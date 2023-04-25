struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    fn new(name: String) -> House {
        House { name, rooms: Vec::new() }
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn generate_report<T>(&self, devices_info: &Vec<T>) -> Vec<String>
        where
            T: Fn(&Device, &Room) -> Option<String>,
    {
        let mut report = Vec::new();
        for room in &self.rooms {
            for device in &room.devices {
                let device_name = &device.name;
                let room_name = &room.name;
                let device_state = devices_info.iter()
                    .find_map(|info| info(&device, &room))
                    .unwrap_or_else(|| format!("Device {} not found in room {}", device_name, room_name));
                report.push(format!("{} in {} is {}", device_name, room_name, device_state));
            }
        }
        report
    }
}
