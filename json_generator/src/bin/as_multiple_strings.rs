extern crate redis;

use redis::{Commands, Value};

use json_generator::{
    Restaurant,
    RECORDS_NUMBER,
    read_json_from_file,
    print_divider,
    save_to_redis,
    read_from_redis,
    get_connection
};

fn main() {
    println!("STRINGS");
    print_divider();
    
    let json_data = read_json_from_file();
    print_divider();

    let mut conn = get_connection();

    let data = serde_json::to_string(&json_data).unwrap();

    let json: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    let mut records: Vec<String> = Vec::new();
    for rec in json {
        let r = serde_json::to_string(&rec).unwrap();
        records.push(r);
    }

    save_to_redis(
        String::from("Сохранение отдельных записей как строк, где ключ - индекс массива json без использования pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            for index in 0..records.len() {
                let _: () = conn.set(index, &records[index]).unwrap();
            }
        }
    );
    
    save_to_redis(
        String::from("Сохранение отдельных записей как строк, где ключ - индекс массива json с использованием pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            let mut pipeline = redis::pipe();
            for index in 0..records.len() {
                pipeline.set(index, &records[index]);
            }
            pipeline.query::<Value>(&mut conn).unwrap(); 
        }
    );
    print_divider();

    read_from_redis(
        String::from("Чтение отдельных записей как строк, где ключ - индекс массива json без использования pipeline"),
        RECORDS_NUMBER,
        || {
            let mut results: Vec<String> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                results.push(conn.get(index.to_string()).unwrap());
            };
        }
    );
    print_divider();
    
    read_from_redis(
        String::from("Чтение отдельных записей как строк, где ключ - индекс массива json без использования pipeline"),
        RECORDS_NUMBER,
        || {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline.get(index.to_string());
            };
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();

}
