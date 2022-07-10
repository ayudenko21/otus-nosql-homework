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

    //let client = redis::Client::open("redis://redis-otus:6379").unwrap();
    let client = redis::Client::open("redis://127.0.0.1:6380").unwrap();
    let mut conn = client.get_connection().unwrap();
    let data = serde_json::to_string(&json_data).unwrap();

    save_to_redis(
        String::from("Сохраняем json как строку под ключом 'json'"),
        &data,
        |data: String| {
            let _: () = conn.set("json", &data).unwrap();
        }
    );
    read_from_redis(
        String::from("Читаем json из redis"),
        || -> String {
            conn.get("json").unwrap()
        }
    );
    print_divider();

    let json: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    let mut records: Vec<String> = Vec::new();
    for rec in json {
        let r = serde_json::to_string(&rec).unwrap();
        records.push(r);
    }
    save_to_redis(
        String::from("Сохраняем отдельные записи как строки, где ключ - индекс массива json"),
        &data,
        |_data: String| {
            for index in 0..records.len() {
                let _: () = conn.set(index, &records[index]).unwrap();
            }
        }
    );
    read_from_redis(
        String::from("Читаем отдельные записи как строки, где ключ - индекс массива json"),
        || -> Vec<String> {
            let mut results :Vec<String> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                results.push(conn.get(index.to_string()).unwrap());
            };
            results
        } 
    );
    print_divider();

    let records: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    save_to_redis(
        String::from("Сохраняем отдельные записи как хеши, где ключ - индекс массива json, а ключи хеша - названия полей в структуре"),
        &data,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.hset(index, "table_no", &records[index as usize].table_no).unwrap();
                let _: () = conn.hset(index, "waiter", &records[index as usize].waiter).unwrap();
                let _: () = conn.hset(index, "check_amount", &records[index as usize].check_amount).unwrap();
                let _: () = conn.hset(index, "number_of_ordered_meals", &records[index as usize].number_of_ordered_meals).unwrap();
            }
        }
    );
    read_from_redis(
        String::from("Читаем отдельные записи как хеши, где ключ - индекс массива json"),
        || -> Vec<Restaurant> {
            let mut results :Vec<Restaurant> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                let restaurant = Restaurant {
                    table_no: conn.hget(index.to_string(), "table_no").unwrap(),
                    waiter: conn.hget(index.to_string(), "waiter").unwrap(),
                    check_amount: conn.hget(index.to_string(), "check_amount").unwrap(),
                    number_of_ordered_meals: conn.hget(index.to_string(), "number_of_ordered_meals").unwrap(),
                };
                results.push(restaurant);
            }
            results
        } 
    );
    print_divider();

    let json: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    let mut records: Vec<String> = Vec::new();
    for rec in json {
        let r = serde_json::to_string(&rec).unwrap();
        records.push(r);
    }
    save_to_redis(
        String::from("Сохраняем отдельные записи как строки в list"),
        &data,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.lpush("restaurant_list", &records[index as usize]).unwrap();
            }
        }
    );

    read_from_redis(
        String::from("Читаем отдельные записи из list"),
        || -> Vec<String> {
            let mut results :Vec<String> = Vec::new();
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


    let json: Vec<Restaurant> = serde_json::from_str(&data).unwrap();
    let mut records: Vec<String> = Vec::new();
    for rec in json {
        let r = serde_json::to_string(&rec).unwrap();
        records.push(r);
    }
    save_to_redis(
        String::from("Сохраняем отдельные записи как строки в sorted set"),
        &data,
        |_data: String| {
            for index in 0..RECORDS_NUMBER {
                let _: () = conn.zadd("restaurant_sorted_set", index as isize, &records[index as usize]).unwrap();
            }
        }
    );

    read_from_redis(
        String::from("Читаем отдельные записи из list"),
        || -> Vec<String> {
            let mut results :Vec<String> = Vec::new();
            for index in 0..RECORDS_NUMBER {
                let bulk = conn.zrange("restaurant_sorted_set", index as isize, index as isize).unwrap();
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


