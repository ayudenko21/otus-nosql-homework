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
    println!("HASH");
    print_divider();
    
    let json_data = read_json_from_file();
    print_divider();

    let mut conn = get_connection();
    
    let data = serde_json::to_string(&json_data).unwrap();

    let records: Vec<Restaurant> = serde_json::from_str(&data).unwrap();

    save_to_redis(
        String::from("Сохраняем отдельные записи как хеши, где ключ - hash_[индекс массива из json], а ключи хеша - названия полей в структуре без использования pipeline"),
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
    print_divider();

    save_to_redis(
        String::from("Сохраняем отдельные записи как хеши, где ключ - hash_[индекс массива из json], а ключи хеша - названия полей в структуре с использованием pipeline"),
        &data,
        RECORDS_NUMBER,
        |_data: String| {
            let mut pipeline = redis::pipe();
            for index in 0..RECORDS_NUMBER {
                pipeline
                    .hset(format!("{}_{}", "hash", index.to_string()), "table_no", &records[index as usize].table_no.to_string())
                    .hset(format!("{}_{}", "hash", index.to_string()), "waiter", &records[index as usize].waiter)
                    .hset(format!("{}_{}", "hash", index.to_string()), "check_amount", &records[index as usize].check_amount)
                    .hset(format!("{}_{}", "hash", index.to_string()), "number_of_ordered_meals", &records[index as usize].number_of_ordered_meals);
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
                let _restaurant = Restaurant {
                    table_no: conn.hget(format!("{}_{}", "hash", index.to_string()), "table_no").unwrap(),
                    waiter: conn.hget(format!("{}_{}", "hash", index.to_string()), "waiter").unwrap(),
                    check_amount: conn.hget(format!("{}_{}", "hash", index.to_string()), "check_amount").unwrap(),
                    number_of_ordered_meals: conn.hget(format!("{}_{}", "hash", index.to_string()), "number_of_ordered_meals").unwrap(),
                };
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
                let key = format!("{}_{}", "hash", index.to_string()); 
                pipeline
                    .hget(&key, "table_no")
                    .hget(&key, "waiter")
                    .hget(&key, "check_amount")
                    .hget(&key, "number_of_ordered_meals");
            }
            pipeline.query::<Value>(&mut conn).unwrap();
        }
    );
    print_divider();
    
}
