use serde_json::value::Value;
use serde_json::Map;

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        println!("{} is being fetched", title);

        let item: Option<&Value> = state.get(title);

        match item {
            Some(result) => {
                println!("\n\nTitle: {}", title);
                println!("Status: {}\n\n", result);
            }
            None => println!("Title: {} was not found", title),
        }
    }
}
