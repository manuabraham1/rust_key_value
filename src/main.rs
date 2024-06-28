use std::collections::BTreeMap;

fn main() {
    let mut obj= BTreeMap::new();

    obj.insert(String::from("Name"), String::from("Manu"));
    obj.insert(String::from("Age"), String::from("23"));


    print!("Before Update: \n");
    for (key, value) in &obj{
        println!("{}: {}", key, value);
    }

    update_value(&mut obj, String::from("Age"), String::from("24"));

    println!("\nAfter update:");
    for (key, value) in &obj {
        println!("{}: {}", key, value);
    }

    if let Some(age) = get_value(&obj, "Age") {
        println!("\nValue for 'Age': {}", age);
    } else {
        println!("\nKey 'Age' not found");
    }

    update_value(&mut obj, String::from("Height"), String::from("150"));

    println!("\nAfter inserting new key:");
    for (key, value) in &obj {
        println!("{}: {}", key, value);
    }
}

fn update_value(map: &mut BTreeMap<String, String>, key: String, value: String) {
    map.insert(key, value);
}


fn get_value(map: &BTreeMap<String, String>, key: &str) -> Option<String> {
    map.get(key).cloned()
}