use serde::{Serialize, Deserialize};
use std::io::{Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub const RECORDS_NUMBER: i32 = 240000;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Restaurant {
    pub table_no: i8,
    pub waiter: String,
    pub check_amount: f32,
    pub number_of_ordered_meals: i8,
}

impl Restaurant {
    pub fn new(table_no: i8, waiter: String, check_amount: f32, number_of_ordered_meals: i8) -> Self {
        Restaurant {
            table_no,
            waiter,
            check_amount,
            number_of_ordered_meals,
        }
    }
    
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn write_json_to_file(data: String) -> u64 {
    let mut file = File::create("data.json").unwrap();
    write!(file, "{}", data).unwrap();

    file.metadata().unwrap().len()
}

pub fn read_json_from_file() -> Vec<Restaurant> {
    let file = File::open("data.json").unwrap();
    let size = file.metadata().unwrap().len();
    let size_in_mb: f64 = (size as f64)/1000000.0;    
    println!("{}{}{}", "Файл: data.json; Размер файла: ", size_in_mb, " Mбайт");

    let buffered = BufReader::new(file);
    let data = buffered.lines().nth(0).unwrap().unwrap();

    let json_data: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    println!("{}{}{}", "Прочитано из файла ", json_data.len(), " записей");
    json_data
}

pub fn get_current_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap();
    since_the_epoch.as_millis()
}

pub fn print_divider() {
    println!("{}", "----------------------------------------");
    println!("{}", "");
}

pub fn save_to_redis<F: FnMut(String)>(description: String, data: &str, ops: i32, mut action: F) {
    println!("{}", description);
    let mut deltas = [0; 10];
    for i in 0..10 {
        let start = get_current_millis();
        action(data.to_owned());
        let end = get_current_millis();
        deltas[i] = end-start;
    }
    let mut delta_sum = 0;
    for i in 0..deltas.len() {
        delta_sum += deltas[i];
    }
    let delta = delta_sum as f64 / deltas.len() as f64;
    let rate = ops as f64 / delta as f64 * 1000_f64;
    println!("{}{:.2}{}", "Сохранение заняло ", delta as f64 / 1000_f64, " секунд");
    println!("{}{:.2}{}", "Средняя скорость записи: ", rate, " операций в секунду");
}

pub fn read_from_redis<F: FnMut()>(description: String, ops: i32, mut action: F) {
    println!("{}", description);
    let mut deltas = [0; 10];
    for i in 0..deltas.len() {
        let start = get_current_millis();
        action();
        let end = get_current_millis();
        deltas[i] = end-start;
    }
    let mut delta_sum = 0;
    for i in 0..deltas.len() {
        delta_sum += deltas[i];
    }
    let delta = delta_sum as f64 / deltas.len() as f64;
    let rate = ops as f64 / delta as f64 * 1000_f64;
    println!("{}{:.2}{}", "Чтение заняло ", delta as f64 / 1000_f64, " секунд");
    println!("{}{:.2}{}", "Средняя скорость чтения: ", rate, " операций в секунду");
}
