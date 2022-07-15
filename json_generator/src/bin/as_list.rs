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
        String::from("Сохраняем отдельные записи как строки в list"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.lpush("restaurant_list", &records[index as usize]).unwrap();
            }
        }
    );

    read_from_redis(
        String::from("Читаем отдельные записи из list"),
        RECORDS_NUMBER,
        || -> Vec<String> {
            let mut results: Vec<String> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                let bulk = conn.lrange("restaurant_list", index as isize, index as isize).unwrap();
                if let Value::Bulk(val) = bulk {
                    if val.len() > 0 {
                        if let Value::Data(value) = &(val[0]) {
                            results.push(String::from_utf8(value.to_owned()).unwrap());
                        }
                    }
                }
            }
            results
        }
    );
    print_divider();
}
