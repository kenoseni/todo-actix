use serde_json::value::Value;
use serde_json::Map;

use crate::write_to_file;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        println!("{} is being deleted", title);

        state.remove(title);
        write_to_file("./state.json", state);

        println!("\n\n{} is being deleted\n\n", title)
    }
}
