use rand::{thread_rng, Rng};

use json_generator::{Restaurant, write_json_to_file, RECORDS_NUMBER, print_divider};

fn main() {
    let waiters = [
        String::from("John"),
        String::from("Kate"),
        String::from("Andrew"),
        String::from("Paul"),
        String::from("Monika"),
    ];

    let records_number = RECORDS_NUMBER;
    let mut data = Vec::new();
    let mut rng = thread_rng();
    for _i in 0..records_number {
        let record = Restaurant::new(
            rng.gen_range(1..16),
            waiters[rng.gen_range(0..5)].clone(),
            rng.gen_range(100.0..10000.0),
            rng.gen_range(1..10),
        );
        data.push(record);
    }
    let str = serde_json::to_string(&data).unwrap();

    let size = write_json_to_file(str);

    let size_in_mb: f64 = (size as f64)/1000000.0;
    println!("{}{}{}{}{}", "Создан файл data.json c ", records_number, " записей и размером ", size_in_mb, " Mб");
    println!("{}{:?}", "Структура записи: ", data[0]);
    print_divider();
}
