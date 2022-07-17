extern crate redis;

use redis::{Commands, Value};

use json_generator::{
    Restaurant,
    RECORDS_NUMBER,
    read_json_from_file,
    print_divider,
    save_to_redis,
    read_from_redis
};

fn main() {
    println!("SORTED SET");
    print_divider();

    let json_data = read_json_from_file();
    print_divider();

    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut conn = client.get_connection().unwrap();
    let data = serde_json::to_string(&json_data).unwrap();

    let json: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    let mut records: Vec<String> = Vec::new();
    for rec in json {
        let r = serde_json::to_string(&rec).unwrap();
        records.push(r);
    }
    save_to_redis(
        String::from("Сохранение отдельных записей как строк в sorted set без использования pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.zadd("restaurant_sorted_set", &records[index as usize], 1).unwrap();
            }
        }
    );
    print_divider();

    save_to_redis(
        String::from("Сохранение отдельных записей как строк в sorted set с использованием pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline.zadd("restaurant_sorted_set", &records[index as usize], 1);
            }
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();

    read_from_redis(
        String::from("Читаем отдельные записи из sorted set без использования pipeline"),
        RECORDS_NUMBER,
        || {
            for index in 0..RECORDS_NUMBER {
                conn.zrange::<&str, Value>("restaurant_sorted_set", index as isize, index as isize).unwrap();
            }
        }
    );
    print_divider();
    
    read_from_redis(
        String::from("Читаем отдельные записи из sorted set с использованием pipeline"),
        RECORDS_NUMBER,
        || {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline.zrange("restaurant_sorted_set", index as isize, index as isize);
            }
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();
    
}
