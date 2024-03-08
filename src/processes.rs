use serde_json::{Map, Value};
use crate::to_do::ItemType;
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;
use crate::to_do::traits::create::Create;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status.stringify(), &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("Command: {} not supported", command)
    }
}

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("Command: {} not supported", command)
    }
}

pub fn process_input(item: ItemType, command: String, state: &Map<String, Value>) {
    match item {
        ItemType::Pending(pending) => process_pending(pending, command, state),
        ItemType::Done(done) => process_done(done, command, state)
    }
}
