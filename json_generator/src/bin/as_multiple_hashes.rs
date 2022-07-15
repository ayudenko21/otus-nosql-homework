extern crate redis;

use redis::Commands;

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

    let records: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    save_to_redis(
        String::from("Сохраняем отдельные записи как хеши, где ключ - индекс массива json, а ключи хеша - названия полей в структуре"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.hset(format!("{}_{}", "hash", index.to_string()), "table_no", &records[index as usize].table_no.to_string()).unwrap();
                let _: () = conn.hset(format!("{}_{}", "hash", index.to_string()), "waiter", &records[index as usize].waiter).unwrap();
                let _: () = conn.hset(format!("{}_{}", "hash", index.to_string()), "check_amount", &records[index as usize].check_amount).unwrap();
                let _: () = conn.hset(format!("{}_{}", "hash", index.to_string()), "number_of_ordered_meals", &records[index as usize].number_of_ordered_meals).unwrap();
            }
        }
    );
    read_from_redis(
        String::from("Читаем отдельные записи как хеши, где ключ - индекс массива json"),
        RECORDS_NUMBER,
        || -> Vec<Restaurant> {
            let mut results: Vec<Restaurant> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                let restaurant = Restaurant {
                    table_no: conn.hget(format!("{}_{}", "hash", index.to_string()), "table_no").unwrap(),
                    waiter: conn.hget(format!("{}_{}", "hash", index.to_string()), "waiter").unwrap(),
                    check_amount: conn.hget(format!("{}_{}", "hash", index.to_string()), "check_amount").unwrap(),
                    number_of_ordered_meals: conn.hget(format!("{}_{}", "hash", index.to_string()), "number_of_ordered_meals").unwrap(),
                };
                results.push(restaurant);
            }
            results
        }
    );
    print_divider();
}
