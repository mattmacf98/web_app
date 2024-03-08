use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done {super_struct: base}
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
