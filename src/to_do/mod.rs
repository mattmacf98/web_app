use crate::to_do::enums::TaskStatus;
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;

pub mod enums;
pub mod structs;

pub enum ItemType {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemType {
    match status {
        TaskStatus::DONE => ItemType::Done(Done::new(title)),
        TaskStatus::PENDING => ItemType::Pending(Pending::new(title))
    }
}
