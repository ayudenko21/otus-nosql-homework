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
    println!("LIST");
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
        String::from("Сохранение отдельных записей сериализованных в JSON без использования pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.lpush("restaurant_list", &records[index as usize]).unwrap();
            }
        }
    );
    print_divider();
    
    save_to_redis(
        String::from("Сохранение отдельных записей сериализованных в JSON с использованием pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline
                    .lpush("restaurant_list", &records[index as usize])
                    .ignore();
            }
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();
    
    read_from_redis(
        String::from("Чтение записей без использования pipeline"),
        RECORDS_NUMBER,
        || {
            for index in 0..RECORDS_NUMBER {
                conn.lrange::<&str, Value>("restaurant_list", index as isize, index as isize).unwrap();
            }
        }
    );
    print_divider();
    
    read_from_redis(
        String::from("Чтение записей с использованием pipeline"),
        RECORDS_NUMBER,
        || {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline.lrange::<&str>("restaurant_list", index as isize, index as isize);
            }
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();
}
