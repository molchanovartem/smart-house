pub(crate) struct SmartThermometer {
    pub(crate) temperature: f32,
}

impl SmartThermometer {
    pub(crate) fn get_temperature(&self) -> f32 {
        self.temperature
    }
}
