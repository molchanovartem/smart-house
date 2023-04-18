pub(crate) struct SmartSocket {
    pub(crate) description: String,
    pub(crate) is_on: bool,
    pub(crate) power_consumption: f32,
}

impl SmartSocket {
    pub(crate) fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub(crate) fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub(crate) fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }

    pub(crate) fn get_description(self) -> String {
        self.description
    }
}
