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

pub fn get_current_micros() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap();
    since_the_epoch.as_micros()
}

pub fn print_divider() {
    println!("{}", "----------------------------------------");
    println!("{}", "");
}

pub fn save_to_redis<F: FnMut(String)>(description: String, data: &str, mut action: F) {
    println!("{}", description);
    let start = get_current_micros();
    action(data.to_owned());
    let end = get_current_micros();
    println!("{}{}{}", "Сохранение заняло ", end-start, " микросекунд");
}

pub fn read_from_redis<F, T>(description: String, mut action: F) -> T
    where F: FnMut() -> T {
    println!("{}", description);
    let start = get_current_micros();
    let result = action();
    let end = get_current_micros();
    println!("{}{}{}", "Чтение заняло ", end-start , " микросекунд");
    result
}