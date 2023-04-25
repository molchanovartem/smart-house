mod smart_socket;
mod smart_thermometer;
mod house;
mod room;

fn main() {
    let mut smart_socket = smart_socket::SmartSocket {
        description: String::from("Socket 1"),
        is_on: false,
        power_consumption: 0.0,
    };
    smart_socket.turn_on();
    println!("Розетка включена");
    println!(
        "Потребление мощности: {} Вт",
        smart_socket.get_power_consumption()
    );

    smart_socket.turn_off();
    println!("Розетка выключена");

    println!("Описание: {}", smart_socket.get_description());

    let smart_thermometer = smart_thermometer::SmartThermometer { temperature: 23.5 };
    println!(
        "Текущая температура: {} °C",
        smart_thermometer.get_temperature()
    );
}
