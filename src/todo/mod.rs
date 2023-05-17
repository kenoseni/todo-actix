/// Logic that handles the management of todo items in our todo module
pub mod enums;
pub mod structs;
pub mod traits;

// Factory method
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}
// A factory pattern is where we abstract the construction of structs in an entry point of the module
pub fn todo_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
